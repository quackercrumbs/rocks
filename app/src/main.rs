use bevy::prelude::*;
use configparser::ini::Ini;
use rocks::nasa;
use bevy_egui::{egui, EguiContext, EguiPlugin};

fn initialize_world(
    mut commands: Commands,
    ass: Res<AssetServer>,
    client: Res<nasa::client::NearEarthObjectClient>
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
        .insert_resource(near_earth_object_client);
    }
}

#[derive(Default)]
struct UiState {
    start_date: String,
    end_date: String,
}

fn controls_ui(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
) {
    egui::Window::new("Controls").show(egui_context.ctx_mut(), |ui| {

        ui.horizontal(|ui| {
            ui.label("Start date: ");
            ui.text_edit_singleline(&mut ui_state.start_date);
        });
        ui.horizontal(|ui| {
            ui.label("End date: ");
            ui.text_edit_singleline(&mut ui_state.end_date);
        });

        ui.horizontal(|ui| {
            let query_button = ui.button("Query");
            if query_button.clicked() {
                // fire off event to query for Nasa data (and possibly recreate NEOs)
                println!("params: start_date={} end_date={}", &ui_state.start_date, &ui_state.end_date)
            }
        });
    });
}

fn main() {
    App::new()
        .insert_resource(Msaa {samples: 4})
        .add_plugins(DefaultPlugins)
        .add_plugin(RocksPlugin)
        .run();
}
