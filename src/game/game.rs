use bevy::prelude::*;
use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use rand::prelude::SliceRandom;
use winit::dpi::PhysicalSize;

use grid::{Grid, ResizeResult, Tile, TileAssets};

use crate::utils;

use super::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    // load assets etc.
    Setup,
    // start a new game, resets previous game
    Start,
    // game is running, await user input
    Run,
    // game over
    Over,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Difficulty::default());
        app.add_state(GameState::Setup);
        app.add_system(update_tile_sprite);

        app.add_system_set(
            SystemSet::on_exit(GameState::Setup)
                .with_system(setup_components)
        );

        app.add_system_set(
            SystemSet::on_enter(GameState::Start)
                .with_system(update_grid)
                .with_system(update_ui.after(update_grid))
        );

        app.add_system_set(
            SystemSet::on_update(GameState::Run)
                .with_system(handle_grid_click)
        );

        app.add_system_set(
            SystemSet::on_enter(GameState::Over)
                .with_system(reveal_mines)
        );
    }
}

// setup all basic game components
fn setup_components(
    mut cmd: Commands,
    difficulty: Res<Difficulty>,
    tile_assets: Res<TileAssets>,
) {
    cmd.spawn_bundle(
        TransformBundle::default())
        .insert(Grid::new(difficulty.size(), tile_assets.tile_size()));
}

// add/remove tiles to grid according to difficulty
fn update_grid(
    mut cmd: Commands,
    difficulty: Res<Difficulty>,
    tile_assets: Res<TileAssets>,
    mut grid_query: Query<(Entity, &mut Grid)>,
    mut tiles_query: Query<(&mut Tile, &mut Transform)>,
) {
    let (grid_entity, mut grid) = grid_query.single_mut();
    if let ResizeResult::Shrink(entities) = grid.resize(difficulty.size()) {
        cmd.entity(grid_entity)
            .remove_children(entities.as_ref());
    }

    let mut new_tile_entities = Vec::<Entity>::new();
    for col in 0..grid.size().columns() {
        for row in 0..grid.size().rows() {
            let x = col as f32 * tile_assets.tile_size().x;
            let y = row as f32 * tile_assets.tile_size().y;

            if let Some(tile_entity) = grid.get_tile(col, row) {
                let (mut tile, mut tile_transform) = tiles_query.get_mut(tile_entity).unwrap();
                tile.change(Tile::Default);
                tile_transform.translation.x = x;
                tile_transform.translation.y = y;
            } else {
                let tile = tile_assets.build_bundle(x, y);
                let tile_entity = cmd.spawn_bundle(tile).id();
                grid.insert(col, row, tile_entity);
                new_tile_entities.push(tile_entity);
            }
        }
    }

    if !new_tile_entities.is_empty() {
        cmd.entity(grid_entity)
            .push_children(new_tile_entities.as_ref());
    }
}

fn update_ui(
    mut state: ResMut<State<GameState>>,
    winit: NonSend<WinitWindows>,
    mut grid_query: Query<(&Grid, &mut Transform)>,
) {
    let (grid, mut grid_transform) = grid_query.single_mut();
    grid_transform.translation.x = 12.;
    grid_transform.translation.y = 12.;

    let width = grid.width() + (grid_transform.translation.x * 2.);
    let height = grid.height() + (grid_transform.translation.y * 2.);

    let window = winit.get_window(WindowId::primary()).unwrap();
    window.set_inner_size(PhysicalSize::new(width as u32, height as u32));
    utils::center_window(window);

    state.set(GameState::Run).expect("run game");
}

fn handle_grid_click(
    mut state: ResMut<State<GameState>>,
    windows: Res<Windows>,
    mouse_button: Res<Input<MouseButton>>,
    grid_query: Query<(&Grid, &Transform)>,
    mut tiles_query: Query<&mut Tile>,
) {
    let mut btn: Option<MouseButton> = None;
    if mouse_button.just_pressed(MouseButton::Left) {
        btn = Some(MouseButton::Left);
    } else if mouse_button.just_pressed(MouseButton::Right) {
        btn = Some(MouseButton::Right);
    } else {
        return;
    }

    windows.get_primary()
        .and_then(|window| {
            window.cursor_position()
        })
        .and_then(|cursor_position| {
            let (grid, grid_transform) = grid_query.single();
            grid.get_tile_xy(
                cursor_position.x - grid_transform.translation.x,
                cursor_position.y - grid_transform.translation.y,
            )
        })
        .and_then(|entity| {
            tiles_query.get_mut(entity).ok()
        })
        .map(|mut tile| {
            let tile = &mut *tile;

            use Tile::*;
            match btn.unwrap() {
                MouseButton::Left => {
                    tile.change(Boom);
                    state.set(GameState::Over).expect("game over");
                }
                MouseButton::Right => {
                    match tile {
                        Default => { tile.change(Flag); }
                        Flag => { tile.change(Default); }
                        _ => {}
                    }
                }
                _ => {}
            }
        });
}

// make sure a tile's sprite is updated according to it's kind
fn update_tile_sprite(
    mut tiles_query: Query<(&Tile, &mut TextureAtlasSprite), Changed<Tile>>
) {
    for (tile, mut sprite) in tiles_query.iter_mut() {
        sprite.index = tile.index();
    }
}

fn reveal_mines(
    difficulty: Res<Difficulty>,
    mut tiles_query: Query<(Entity, &mut Tile)>,
) {
    let mut want = difficulty.mines();
    let mut remaining_tiles = Vec::<Entity>::with_capacity(tiles_query.iter().len());

    for (tile_entity, mut tile) in tiles_query.iter_mut() {
        let tile = &mut *tile;
        match tile {
            Tile::Boom => {
                want -= 1;
            }
            Tile::Mine => {
                want -= 1;
                tile.change(Tile::Boom);
            }
            _ => {
                remaining_tiles.push(tile_entity);
            }
        }
    }

    remaining_tiles.shuffle(&mut rand::thread_rng());
    remaining_tiles.truncate(want);

    for tile in remaining_tiles {
        let (_, mut tile) = tiles_query.get_mut(tile).unwrap();
        tile.change(Tile::Mine);
    }
}