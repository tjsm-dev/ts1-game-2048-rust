use bevy::render::camera::RenderTarget;
use bevy::window::{EnabledButtons, WindowRef, WindowResolution};
use bevy::prelude::*;

use super::resource::RankWindowId;

pub fn spawn_camera(mut commands: Commands) {
    let _ = commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .id();

    let second_window = commands
        .spawn(Window {
            title: "Rank".to_owned(),
            enabled_buttons: EnabledButtons{ minimize: false, maximize: false, close: false },
            resolution: WindowResolution::new(600.0, 400.0),
            visible: false,
            ..default()
        })
        .id();

    let _ = commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(6.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                target: RenderTarget::Window(WindowRef::Entity(second_window)),
                ..default()
            },
            ..default()
        })
        .id();

    commands.insert_resource(RankWindowId(second_window));
}