use tokio::runtime::Runtime;

use bevy::prelude::*;
use configparser::ini::Ini;
use rocks::nasa::{self, models::NearEarthObjectResponse};
use bevy_egui::{egui, EguiContext, EguiPlugin};

fn initialize_world(
    mut commands: Commands,
    ass: Res<AssetServer>,
) {
    // note that we have to include the `Scene0` label
    let earth_model = ass.load("models/earth.glb#Scene0");
    let earth_parent = (
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::identity(),
    );
    // to be able to position our 3d model:
    // spawn a parent entity with a Transform and GlobalTransform
    // and spawn our gltf as a scene under it
    commands.spawn_bundle(earth_parent).with_children(|parent| {
        parent.spawn_scene(earth_model);
    });

    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0., 0., -5.).looking_at(earth_parent.0.translation, Vec3::Y),
        ..Default::default()
    });
}

pub struct RocksPlugin;
impl Plugin for RocksPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(initialize_world)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 4.0f32
        })
        .add_plugin(EguiPlugin)
        .init_resource::<UiState>()
        .add_system(controls_ui)
        .add_system(read_new_near_earth_object_data_stream);
    }
}

#[derive(Default, Debug)]
struct UiState {
    date_range: DateRange,
}

#[derive(Default, Debug)]
struct DateRange {
    start_date: String,
    end_date: String,
}

fn controls_ui(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    data_request_sender: Res<NearEarthObjectDataRequestSender>,
) {
    egui::Window::new("Controls").show(egui_context.ctx_mut(), |ui| {

        ui.horizontal(|ui| {
            ui.label("Start date: ");
            ui.text_edit_singleline(&mut ui_state.date_range.start_date);
        });
        ui.horizontal(|ui| {
            ui.label("End date: ");
            ui.text_edit_singleline(&mut ui_state.date_range.end_date);
        });

        ui.horizontal(|ui| {
            let query_button = ui.button("Query");
            if query_button.clicked() {
                // fire off event to query for Nasa data (and possibly recreate NEOs)
                info!("params: date_range={:?}", ui_state.date_range);
                if let Err(e) = data_request_sender.0.send(
                    DateRange{ start_date: ui_state.date_range.start_date.clone(), end_date: ui_state.date_range.end_date.clone() }
                ) {
                    error!("Error when trying to send data request {:?}", e)
                }
            }
        });
    });
}

fn read_new_near_earth_object_data_stream(
    mut data_receiver: ResMut<NearEarthObjectDataReciever>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    match data_receiver.0.try_recv() {
        Ok(v) => {
            info!("New near earth object data! {:?}", v.element_count);
            // todo: send event to spawn asteroids?
            for (date, neo_objects) in v.near_earth_objects.iter() {
                info!("date: {} num_objects: {}", date, neo_objects.len());
                commands.spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 0.1, subdivisions: 10})),
                    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                    transform: Transform::from_xyz(1., 0., 0.),
                    ..Default::default()
                });
            }
        },
        Err(tokio::sync::mpsc::error::TryRecvError::Empty) => (),
        Err(e) => {
            error!("Error when trying to read from data_reciever: {:?}", e);
        }
    }
}

struct NearEarthObjectDataReciever(tokio::sync::mpsc::UnboundedReceiver<nasa::models::NearEarthObjectResponse>);
struct NearEarthObjectDataRequestSender(tokio::sync::mpsc::UnboundedSender<DateRange>);

fn main() -> Result<(), Box<dyn std::error::Error>>{

    // initialize API client
    let mut private_config = Ini::new();
    private_config.load("config/private.ini").expect("unable to load config from: config/private.ini");
    let nasa_api_key = private_config.get("topsecrets","NASA_API_KEY").expect("could not find NASA_API_KEY");
    let near_earth_object_client = nasa::client::NearEarthObjectClient::new(&nasa_api_key);

    // setup runtime to handle external calls
    // create channel used to communicate between bevy ECS to tokio
    let (request_data_sender, mut request_data_receiver) = tokio::sync::mpsc::unbounded_channel::<DateRange>();
    let (response_data_sender, response_data_receiver) = tokio::sync::mpsc::unbounded_channel::<NearEarthObjectResponse>();

    // Create the runtime
    let rt  = Runtime::new()?;
    rt.spawn( async move {
        loop {
            // listen for data retrieval requests
            match request_data_receiver.recv().await {
                Some(date_range) =>{
                    println!("Got message in tokio: {:?}", date_range);
                    // default values of empty
                    let start_date = if date_range.start_date.is_empty() {
                        "2020-01-01"
                    } else {
                        &date_range.start_date
                    };
                    let end_date = if date_range.end_date.is_empty() {
                        "2020-01-01"
                    } else {
                        &date_range.end_date
                    };

                    if let Ok(response) = near_earth_object_client.get_near_earth_objects(start_date, end_date).await {
                        if let Err(_) = response_data_sender.send(response) {
                            println!("The reciever dropped for response_data");
                        }
                    } else {
                        println!("Error when trying to call api with date_range={:?}", date_range)
                    }
                },
                None => ()
            }
        }
    });

    App::new()
        .insert_resource(Msaa {samples: 4})
        .add_plugins(DefaultPlugins)
        .add_plugin(RocksPlugin)
        .insert_resource(NearEarthObjectDataReciever(response_data_receiver))
        .insert_resource(NearEarthObjectDataRequestSender(request_data_sender))
        .run();
    
    Ok(())
}
