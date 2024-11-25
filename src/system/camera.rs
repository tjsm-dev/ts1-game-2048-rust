use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    let _ = commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .id();
}