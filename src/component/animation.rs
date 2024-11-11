use bevy::prelude::*;

#[derive(Component)]
pub struct MoveAnimation {
    pub start_pos: Vec2,
    pub end_pos: Vec2,
    pub timer: Timer,
}

#[derive(Component)]
pub struct MergeAnimation {
    pub timer: Timer,
    pub scale: f32,
}

impl Default for MergeAnimation {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Once),
            scale: 1.0,
        }
    }
} 