use std::hash::{Hash, Hasher};

use bevy::prelude::*;
use bevy::sprite::Anchor;

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq)]
pub enum Tile {
    Default,
    Flag,
    Mine,
    Boom,
}

impl Tile {
    pub fn all() -> [Tile; 4] {
        use Tile::*;
        [Default, Flag, Mine, Boom]
    }

    pub fn index(&self) -> usize {
        use Tile::*;
        return match self {
            Default => 0,
            Flag => 1,
            Mine => 2,
            Boom => 3,
        };
    }

    pub fn change(&mut self, set: Tile) {
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

/// Resource to store grid tiles' textures.
pub struct TileAssets {
    tile_size: Vec2,
    texture_atlas_handle: Handle<TextureAtlas>,
}

impl TileAssets {
    #[inline(always)]
    pub fn new(tile_size: Vec2, texture_atlas_handle: Handle<TextureAtlas>) -> Self {
        Self {
            tile_size,
            texture_atlas_handle,
        }
    }

    #[inline(always)]
    pub fn texture_atlas_handle(&self) -> Handle<TextureAtlas> {
        self.texture_atlas_handle.clone()
    }

    #[inline(always)]
    pub fn tile_size(&self) -> Vec2 { self.tile_size }

    #[inline]
    pub fn build_bundle(&self, x: f32, y: f32) -> TileBundle {
        let mut sprite = TextureAtlasSprite::new(Tile::Default.index());
        sprite.anchor = Anchor::BottomLeft;

        TileBundle {
            tile: Tile::Default,
            sprite: SpriteSheetBundle {
                sprite,
                texture_atlas: self.texture_atlas_handle(),
                transform: Transform::from_xyz(x, y, 0.),
                ..Default::default()
            },
        }
    }
}

