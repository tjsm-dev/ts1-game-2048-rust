use bevy::prelude::*;
use ts1_game_2048_rust::system::camera;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, camera::spawn_camera)
    .run();
}
