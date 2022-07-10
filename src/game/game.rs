use std::borrow::BorrowMut;
use std::ops::Deref;

use bevy::prelude::*;
use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use rand::prelude::SliceRandom;
use winit::dpi::PhysicalSize;

use crate::load::LoadState;
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
        app.insert_resource(ui::Colors::default());
        app.insert_resource(5. as ui::EdgeSize);
        app.insert_resource(8. as ui::EdgePadding);
        app.insert_resource(grid::TileSize::default());
        app.insert_resource(Difficulty::default());
        app.add_state(GameState::Setup);

        app.add_system_to_stage(CoreStage::PostUpdate, update_tile_sprite);
        app.add_system_set(
            SystemSet::on_update(LoadState::Loaded)
                .with_system(change_difficulty)
        );

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
    ui_colors: Res<ui::Colors>,
    ui_sprites: Res<utils::SpriteSheetBundleBuilder<ui::UiComponent>>,
    difficulty: Res<Difficulty>,
    tile_size: Res<grid::TileSize>,
) {
    // edge
    ui::Edge::spawn(
        cmd.borrow_mut(),
        ui_colors.deref(),
        ui_sprites.deref(),
    );

    // grid
    grid::Grid::spawn(
        cmd.borrow_mut(),
        difficulty.size(),
        tile_size.deref().into(),
    );
}

// add/remove tiles to grid according to difficulty
fn update_grid(
    mut cmd: Commands,
    difficulty: Res<Difficulty>,
    tile_size: Res<grid::TileSize>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut grid_query: Query<(Entity, &mut grid::Grid)>,
    mut tiles_query: Query<(&mut grid::Tile, &mut Transform)>,
) {
    let (grid_entity, mut grid) = grid_query.single_mut();
    if let grid::ResizeResult::Shrink(entities) = grid.resize(difficulty.size()) {
        cmd.entity(grid_entity).remove_children(entities.as_ref());
        for tile_entity in entities {
            cmd.entity(tile_entity).despawn();
        }
    }

    let tile_size: Vec2 = tile_size.deref().into();
    let mut new_tile_entities = Vec::<Entity>::new();

    for col in 0..grid.size().columns() {
        for row in 0..grid.size().rows() {
            let x = col as f32 * tile_size.x;
            let y = row as f32 * tile_size.y;

            if let Some(tile_entity) = grid.get_tile(col, row) {
                let (mut tile, mut tile_transform) = tiles_query.get_mut(tile_entity).unwrap();
                tile.change(grid::Tile::Default);
                tile_transform.translation.x = x;
                tile_transform.translation.y = y;
            } else {
                let tile_entity = grid::TileBundle::spawn(
                    cmd.borrow_mut(),
                    texture_atlases.get_handle(grid::TILE_TEXTURE_ATLAS),
                    x,
                    y,
                );

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
    mut set: ParamSet<(
        Query<(&grid::Grid, &mut Transform)>,
        Query<(&ui::EdgeSide, &mut Sprite, &mut Transform)>,
        Query<(&ui::EdgeCorner, &mut Transform)>,
    )>,
) {
    let mut grid_query = set.p0();
    let (grid, mut grid_transform) = grid_query.single_mut();
    grid_transform.translation.x = 18.;
    grid_transform.translation.y = 18.;

    let width = grid.width() + (grid_transform.translation.x * 2.);
    let height = grid.height() + (grid_transform.translation.y * 2.);

    let edge_size = 5.;
    let mut edge_sides_query = set.p1();
    for (edge_side, mut edge_sprite, mut edge_transform) in edge_sides_query.iter_mut() {
        use ui::EdgeSide;
        match edge_side {
            EdgeSide::Top => {
                edge_sprite.custom_size = Some(Vec2::new(width, edge_size));
                edge_transform.translation.y = height;
            }
            EdgeSide::Left => {
                edge_sprite.custom_size = Some(Vec2::new(edge_size, height));
            }
            EdgeSide::Bottom => {
                edge_sprite.custom_size = Some(Vec2::new(width, edge_size));
            }
            EdgeSide::Right => {
                edge_sprite.custom_size = Some(Vec2::new(edge_size, height));
                edge_transform.translation.x = width;
            }
        }
    }

    let mut edge_corners_query = set.p2();
    for (edge_corner, mut edge_transform) in edge_corners_query.iter_mut() {
        use ui::EdgeCorner;
        match edge_corner {
            EdgeCorner::BottomLeft => {}
            EdgeCorner::TopRight => {
                edge_transform.translation.x = width;
                edge_transform.translation.y = height;
            }
        }
    }

    let window = winit.get_window(WindowId::primary()).unwrap();
    window.set_inner_size(PhysicalSize::new(width as u32, height as u32));
    // todo: center window once
    utils::center_window(window);

    let _ = state.set(GameState::Run);
}

fn handle_grid_click(
    mut state: ResMut<State<GameState>>,
    windows: Res<Windows>,
    mouse_button: Res<Input<MouseButton>>,
    grid_query: Query<(&grid::Grid, &Transform)>,
    mut tiles_query: Query<&mut grid::Tile>,
) {
    #[allow(unused_assignments)]
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

            use grid::Tile;
            match btn.unwrap() {
                MouseButton::Left => {
                    tile.change(Tile::Boom);
                    let _ = state.set(GameState::Over);
                }
                MouseButton::Right => {
                    match tile {
                        Tile::Default => { tile.change(Tile::Flag); }
                        Tile::Flag => { tile.change(Tile::Default); }
                        _ => {}
                    }
                }
                _ => {}
            }
        });
}

// make sure a tile's sprite is updated according to it's kind
fn update_tile_sprite(
    mut tiles_query: Query<(&grid::Tile, &mut TextureAtlasSprite), Changed<grid::Tile>>
) {
    for (tile, mut sprite) in tiles_query.iter_mut() {
        let index = tile.index();
        if sprite.index != index {
            sprite.index = index;
        }
    }
}

// reveal mines when the game ends
fn reveal_mines(
    difficulty: Res<Difficulty>,
    mut tiles_query: Query<(Entity, &mut grid::Tile)>,
) {
    let mut want = difficulty.mines();
    let mut remaining_tiles = Vec::<Entity>::with_capacity(tiles_query.iter().len());

    for (tile_entity, mut tile) in tiles_query.iter_mut() {
        let tile = &mut *tile;

        use grid::Tile;
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
        tile.change(grid::Tile::Mine);
    }
}

// change difficulty on F1/F2/F3 key press
fn change_difficulty(
    key: Res<Input<KeyCode>>,
    mut difficulty: ResMut<Difficulty>,
    mut state: ResMut<State<GameState>>,
) {
    if key.just_released(KeyCode::F1) {
        difficulty.change(Difficulty::Beginner);
    } else if key.just_released(KeyCode::F2) {
        difficulty.change(Difficulty::Intermediate);
    } else if key.just_released(KeyCode::F3) {
        difficulty.change(Difficulty::Expert);
    } else {
        return;
    }

    let _ = state.set(GameState::Start);
}