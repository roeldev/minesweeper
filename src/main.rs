use std::ops::Deref;

use bevy::prelude::*;
use bevy::render::texture::ImageType;
use bevy::sprite::Rect;
use bevy::window::{WindowId, WindowResized};
use bevy::winit::WinitWindows;

use crate::game::grid;
use crate::game::ui::UiComponent;

mod game;
mod utils;
mod load;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::hex("c8c8c8").unwrap()));
    app.insert_resource(WindowDescriptor {
        title: "Minesweeper".to_string(),
        width: 1.,
        height: 1.,
        position: Some(Vec2::splat(10000.)),
        resizable: false,
        cursor_visible: true,
        scale_factor_override: Some(1.),
        ..default()
    });
    app.add_plugins(DefaultPlugins);
    app.add_plugin(load::LoadAssetsPlugin);
    app.add_system_set(
        SystemSet::on_enter(load::LoadState::Loaded)
            .with_system(start_game)
    );
    app.add_startup_system(setup);
    app.add_system(bevy::input::system::exit_on_esc_system);
    app.add_system(center_camera_on_resize);
    app.add_plugin(game::GamePlugin);
    app.run();
}

fn setup(
    mut cmd: Commands,
    winit: NonSend<WinitWindows>,
    asset_server: Res<AssetServer>,
    mut load_assets: ResMut<load::LoadAssets>,
) {
    // window icon
    let _ = bevy_window_icon::set_from_data(
        winit.get_window(WindowId::primary()).expect("Primary winit window does not exist"),
        include_bytes!("../icon.png"),
        ImageType::Extension("png"),
    );

    // camera
    cmd.spawn_bundle(OrthographicCameraBundle::new_2d());

    // grid tiles
    let tile_image = asset_server.load::<Image, _>("tiles.png");
    load_assets.push(tile_image.clone_untyped());

    // ui
    let ui_image = asset_server.load::<Image, _>("ui.png");
    load_assets.push(ui_image.clone_untyped());
}

fn start_game(
    mut cmd: Commands,
    mut state: ResMut<State<game::GameState>>,
    tile_size: Res<grid::TileSize>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    texture_atlases.set_untracked(grid::TILE_TEXTURE_ATLAS, TextureAtlas::from_grid(
        asset_server.get_handle("tiles.png"),
        tile_size.deref().into(),
        grid::Tile::all().len(),
        1,
    ));

    let mut slicer = utils::TextureAtlasSlicer::<UiComponent>::new();
    slicer.add(UiComponent::EdgeCorner, rect(0., 6., 5., 11.));
    slicer.add(UiComponent::SmileyButton, rect(18., 0., 44., 26.));
    slicer.add(UiComponent::SmileyDead, rect(0., 11., 17., 28.));

    let (ui_textures, ui_indexes) = slicer.slice(asset_server.get_handle("ui.png"));
    cmd.insert_resource(utils::SpriteSheetBundleBuilder::new(texture_atlases.add(ui_textures), ui_indexes));

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