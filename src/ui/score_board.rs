use bevy::{
    prelude::{Commands, EventReader},
};
<<<<<<< Updated upstream

use crate::system::events::{TextPopup, TextPopupEvent, ScoreBoardType, ShowScoreBoard};


pub fn show_score_board(
    mut commands: Commands,
    mut events: EventReader<ShowScoreBoard>,
) {
    for event in events.read() {
        match event.0 {
            ScoreBoardType::User => {
                println!("Rank Score Board Selected!");
            },
            ScoreBoardType::Score => {
                generate_text_popup_from_event(
                    &mut commands,
                    &TextPopupEvent {
                        content: "User Score Board Selected!".to_string(),
                    },
                );
            },
            ScoreBoardType::Rank => {
                println!("Rank Score Board Selected!");
            }
        }
    }
}

pub fn generate_text_popup_from_event(
    commands: &mut Commands,
    text_popup_event: &TextPopupEvent,
) {
    spawn_text_popup(
        commands,
        text_popup_event,
    );
}

#[allow(clippy::too_many_arguments)]
fn spawn_text_popup(
    commands: &mut Commands,
    text_popup_event: &TextPopupEvent
) {
    let mut spawned_root = commands.spawn( TextPopup);
}
=======
use bevy::render::color::Color;
use bevy::text::JustifyText;
use bevy::ui::UiRect;
// use crate::system::events::{TextPopup, TextPopupEvent, ScoreBoardType, ShowScoreBoard, TextPopupExpires};

use bevy::{prelude::*, render::camera::RenderTarget, window::WindowRef};

use bevy::prelude::{TextSection, Window}; // Import necessary structs
>>>>>>> Stashed changes
