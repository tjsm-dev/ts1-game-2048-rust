use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    
    let first_window_camera = commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .id();

    commands
    .spawn((NodeBundle::default(), TargetCamera(first_window_camera)))
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Main Menu Window is connected!",
            TextStyle::default(),
        ));
    });
}