use bevy::prelude::*;

fn initialize_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ass: Res<AssetServer>,
) {
    // note that we have to include the `Scene0` label
    let earth_model = ass.load("models/earth.glb#Scene0");

    // to be able to position our 3d model:
    // spawn a parent entity with a Transform and GlobalTransform
    // and spawn our gltf as a scene under it
    commands.spawn_bundle((
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::identity(),
    )).with_children(|parent| {
        parent.spawn_scene(earth_model);
    });

    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
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
        });
    }
}

fn main() {
    App::new()
        .insert_resource(Msaa {samples: 4})
        .add_plugins(DefaultPlugins)
        .add_plugin(RocksPlugin)
        .run();
}
