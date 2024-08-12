use bevy::{
    asset::Handle, hierarchy::BuildChildren, prelude::{AlignItems, Commands, EventReader, FlexDirection, JustifyContent, NodeBundle, Style, Text, TextBundle, TextStyle, Val}
};
use bevy::render::color::Color;
use bevy::text::JustifyText;
use bevy::ui::UiRect;
use crate::system::events::{TextPopup, TextPopupEvent, ScoreBoardType, ShowScoreBoard, TextPopupExpires};
use bevy::prelude::{TextSection, Window}; // Import necessary structs

pub fn show_score_board(
    mut commands: Commands,
    mut events: EventReader<ShowScoreBoard>,
    mut windows: ResMut<Window>// Add windows resource
) {
    for event in events.read() {
        match event.0 {
            ScoreBoardType::User => {
                println!("Rank Score Board Selected!");
            },
            ScoreBoardType::Score => {
                // Create a new window
                let window_id = windows.add(Window {
                    title: "Score Board".to_string(),
                    width: 400.0,
                    height: 300.0,
                    ..Default::default()
                });

                generate_text_popup_from_event(
                    &mut commands,
                    &TextPopupEvent {
                        content: "User Score Board Selected!".to_string(),
                        font: Some(Handle::default()),
                        font_size: 12.0,
                        font_color: Color::WHITE,
                        border: UiRect::all(Val::Px(5.)),
                        border_color: Color::BLACK,
                        padding: UiRect::all(Val::Px(5.)),
                        margin: UiRect::all(Val::Px(5.)),
                        modal: None,
                        text_alignment: JustifyText::Center,
                        background_color: Color::GRAY,
                    },
                    window_id, // Pass the window ID
                );
                println!("User Score Board Selected!");
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
    window_id: WindowId, // Add window ID parameter
) {
    spawn_text_popup(
        commands,
        text_popup_event,
        window_id, // Pass the window ID
    );
}

#[allow(clippy::too_many_arguments)]
fn spawn_text_popup(
    commands: &mut Commands,
    text_popup_event: &TextPopupEvent,
    window_id: WindowId, // Add window ID parameter
) {
    let spawned_root = commands.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        ..Default::default()
    }).id();

    commands.entity(spawned_root).insert(TextPopupExpires {
        expires_in: 2.0,
    });

    commands.spawn(TextBundle {
        text: Text::from_section(
            &text_popup_event.content,
            TextStyle {
                font: text_popup_event.font.clone().unwrap_or_default(),
                font_size: text_popup_event.font_size,
                color: text_popup_event.font_color,
            }
        ),
        style: Style {
            margin: text_popup_event.margin,
            padding: text_popup_event.padding,
            ..Default::default()
        },
        ..Default::default()
    }).set_parent(spawned_root).insert(WindowId(window_id)); // Associate with the new window
}
