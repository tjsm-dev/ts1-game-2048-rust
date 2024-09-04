use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::{WindowCloseRequested, WindowRef, WindowMode, EnabledButtons},
    render::camera::RenderTarget,
};
use crate::system::events::ChangeGameStatus;
use crate::common::status_type::GameStatusType;
use crate::system::resource::GameContext;

const MAIN_MENU_WIDTH: f32 = 200.;
const MAIN_MENU_HEIGHT: f32 = 200.;
pub const POP_UP_MENU_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);

#[derive(Component)]
struct MainMenuHandle;

pub fn show_menu(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut events: EventReader<ChangeGameStatus>,
    mut game_context: ResMut<GameContext>
) {
    for event in events.read() {
        match event.0 {
            GameStatusType::MainMenu => {
                println!("Main Menu Selected!");
                let mesh = Mesh::from(Rectangle::new(MAIN_MENU_WIDTH, MAIN_MENU_HEIGHT));
                let material = ColorMaterial::from(Color::rgb(0., 1., 0.));
            
                let mesh_handle = meshes.add(mesh);
                let material_handle = materials.add(material);
            
                commands.spawn((
                    MainMenuHandle,
                    MaterialMesh2dBundle {
                        mesh: mesh_handle.into(),
                        material: material_handle,
                        ..default()
                    },
                ));
            },
            GameStatusType::Rank => {
                if GameStatusType::Rank == game_context.status {
                    println!("Rank Menu Already Selected!");
                    return;
                }
                println!("Rank Menu Selected!");
                game_context.status = GameStatusType::Rank;

                let second_window = commands
                    .spawn(Window {
                        title: "Rank Menu".to_owned(),
                        
                        prevent_default_event_handling: true,
                        enabled_buttons: EnabledButtons {
                            close: false,
                            minimize: false,
                            maximize: false,
                        },
                        ..default()
                    })
                    .id();
                
                let second_window_camera = commands
                    .spawn(Camera3dBundle {
                        transform: Transform::from_xyz(6.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
                        camera: Camera {
                            target: RenderTarget::Window(WindowRef::Entity(second_window)),
                            ..default()
                        },
                        ..default()
                    })
                    .id();
                commands
                    .spawn((NodeBundle::default(), TargetCamera(second_window_camera)))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Second window",
                            TextStyle::default(),
                        ));
        });
            }
            GameStatusType::Game => {
                println!("Game Menu Selected!");
                game_context.status = GameStatusType::Game;
            }
        }
    }
}

// Add this component to identify the Rank window
#[derive(Component)]
struct RankWindow;

// Add this system to handle window close events
// pub fn handle_window_close(
//     mut close_events: EventReader<WindowCloseRequested>,
//     mut windows: Query<&mut Window>,
// ) {
//     for event in close_events.iter() {
//         if let Ok(window) = windows.get_mut(event.window) {
//             // Prevent the window from closing
//             window.close_requested = false;
//         }
//     }
// }

fn spawn_main_menu(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    println!("Main Menu Selected!");
    let mesh = Mesh::from(Rectangle::new(MAIN_MENU_WIDTH, MAIN_MENU_HEIGHT));
    let material = ColorMaterial::from(Color::rgb(0., 1., 0.));

    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(material);

    commands.spawn((
        MainMenuHandle,
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            ..default()
        },
    ));
}