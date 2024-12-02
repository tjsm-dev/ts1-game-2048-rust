use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use crate::system::events::{ScoreData, ShowScoreBoard, ToggleScoreBoard};

const SCORE_FILE: &str = "scores.json";

#[derive(Component)]
pub struct ScoreBoard;

#[derive(Resource, Default)]
pub struct ScoreBoardState {
    pub visible: bool,
}

pub fn create_score_board(
    mut commands: Commands,
    mut score_board_state: ResMut<ScoreBoardState>,
    query: Query<Entity, With<ScoreBoard>>,
    mut events: EventReader<ShowScoreBoard>,
) {
    for _ in events.read() {
        score_board_state.visible = true;
        println!("Received ShowScoreBoard event");  // Debug print
        
        // Clear existing score board if any
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
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
                    z_index: ZIndex::Global(100),  // Make sure it appears on top
                    ..default()
                },
                ScoreBoard,
            ))
            .with_children(|parent| {
                // Title with game over message
                parent.spawn(TextBundle::from_section(
                    "Game Over!",
                    TextStyle {
                        font_size: 50.0,
                        color: Color::RED,
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

                // High Scores title
                parent.spawn(TextBundle::from_section(
                    "High Scores",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::GOLD,
                        ..default()
                    },
                ));

                // Scores list
                for (index, score) in scores.iter().enumerate() {
                    parent.spawn(TextBundle::from_section(
                        format!("#{}: {} points - {}", index + 1, score.score, score.date),
                        TextStyle {
                            font_size: 30.0,
                            color: Color::WHITE,
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
                    "Press any key to start new game",
                    TextStyle {
                        font_size: 25.0,
                        color: Color::GRAY,
                        ..default()
                    },
                ));
            });
    }
}

fn load_scores() -> Result<Vec<ScoreData>, Box<dyn std::error::Error>> {
    if !Path::new(SCORE_FILE).exists() {
        println!("Score file not found");
        return Ok(Vec::new());
    }

    let file_content = fs::read_to_string(SCORE_FILE)?;
    let scores: Vec<ScoreData> = serde_json::from_str(&file_content)?;
    Ok(scores)
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

pub fn handle_score_board_input(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<ScoreBoard>>,
) {
    // If any key is pressed and score board exists
    if keyboard.get_pressed().count() > 0 && !query.is_empty() {
        // Remove the score board
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn toggle_score_board(
    mut score_board_state: ResMut<ScoreBoardState>,
    mut events: EventReader<ToggleScoreBoard>,
) {
    for _ in events.read() {
        score_board_state.visible = !score_board_state.visible;
    }
}