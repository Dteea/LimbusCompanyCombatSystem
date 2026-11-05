// Doney Tran
// 11/4/26
// CS423

use bevy::prelude::*;

#[derive(Component)]
struct Character;
#[derive(Component)]
struct Name(String);
#[derive(Resource)]
struct GreetTimer(Timer);

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, (add_character, hello_world));
        app.add_systems(Update, greet_character);
    }
}

fn hello_world() {
    println!("hello_world");
}

fn add_character(mut commands: Commands) {
    commands.spawn((Character, Name("jeff".to_string())));
    commands.spawn((Character, Name("canda".to_string())));
}

fn greet_character(time: Res<Time>, mut timer: ResMut<GreetTimer>,
   query: Query<&Name, With<Character>>) {

    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}", name.0);
        }
    }
}

fn main() {
    // Notes 
    // App has Update, Schedule
    // World is a startup system, runs only once
    // Can seperate things into plugins
    // Resources are unique global data, Time is one

    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(HelloPlugin)
    .run();
     
}