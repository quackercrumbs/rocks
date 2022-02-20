use tokio::runtime::Runtime;

use bevy::prelude::*;
use configparser::ini::Ini;
use rocks::nasa::{self, models::NearEarthObjectResponse};
use bevy_egui::{egui, EguiContext, EguiPlugin};

// in kilometers
const UNIT_SIZE: f32 = 10000.;

fn initialize_world(
    mut commands: Commands,
    ass: Res<AssetServer>
) {

    const EARTH_DIAMETER_KM: f32 = 6378.137 * 2.;
    const EARTH_DIAMETER: f32 = EARTH_DIAMETER_KM / UNIT_SIZE;

    // note that we have to include the `Scene0` label
    let earth_model = ass.load("models/earth.glb#Scene0");
    let earth_parent = (
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(EARTH_DIAMETER,EARTH_DIAMETER,EARTH_DIAMETER)),
        GlobalTransform::identity(),
    );
    // to be able to position our 3d model:
    // spawn a parent entity with a Transform and GlobalTransform
    // and spawn our gltf as a scene under it
    commands.spawn_bundle(earth_parent).with_children(|parent| {
        parent.spawn_scene(earth_model);
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
        .add_system(read_new_near_earth_object_data_stream)
        .add_startup_system(camera::spawn_camera)
        .add_system(camera::pan_orbit_camera);
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
    near_earth_objects: Query<(&Transform, &NearEarthObject), Without<camera::PanOrbitCamera>>,
    mut camera: Query<(&mut camera::PanOrbitCamera, &mut Transform)>,
) {
    egui::Window::new("Controls").show(egui_context.ctx_mut(), |ui| {

        ui.horizontal(|ui| {
            ui.label("Date: ");
            ui.text_edit_singleline(&mut ui_state.date_range.start_date);
        });

        ui.horizontal(|ui| {
            let query_button = ui.button("Query");
            if query_button.clicked() {
                // todo: validate date is valid
                let date = if ui_state.date_range.start_date.trim().is_empty() {
                    "2020-01-01"
                } else {
                    ui_state.date_range.start_date.trim()
                };
                // fire off event to query for Nasa data (and possibly recreate NEOs)
                info!("params: date_range={:?}", ui_state.date_range);
                // NOTE: for now, only support querying data for 1 day
                if let Err(e) = data_request_sender.0.send(
                    DateRange{ start_date: date.into(), end_date: date.into() }
                ) {
                    error!("Error when trying to send data request {:?}", e)
                }
            }
        });
        for (_camera, transform) in camera.iter() {
            ui.horizontal(|ui| {
                ui.label(format!("Camera Translation: {:?}", transform.translation));
            });
            ui.horizontal(|ui| {
                ui.label(format!("Camera Rotation: {:?}", transform.rotation));
            });
        }
        

        ui.separator();
        if ui.button("Reset Camera").clicked() {
            // update camera back to original position
            for (mut camera, mut transform) in camera.iter_mut() {
                // todo: code duplicated from spawn_camera, we probably want dont want duplicated code ...
                let translation = Vec3::new(-2.0, 2.5, 5.0);
                let new_camera_transform = Transform::from_translation(translation)
                    .looking_at(Vec3::ZERO, Vec3::Y);
                transform.translation = new_camera_transform.translation;
                transform.rotation = new_camera_transform.rotation;
                camera.focus = new_camera_transform.translation;
            }
        }
        for (object_transform,object) in near_earth_objects.iter() {
            ui.horizontal(|ui| {
                if ui.button(format!("{}", object.0)).on_hover_text("Click to copy").clicked() {
                    ui.output().copied_text = format!("{}", object.0);
                }
                if ui.button("focus").clicked() {
                    // update camera focus point
                    // todo: update the zoom to match pan_orbit camera (or update pan_orbit camera zoom)
                    for (mut camera, mut transform) in camera.iter_mut() {
                        let new_camera_transform = Transform{
                            translation: object_transform.translation - 1.,
                            ..Default::default()
                        };
                        let new_camera_transform = new_camera_transform.looking_at(object_transform.translation, Vec3::Y);
                        transform.translation = new_camera_transform.translation;
                        transform.rotation = new_camera_transform.rotation;
                        camera.focus = object_transform.translation;
                    }
                }
            });
        }

    });
}

#[derive(Component)]
struct NearEarthObject(String);

fn read_new_near_earth_object_data_stream(
    mut data_receiver: ResMut<NearEarthObjectDataReciever>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    existing_near_earth_objects: Query<(Entity, &NearEarthObject)>,
) {
    match data_receiver.0.try_recv() {
        Ok(v) => {
            info!("New near earth object data! {:?}", v.element_count);

            // delete the existing objects
            for (e, _object) in existing_near_earth_objects.iter() {
                commands.entity(e).despawn();
            }

            // todo: send event to spawn asteroids?
            for (date, neo_objects) in v.near_earth_objects.iter() {
                info!("date: {} num_objects: {}", date, neo_objects.len());
                for neo_object in neo_objects {
                    // todo: calculate radius and distance
                    // todo: instead of copying the string, convert to f32
                    let close_approach_date = neo_object.close_approach_data
                        .first();
                    if let Some(close_approach_date) = close_approach_date {
                        let miss_distance = close_approach_date.miss_distance.kilometers.parse::<f32>().ok()
                        .map(|distance| distance / UNIT_SIZE);
                        
                        if let Some(miss_distance) = miss_distance {
                            commands.spawn_bundle(PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 0.1, subdivisions: 10})),
                                material: materials.add(Color::rgb(1., 1., 0.0).into()),
                                transform: Transform::from_xyz(miss_distance, 0., 0.),
                                ..Default::default()
                            }).insert(NearEarthObject(neo_object.id.clone()));
                         }
                    }
                }
            }
        },
        Err(tokio::sync::mpsc::error::TryRecvError::Empty) => (),
        Err(e) => {
            error!("Error when trying to read from data_reciever: {:?}", e);
            // todo: close reciever?
        }
    }
}

