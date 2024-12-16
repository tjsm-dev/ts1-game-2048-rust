use bevy::prelude::*;

use crate::system::events::ShowScoreBoard;
use crate::system::data::load_scores;

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
