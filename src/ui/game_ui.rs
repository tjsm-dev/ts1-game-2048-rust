use bevy::prelude::*;
use crate::component::board::Board;
use crate::component::animation::{MoveAnimation, MergeAnimation};
use crate::system::resource::GameContext;

#[derive(Component)]
pub struct GameUI;

#[derive(Component)]
pub struct GameGrid;

#[derive(Component)]
pub struct ScorePanel;

#[derive(Component)]
pub struct BestScorePanel;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct BestScoreText;

#[derive(Component)]
pub struct TileText;

// Constants for grid and tile sizing
const GRID_SIZE: f32 = 400.0;  // Total grid size
const GRID_PADDING: f32 = 12.0;
const CELL_MARGIN: f32 = 6.0;
// Calculate cell size based on grid size: (grid - 2*padding - 8*margin) / 4
const CELL_SIZE: f32 = (GRID_SIZE - 2.0 * GRID_PADDING - 8.0 * CELL_MARGIN) / 4.0;
const ANIMATION_DURATION: f32 = 0.15;

pub fn spawn_game_ui(mut commands: Commands, game_context: Res<GameContext>) {
    // Root container (takes up entire window)
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,  // Add this
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                // Add a background color to the root container
                background_color: Color::rgb(0.86, 0.86, 0.86).into(), // Light gray background
                ..default()
            },
        ))
        .with_children(|parent| {
            // Game container (fixed size, centered)
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(GRID_SIZE),
                        // Add some margin to ensure it doesn't touch window edges
                        margin: UiRect::all(Val::Auto),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,      // Center children horizontally
                        justify_content: JustifyContent::Center, // Center children vertically
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|parent| {
                // Header with scores
                parent
                    .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                justify_content: JustifyContent::SpaceBetween,
                                margin: UiRect::bottom(Val::Px(20.0)),
                                ..default()
                            },
                            ..default()
                        },
                    )
                    .with_children(|parent| {
                        spawn_score_container(parent, "Score", 0, ScorePanel, ScoreText);
                        spawn_score_container(parent, "Best", game_context.best_score, BestScorePanel, BestScoreText);
                    });

                // Game Grid
                parent
                    .spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(GRID_SIZE),
                                height: Val::Px(GRID_SIZE),
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(GRID_PADDING)),
                                ..default()
                            },
                            background_color: Color::rgb(0.75, 0.71, 0.66).into(),
                            ..default()
                        },
                        GameGrid,
                    ))
                    .with_children(|parent| {
                        for _ in 0..4 {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(25.0),
                                        flex_direction: FlexDirection::Row,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    for _ in 0..4 {
                                        spawn_cell(parent);
                                    }
                                });
                        }
                    });
            });
        });
}

fn spawn_score_container(
    parent: &mut ChildBuilder,
    label: &str,
    initial_score: u32,
    panel_component: impl Component,
    score_component: impl Component,
) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    min_width: Val::Px(100.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::rgb(0.7, 0.6, 0.5)),
                ..default()
        }, panel_component
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font_size: 20.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
            parent.spawn((
                TextBundle::from_section(
                    initial_score.to_string(),
                    TextStyle {
                        font_size: 30.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ),
                score_component,
            ));
        });
}

fn spawn_cell(parent: &mut ChildBuilder) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Px(CELL_SIZE),
            height: Val::Px(CELL_SIZE),
            margin: UiRect::all(Val::Px(CELL_MARGIN)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgb(0.85, 0.8, 0.75).into(),
        ..default()
    });
}

