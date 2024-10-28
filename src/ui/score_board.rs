use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use crate::system::events::{ScoreData, TextPopupEvent};

const SCORE_FILE: &str = "scores.json";

#[derive(Component)]
pub struct ScoreBoard;

#[derive(Resource, Default)]
pub struct ScoreBoardState {
    pub visible: bool,
}

#[derive(Serialize, Deserialize)]
struct ScoreList {
    scores: Vec<ScoreData>,
}

pub fn create_score_board(
    mut commands: Commands,
    mut score_board_state: ResMut<ScoreBoardState>,
    query: Query<Entity, With<ScoreBoard>>,
) {
    // Toggle the visibility state
    score_board_state.visible = !score_board_state.visible;

    // If we're hiding the score board, despawn it
    if !score_board_state.visible {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        return;
    }

    // If the score board is already visible, don't spawn another one
    if query.iter().count() > 0 {
        return;
    }

    let scores = match load_scores() {
        Ok(scores) => scores,
        Err(e) => {
            println!("Error loading scores: {}", e);
            return;
        }
    };
    
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.7)),
                ..default()
            },
            ScoreBoard,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle::from_section(
                "High Scores",
                TextStyle {
                    font_size: 50.0,
                    color: Color::GOLD,
                    ..default()
                },
            ));

            // Subtitle with total number of scores
            parent.spawn(TextBundle::from_section(
                format!("Total Scores: {}", scores.len()),
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            // Spacer
            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(30.0),
                    ..default()
                },
                ..default()
            });

            // Scores list
            for (index, score) in scores.iter().enumerate() {
                let color = if score.score > 900 {
                    Color::GOLD
                } else if score.score > 700 {
                    Color::SILVER
                } else if score.score > 500 {
                    Color::rgb(0.8, 0.5, 0.2) // Bronze
                } else {
                    Color::WHITE
                };

                parent.spawn(TextBundle::from_section(
                    format!("#{}: {} points - {}", index + 1, score.score, score.date),
                    TextStyle {
                        font_size: 30.0,
                        color,
                        ..default()
                    },
                ));
            }

            // Instructions
            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(30.0),
                    ..default()
                },
                ..default()
            });

            parent.spawn(TextBundle::from_section(
                "Press 'S' again to close",
                TextStyle {
                    font_size: 20.0,
                    color: Color::GRAY,
                    ..default()
                },
            ));
        });
}

fn load_scores() -> Result<Vec<ScoreData>, Box<dyn std::error::Error>> {
    if !Path::new(SCORE_FILE).exists() {
        println!("Score file not found");
        return Ok(Vec::new());
    }

    let file_content = fs::read_to_string(SCORE_FILE)?;
    let score_list: ScoreList = serde_json::from_str(&file_content)?;
    Ok(score_list.scores)
}

pub fn save_score(score: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut scores = load_scores().unwrap_or_default();
    
    let new_score = ScoreData {
        score,
        date: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };
    
    scores.push(new_score);
    scores.sort_by(|a, b| b.score.cmp(&a.score));
    scores.truncate(10); // Keep only top 10 scores
    
    let json = serde_json::to_string_pretty(&scores)?;
    fs::write(SCORE_FILE, json)?;
    
    Ok(())
}

pub fn cleanup_score_board(
    mut commands: Commands,
    query: Query<Entity, With<ScoreBoard>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn generate_text_popup_from_event(
    commands: &mut Commands,
    text_popup_event: &TextPopupEvent,
    window: &Window,  // Change to Window reference instead of WindowRef
) {
    spawn_text_popup(
        commands,
        text_popup_event,
        window,
    );
}

#[allow(clippy::too_many_arguments)]
fn spawn_text_popup(
    commands: &mut Commands,
    text_popup_event: &TextPopupEvent,
    window: &Window,  // Change to Window reference
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
    }).set_parent(spawned_root);  // Remove the insert(window_id) call
}

#[derive(Component, Event)]
pub struct TextPopupExpires {
    pub expires_in: f32,
}
