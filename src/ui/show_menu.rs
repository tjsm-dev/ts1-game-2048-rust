use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::system::events::{MenuType, ShowMenu};

const MAIN_MENU_WIDTH: f32 = 200.;
const MAIN_MENU_HEIGHT: f32 = 200.;
pub const POP_UP_MENU_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);

#[derive(Component)]
struct MainMenuHandle;

#[derive(Component)]
struct Position(Vec2);

#[derive(Bundle)]
struct MainMenuBundle {
    handle: MainMenuHandle,
    position: Position,
}

impl MainMenuBundle {
    fn new(x: f32, y: f32) -> Self {
        Self {
            handle: MainMenuHandle,
            position: Position(Vec2::new(x, y)),
        }
    }
}

pub fn show_menu(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut events: EventReader<ShowMenu>
) {
    for event in events.read() {
        match event.0 {
            MenuType::Main => {
                println!("Main Menu Selected!");
                let mesh = Mesh::from(Rectangle::new(MAIN_MENU_WIDTH, MAIN_MENU_HEIGHT));
                let material = ColorMaterial::from(Color::rgb(0., 1., 0.));
            
                let mesh_handle = meshes.add(mesh);
                let material_handle = materials.add(material);
            
                commands.spawn((
                    MainMenuBundle::new(20., -25.),
                    MaterialMesh2dBundle {
                        mesh: mesh_handle.into(),
                        material: material_handle,
                        ..default()
                    },
                ));
            },
            MenuType::Rank => {
                println!("Rank Menu Selected!")
            }
        }
    }
}

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
        MainMenuBundle::new(20., -25.),
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            ..default()
        },
    ));
}