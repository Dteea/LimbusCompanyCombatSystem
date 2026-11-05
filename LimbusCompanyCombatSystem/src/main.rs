// Doney Tran
// 11/4/26
// CS423

use bevy::prelude::*;

#[derive(Component)]
enum Direction {
    Left,
    Right,
}

fn setup(mut commands: Commands, asset_server : Res<AssetServer>) {
    commands.spawn(Camera2d);
    
    commands.spawn((
        Sprite::from_image(asset_server.load("Memcho.png")),
        Transform::from_xyz(0., 0., 0.),
        Direction::Right,
    ));
}

// Part of the update system
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Right => transform.translation.x += 150.0 * time.delta_secs(),
            Direction::Left => transform.translation.x -= 150.0 * time.delta_secs(),
        }

        if transform.translation.x > 200.0 {
            *logo = Direction::Left;
        } else if transform.translation.x < -200.0 {
            *logo = Direction::Right;
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
    .add_systems(Startup, setup)
    .add_systems(Update, sprite_movement)
    .run();
     
}