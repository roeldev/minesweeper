use bevy::prelude::*;
use bevy::sprite::Rect;
use bevy::window::{WindowId, WindowResized};
use bevy::winit::WinitWindows;

use crate::game::grid::{Tile, TileAssets};

mod game;
mod utils;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("c8c8c8").unwrap()))
        .insert_resource(WindowDescriptor {
            title: "Minesweeper".to_string(),
            width: 1.,
            height: 1.,
            resizable: false,
            cursor_visible: true,
            scale_factor_override: Some(1.),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_window)
        .add_startup_system(setup_game)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system(center_camera_on_resize)
        .add_plugin(game::GamePlugin)
        .run();
}

fn setup_window(winit: NonSend<WinitWindows>) {
    let window = winit.get_window(WindowId::primary())
        .expect("Primary winit window does not exist");

    let _ = bevy_icon::set_window_icon(window, include_bytes!("../icon.png"), "png");
}

fn setup_game(
    mut cmd: Commands,
    mut state: ResMut<State<game::GameState>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // camera
    cmd.spawn_bundle(OrthographicCameraBundle::new_2d());

    // tiles
    let tile_size = Vec2::splat(24.);
    let tiles_textures = TextureAtlas::from_grid(
        asset_server.load("tiles.png"),
        tile_size,
        Tile::all().len(),
        1,
    );
    cmd.insert_resource(TileAssets::new(
        tile_size,
        texture_atlases.add(tiles_textures),
    ));

    // ui
    // let mut sprites_textures = TextureAtlas::new_empty(asset_server.load("sprites.png"), Vec2::ZERO);
    // let sprites = vec![
    //     (game::SpriteKind::SmileyButton, sprites_textures.add_texture(rect(0., 16., 26., 42.))),
    // ];
    //
    // cmd.insert_resource(SpritesBuilder::new(slicer.sprites, texture_atlases.add(slicer.texture_atlas)));
    let _ = state.set(game::GameState::Start);
}

fn center_camera_on_resize(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut resize_event: EventReader<WindowResized>,
) {
    if let Some(resized) = resize_event.iter().last() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation.x = resized.width * 0.5;
            camera_transform.translation.y = resized.height * 0.5;
        }
    }
}

#[inline(always)]
fn rect(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Rect {
    Rect {
        min: Vec2::new(min_x, min_y),
        max: Vec2::new(max_x, max_y),
    }
}