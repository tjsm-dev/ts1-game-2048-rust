use bevy::prelude::EventReader;

use crate::system::events::StatusType;


pub fn show_main_menu(mut events: EventReader<StatusType>) {
    for event in events.read() {
        match event {
            StatusType::MainMenu => {
                println!("Main Menu Selected!")
            },
            StatusType::Rank => {
                println!("Rank Menu Selected!")
            }
            StatusType::Game => {
                println!("Game Menu Selected!")
            }
        }
    }
}