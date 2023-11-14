mod colors;

use std::cmp::Ordering;
use bevy::prelude::*;
use itertools::Itertools;
use rand::prelude::IteratorRandom;
const TILE_SIZE: f32 = 40.0;
const TILE_SPACER: f32 = 10.0;

enum BoardShift {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component)]
struct Board {
    size: u8,
    physical_size: f32,
}

#[derive(Component)]
struct Positon {
    x: u8,
    y: u8,
}

#[derive(Component)]
struct Points {
    value: u32,
}

#[derive(Component)]
struct TileText;

#[derive(Resource)]
struct FontSpec {
    family: Handle<Font>,
}

impl FromWorld for FontSpec {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world
            .get_resource_mut::<AssetServer>()
            .unwrap();
        FontSpec { family: asset_server.load("fonts/FiraSans-Bold.ttf") }
    }
}

impl Positon {
    fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

impl TryFrom<&KeyCode> for BoardShift {
    type Error = &'static str;

    fn try_from(value: &KeyCode) -> Result<Self, Self::Error> {
        match value {
            KeyCode::W => Ok(BoardShift::Up),
            KeyCode::A => Ok(BoardShift::Left),
            KeyCode::S => Ok(BoardShift::Down),
            KeyCode::D => Ok(BoardShift::Right),
            _ => Err("Invalid keycode"),
        }
    }
}

impl Board {
    fn new(size: u8) -> Self {
        let physical_size = f32::from(size) * TILE_SIZE + f32::from(size + 1) * TILE_SPACER;
        Self {
            size,
            physical_size,
        }
    }

    fn cell_position_to_physical(&self, pos: u8) -> f32 {
        let offset = -self.physical_size / 2.0 + 0.5 * TILE_SIZE;

        offset + f32::from(pos) * TILE_SIZE + f32::from(pos + 1) * TILE_SPACER
    }

    fn size(&self) -> Vec2 {
        Vec2::new(self.physical_size, self.physical_size)
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("#1f2638").unwrap()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2048".to_string(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<FontSpec>()
        .add_systems(
            Startup,
            (setup, spawn_board, apply_deferred, spawn_tile).chain(),
        )
        .add_systems(Update, (render_tile_points, board_shift))
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_board(mut commands: Commands) {
    let board = Board::new(4);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: colors::BOARD,
                custom_size: Some(board.size()),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            for tile in (0..board.size).cartesian_product(0..board.size) {
                builder.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: colors::TITLE_PLACEHOLDER,
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        board.cell_position_to_physical(tile.0),
                        board.cell_position_to_physical(tile.1),
                        1.0,
                    ),
                    ..default()
                });
            }
        })
        .insert(board);
}

fn spawn_tile(mut commands: Commands, query_board: Query<&Board>, font_spec: Res<FontSpec>) {
    let board = query_board.single();

    let mut rng = rand::thread_rng();

    let starting_tiles: Vec<(u8, u8)> = (0..board.size)
        .cartesian_product(0..board.size)
        .choose_multiple(&mut rng, 2);

    for (x, y) in starting_tiles.iter() {
        let pos = Positon::new(*x, *y);
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: colors::TILE,
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    board.cell_position_to_physical(pos.x),
                    board.cell_position_to_physical(pos.y),
                    1.0,
                ),
                ..default()
            })
            .with_children(|builder| {
                builder.spawn(Text2dBundle {
                    text: Text::from_section(
                        "4",
                        TextStyle {
                            font: font_spec.family.clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                            ..default()
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    transform: Transform::from_xyz(0.0, 0.0, 1.0),
                    ..default()
                })
                .insert(TileText);
            })
            .insert(Points { value: 2 })
            .insert(pos);
    }
}

fn render_tile_points(mut texts: Query<&mut Text, With<TileText>>, tiles: Query<(&Points, &Children)>) {
    
    for (point, children) in tiles.iter() {
        if let Some(entity) = children.first() {
            let mut text = texts.get_mut(*entity).expect("No text component found!");
            let mut text_section = text.sections.first_mut().expect("Can't get accessible text");
            text_section.value = point.value.to_string();
        }
    }
}

fn board_shift(
    input: Res<Input<KeyCode>>,
    mut tiles: Query<(Entity, &mut Positon, &mut Points)>
) {
    
    let shift_direction = input.get_just_pressed().find_map(
        |key_code| BoardShift::try_from(key_code).ok(),
    );

    match shift_direction {
        Some(BoardShift::Left) => {
            dbg!("Left");

            let mut it = tiles.iter_mut().sorted_by(|a, b| {
                match Ord::cmp(&a.1.y, &b.1.y) {
                    Ordering::Equal => {
                        Ord::cmp(&a.1.x, &b.1.x)
                    }
                    ordering => ordering,
                }
            });
        }
        Some(BoardShift::Right) => {
            dbg!("Right");
        }
        Some(BoardShift::Up) => {
            dbg!("Up");
        }
        Some(BoardShift::Down) => {
            dbg!("Down");
        }
        None => (),
    }
}

