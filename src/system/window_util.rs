use bevy::{app::AppExit, prelude::{Entity, EventReader, EventWriter, Query, With}, window::{Window, WindowCloseRequested}};

pub fn handle_window_close(
    mut close_events: EventReader<WindowCloseRequested>,
    mut exit: EventWriter<AppExit>,
    windows: Query<Entity, With<Window>>,
) {
    for event in close_events.read() {
        // Check if the closed window is the primary window (usually the first one)
        if event.window == windows.iter().next().unwrap() {
            // If it's the primary window, exit the entire application
            exit.send(AppExit);
        }
    }
}