// Add this system to sync board state with UI
pub fn sync_board_with_ui(
    board: Res<Board>,
    mut commands: Commands,
    grid_query: Query<Entity, With<GameGrid>>,
    tile_query: Query<Entity, With<TileText>>,
    panel_query: Query<Entity, With<ScorePanel>>,
    score_query: Query<Entity, With<ScoreText>>,
) {
    // First, remove ALL existing tile texts
    for entity in tile_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in score_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // 점수 무한 중첩 그리기 상태
    let panel_entity = panel_query.single();

    commands.entity(panel_entity).with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                board.score.to_string(),
                TextStyle {
                    font_size: 30.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ),
            ScoreText,
        ));
    });

    let grid_entity = grid_query.single();
    
    // Create a new tile for each non-zero value
    for row in 0..4 {
        for col in 0..4 {
            let value = board.tiles[row][col].value;
            if value > 0 {
                commands.entity(grid_entity).with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(CELL_SIZE),
                                height: Val::Px(CELL_SIZE),
                                position_type: PositionType::Absolute,
                                left: Val::Px(GRID_PADDING + col as f32 * (CELL_SIZE + CELL_MARGIN * 2.0) + CELL_MARGIN),
                                top: Val::Px(GRID_PADDING + row as f32 * (CELL_SIZE + CELL_MARGIN * 2.0) + CELL_MARGIN),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: get_tile_color(value as u32),
                            ..default()
                        },
                        TileText,
                    )).with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            value.to_string(),
                            TextStyle {
                                font_size: match value {
                                    v if v >= 1000 => 32.0,
                                    v if v >= 100 => 36.0,
                                    _ => 40.0,
                                },
                                color: if value <= 4 {
                                    Color::rgb(0.47, 0.43, 0.39)
                                } else {
                                    Color::rgb(0.97, 0.96, 0.94)
                                },
                                ..default()
                            },
                        ));
                    });
                });
            }
        }
    }
}

fn get_tile_color(value: u32) -> BackgroundColor {
    match value {
        2 => Color::rgb(0.93, 0.89, 0.85),    // #EEE4DA
        4 => Color::rgb(0.93, 0.88, 0.78),    // #EDE0C8
        8 => Color::rgb(0.95, 0.69, 0.47),    // #F2B179
        16 => Color::rgb(0.96, 0.58, 0.39),   // #F59563
        32 => Color::rgb(0.96, 0.49, 0.37),   // #F67C5F
        64 => Color::rgb(0.96, 0.37, 0.23),   // #F65E3B
        128 => Color::rgb(0.93, 0.81, 0.45),  // #EDCF72
        256 => Color::rgb(0.93, 0.80, 0.38),  // #EDCC61
        512 => Color::rgb(0.93, 0.78, 0.31),  // #EDC850
        1024 => Color::rgb(0.93, 0.77, 0.25), // #EDC53F
        2048 => Color::rgb(0.93, 0.76, 0.18), // #EDC22E - Victory!
        _ => Color::rgb(0.79, 0.75, 0.71),    // #CDC1B4 - Should never happen in normal gameplay
    }.into()
}

pub fn animate_tiles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Style, &mut MoveAnimation)>,
) {
    for (entity, mut style, mut anim) in query.iter_mut() {
        anim.timer.tick(time.delta());
        let progress = anim.timer.elapsed_secs() / anim.timer.duration().as_secs_f32();
        
        // Smooth easing function
        let progress = ease_out_quad(progress);
        
        // Lerp between start and end positions
        let current_x = anim.start_pos.x + (anim.end_pos.x - anim.start_pos.x) * progress;
        let current_y = anim.start_pos.y + (anim.end_pos.y - anim.start_pos.y) * progress;
        
        style.left = Val::Px(current_x);
        style.top = Val::Px(current_y);

        if anim.timer.finished() {
            commands.entity(entity).remove::<MoveAnimation>();
        }
    }
}

// Add a smooth easing function
fn ease_out_quad(x: f32) -> f32 {
    1.0 - (1.0 - x) * (1.0 - x)
}

pub fn animate_merges(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Style, &mut MergeAnimation)>,
) {
    for (entity, mut style, mut anim) in query.iter_mut() {
        anim.timer.tick(time.delta());
        let progress = anim.timer.elapsed_secs() / anim.timer.duration().as_secs_f32();
        
        // Scale up then down
        let scale = if progress < 0.5 {
            1.0 + (progress * 0.2) // Scale up to 1.2
        } else {
            1.2 - ((progress - 0.5) * 0.4) // Scale down to 1.0
        };
        
        style.width = Val::Px(CELL_SIZE * scale);
        style.height = Val::Px(CELL_SIZE * scale);

        if anim.timer.finished() {
            commands.entity(entity).remove::<MergeAnimation>();
        }
    }
}