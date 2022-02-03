use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_homies(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Calvin Quach".to_string()));
}

struct GreetTimer(Timer);
fn greet_homies(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("Welcome to the crib, {}", name.0);
        }
    }
}

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_homies)
        .add_system(greet_homies)
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, true)));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)

        .run();
}
