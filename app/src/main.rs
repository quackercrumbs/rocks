use std::time::{Instant, Duration};
use tokio::runtime::Runtime;

use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use configparser::ini::Ini;
use rocks::nasa;
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
        transform: Transform::from_xyz(-3., -3., -3.).looking_at(earth_parent.0.translation, Vec3::Y),
        ..Default::default()
    });
}

pub struct RocksPlugin;
impl Plugin for RocksPlugin {
    fn build(&self, app: &mut App) {

        // initialize API client
        let mut private_config = Ini::new();
        private_config.load("config/private.ini").expect("unable to load config from: config/private.ini");
        let nasa_api_key = private_config.get("topsecrets","NASA_API_KEY").expect("could not find NASA_API_KEY");
        let near_earth_object_client = nasa::client::NearEarthObjectClient::new(&nasa_api_key);

        app.add_startup_system(initialize_world)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 4.0f32
        })
        .add_plugin(EguiPlugin)
        .init_resource::<UiState>()
        .add_system(controls_ui)
        .add_system(read_new_near_earth_object_data_stream)
        .insert_resource(near_earth_object_client);
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
                info!("params: date_range={}", ui.date_range);
                if let Err(e) = data_request_sender.0.send(
                    DateRange{ start_date: ui.date_range.start_date, end_date: ui.date_range.end_range }
                ) {
                    error!("Error when trying to send data request {:?}", e)
                }
            }
        });
    });
}

fn read_new_near_earth_object_data_stream(
    mut data_receiver: ResMut<NearEarthObjectDataReciever>,
) {
    match data_receiver.0.try_recv() {
        Ok(v) => {
            info!("New near earth object data! {:?}", v);
        },
        Err(Empty) => {
            // do nothing
        },
        Err(e) => {
            error!("Error when trying to read from data_reciever: {:?}", e);
        }
    }
}

struct NearEarthObjectDataReciever(tokio::sync::mpsc::UnboundedReceiver<nasa::models::NearEarthObjectResponse>);
struct NearEarthObjectDataRequestSender(tokio::sync::mpsc::UnboundedSender<DateRange>);

fn main() -> Result<(), Box<dyn std::error::Error>>{

    // setup runtime to handle external calls
    // create channel used to communicate between bevy ECS to tokio
    let (request_data_sender, mut request_data_receiver) = tokio::sync::mpsc::unbounded_channel::<UiState>();
    let (response_data_sender, rx_response_data_receiver) = tokio::sync::mpsc::unbounded_channel::<UiState>();

    // Create the runtime
    let rt  = Runtime::new()?;
    rt.spawn( async move {
        loop {
            // listen for data retrieval requests
            match request_data_receiver.recv().await {
                Some(v) =>{
                    println!("Got message in tokio: {:?}", v);
                    if let Err(_) = response_data_sender.send(NearEarthObjectResponse::default()) {
                        println!("The reciever dropped for response_data");
                    }
                },
                None => println!("sender dropped")
            }
        }
    });

    App::new()
        .insert_resource(Msaa {samples: 4})
        .add_plugins(DefaultPlugins)
        .add_plugin(RocksPlugin)
        .insert_resource(NearEarthObjectDataReciever(rx_response_data))
        .insert_resource(NearEarthObjectDataRequestSender(tx_request_data))
        .run();
    
    Ok(())
}
