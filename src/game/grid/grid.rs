use bevy::prelude::{Commands, Component, Entity, TransformBundle, Vec2};

use super::*;

pub(crate) enum ResizeResult {
    None,
    Grow(usize),
    Shrink(Vec<Entity>),
}

#[derive(Component)]
pub(crate) struct Grid {
    grid_size: GridSize,
    tile_size: Vec2,
    tiles: Vec<Option<Entity>>,
}

impl Grid {
    #[inline]
    pub fn spawn(cmd: &mut Commands, grid_size: GridSize, tile_size: Vec2) -> Entity {
        cmd.spawn_bundle(
            TransformBundle::default())
            .insert(Self {
                grid_size,
                tile_size,
                tiles: vec![None; grid_size.capacity()],
            }).id()
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize { self.grid_size.capacity() }

    #[inline(always)]
    pub fn size(&self) -> GridSize { self.grid_size }

    #[inline]
    pub fn width(&self) -> f32 { self.grid_size.columns() as f32 * self.tile_size.x }

    #[inline]
    pub fn height(&self) -> f32 { self.grid_size.rows() as f32 * self.tile_size.y }

    #[must_use]
    pub(crate) fn resize(&mut self, grid_size: GridSize) -> ResizeResult {
        let have = self.grid_size.capacity();
        let want = grid_size.capacity();
        if have == want {
            return ResizeResult::None;
        }

        self.grid_size = grid_size;
        if have < want {
            self.tiles.resize(want, None);
            return ResizeResult::Grow(want - have);
        }

        let shrink = self.tiles
            .iter()
            .skip(want)
            .filter_map(|x| { *x })
            .collect::<Vec<_>>();

        self.tiles.truncate(self.capacity());
        ResizeResult::Shrink(shrink)
    }
    #[inline]
    pub(crate) fn insert(&mut self, col: usize, row: usize, entity: Entity) {
        self.tiles[self.grid_size.index_of(col, row)] = Some(entity);
    }

    #[inline]
    pub fn get_tile(&self, col: usize, row: usize) -> Option<Entity> {
        if col >= self.grid_size.columns() || row >= self.grid_size.rows() {
            return None;
        }

        let index = self.grid_size.index_of(col, row);
        if index >= self.tiles.len() {
            return None;
        }

        self.tiles[index]
    }

    #[inline]
    pub fn get_tile_xy(&self, x: f32, y: f32) -> Option<Entity> {
        if x < 0. || y < 0. {
            return None;
        }

        self.get_tile(
            f32::floor(x / self.tile_size.x) as usize,
            f32::floor(y / self.tile_size.y) as usize,
        )
    }
}
