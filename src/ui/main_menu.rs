use bevy::prelude::EventReader;

use crate::system::events::{MenuType, ShowMenu};


pub fn show_main_menu(mut events: EventReader<ShowMenu>) {
    for event in events.read() {
        match event.0 {
            MenuType::Main => {
                println!("Main Menu Selected!")
            },
            MenuType::Rank => {
                println!("Rank Menu Selected!")
            }
        }
    }
}