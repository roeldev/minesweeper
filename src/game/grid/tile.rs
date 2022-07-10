use std::hash::{Hash, Hasher};

use bevy::prelude::*;
use bevy::sprite::Anchor;

pub const TILE_TEXTURE_ATLAS: &str = "tiles";

/// Tile size resource
pub struct TileSize(Vec2);

impl TileSize {
    pub fn new(size: f32) -> Self {
        Self { 0: Vec2::splat(size) }
    }
}

impl Default for TileSize {
    fn default() -> Self { Self::new(24.) }
}

impl From<&TileSize> for Vec2 {
    fn from(val: &TileSize) -> Self { val.0 }
}

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
pub enum Tile {
    Default,
    Flag,
    Mine,
    Boom,
}

impl Tile {
    #[inline(always)]
    pub fn all() -> [Self; 4] {
        use Tile::*;
        [Default, Flag, Mine, Boom]
    }

    #[inline]
    pub fn index(&self) -> usize {
        use Tile::*;
        return match self {
            Default => 0,
            Flag => 1,
            Mine => 2,
            Boom => 3,
        };
    }

    #[inline(always)]
    pub fn change(&mut self, set: Self) {
        *self = set;
    }
}

impl Hash for Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.index());
    }
}

#[derive(Bundle)]
pub struct TileBundle {
    pub tile: Tile,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}

impl TileBundle {
    #[inline]
    pub fn spawn(cmd: &mut Commands, texture_atlas: Handle<TextureAtlas>, x: f32, y: f32) -> Entity {
        let mut sprite = TextureAtlasSprite::new(Tile::Default.index());
        sprite.anchor = Anchor::BottomLeft;

        cmd.spawn_bundle(Self {
            tile: Tile::Default,
            sprite: SpriteSheetBundle {
                sprite,
                texture_atlas,
                transform: Transform::from_xyz(x, y, 0.),
                ..Default::default()
            },
        }).id()
    }
}
