mod colors;

use bevy::prelude::*;
use itertools::Itertools;
use rand::prelude::IteratorRandom;
const TILE_SIZE: f32 = 40.0;
const TILE_SPACER: f32 = 10.0;

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
        .add_systems(Startup, (setup, spawn_board))
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Points {
    value: u32,
}

#[derive(Component)]
struct Positon {
    x: u8,
    y: u8,
}

impl Positon {
    fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

#[derive(Component)]
struct Board {
    size: u8,
    physical_size: f32,
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

fn spawn_tile(mut commands: Commands, query_board: Query<&Board>) {
    let board = query_board.single();

    let mut rng = rand::thread_rng();

    let starting_tiles: Vec<(u8, u8)> = (0..board.size)
        .cartesian_product(0..board.size)
        .choose_multiple(&mut rng, 2);

    for(x, y) in starting_tiles.iter() {
        let pos = Positon::new(*x, *y);
        commands
            .spawn(
                SpriteBundle {
                    sprite: Sprite { 
                        color: colors::TITLE_PLACEHOLDER,
                        custom_size: Some(board.size()),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        board.cell_position_to_physical(pos.x), 
                        board.cell_position_to_physical(pos.y), 
                        1.0,
                    ),
                    ..default()
            })
            .insert(Points {value: 2})
            .insert(pos);
    }
}