mod camera {
    use bevy::prelude::*;
    use bevy::input::mouse::{MouseWheel,MouseMotion};
    use bevy::render::camera::PerspectiveProjection;

    #[derive(Component)]
    pub struct PanOrbitCamera {
        /// The "focus point" to orbit around. It is automatically updated when panning the camera
        pub focus: Vec3,
        pub radius: f32,
        pub upside_down: bool,
    }

    impl Default for PanOrbitCamera {
        fn default() -> Self {
            PanOrbitCamera {
                focus: Vec3::ZERO,
                radius: 5.0,
                upside_down: false,
            }
        }
    }

    /// Pan the camera with middle mouse click, zoom with scroll wheel, orbit with right mouse click.
    pub fn pan_orbit_camera(
        windows: Res<Windows>,
        mut ev_motion: EventReader<MouseMotion>,
        mut ev_scroll: EventReader<MouseWheel>,
        input_mouse: Res<Input<MouseButton>>,
        mut query: Query<(&mut PanOrbitCamera, &mut Transform, &PerspectiveProjection)>,
    ) {
        // change input mapping for orbit and panning here
        let orbit_button = MouseButton::Right;
        let pan_button = MouseButton::Middle;

        let mut pan = Vec2::ZERO;
        let mut rotation_move = Vec2::ZERO;
        let mut scroll = 0.0;
        let mut orbit_button_changed = false;

        if input_mouse.pressed(orbit_button) {
            for ev in ev_motion.iter() {
                rotation_move += ev.delta;
            }
        } else if input_mouse.pressed(pan_button) {
            // Pan only if we're not rotating at the moment
            for ev in ev_motion.iter() {
                pan += ev.delta;
            }
        }
        for ev in ev_scroll.iter() {
            scroll += ev.y;
        }
        if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
            orbit_button_changed = true;
        }

        for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
            if orbit_button_changed {
                // only check for upside down when orbiting started or ended this frame
                // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
                let up = transform.rotation * Vec3::Y;
                pan_orbit.upside_down = up.y <= 0.0;
            }

            let mut any = false;
            if rotation_move.length_squared() > 0.0 {
                any = true;
                let window = get_primary_window_size(&windows);
                let delta_x = {
                    let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                    if pan_orbit.upside_down { -delta } else { delta }
                };
                let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
                let yaw = Quat::from_rotation_y(-delta_x);
                let pitch = Quat::from_rotation_x(-delta_y);
                transform.rotation = yaw * transform.rotation; // rotate around global y axis
                transform.rotation = transform.rotation * pitch; // rotate around local x axis
            } else if pan.length_squared() > 0.0 {
                any = true;
                // make panning distance independent of resolution and FOV,
                let window = get_primary_window_size(&windows);
                pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
                // translate by local axes
                let right = transform.rotation * Vec3::X * -pan.x;
                let up = transform.rotation * Vec3::Y * pan.y;
                // make panning proportional to distance away from focus point
                let translation = (right + up) * pan_orbit.radius;
                pan_orbit.focus += translation;
            } else if scroll.abs() > 0.0 {
                any = true;
                pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
                // dont allow zoom to reach zero or you get stuck
                pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
            }

            if any {
                // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
                // parent = x and y rotation
                // child = z-offset
                let rot_matrix = Mat3::from_quat(transform.rotation);
                transform.translation = pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
            }
        }
    }

    fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
        let window = windows.get_primary().unwrap();
        let window = Vec2::new(window.width() as f32, window.height() as f32);
        window
    }

    /// Spawn a camera like this
    pub fn spawn_camera(mut commands: Commands) {
        let translation = Vec3::new(-2.0, 2.5, 5.0);
        let radius = translation.length();

        commands.spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(translation)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        }).insert(PanOrbitCamera {
            radius,
            ..Default::default()
        });
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

                    if let Ok(response) = near_earth_object_client.get_near_earth_objects(&date_range.start_date, &date_range.end_date).await {
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
