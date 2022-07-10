use bevy::prelude::{Image, IVec2, TextureAtlas};
use bevy::sprite::{Rect, SpriteSheetBundle, TextureAtlasSprite};
use winit::dpi::PhysicalPosition;

use crate::{Handle, Vec2};

pub fn center_window(window: &winit::window::Window) -> Option<IVec2> {
    let monitor = window.current_monitor()?;
    let monitor_size = monitor.size();
    let monitor_position = monitor.position();
    let window_size = window.outer_size();

    let position = IVec2::new(
        monitor_position.x + ((monitor_size.width - window_size.width) as i32 / 2),
        monitor_position.y + ((monitor_size.height - window_size.height) as i32 / 2),
    );
    window.set_outer_position(PhysicalPosition::new(position.x, position.y));

    return Some(position);
}

pub struct TextureAtlasSlicer<T: Copy> {
    rects: Vec<(T, Rect)>,
}

impl<T: Copy> TextureAtlasSlicer<T> {
    pub fn new() -> Self {
        Self {
            rects: Vec::new()
        }
    }

    pub fn add(&mut self, typ: T, rect: Rect) {
        self.rects.push((typ, rect));
    }

    pub fn slice(&self, img: Handle<Image>) -> (TextureAtlas, Vec<(T, usize)>) {
        let mut atlas = TextureAtlas::new_empty(img, Vec2::ZERO);

        let mut indexes = Vec::with_capacity(self.rects.len());
        for (typ, rect) in self.rects.iter() {
            indexes.push((*typ, atlas.add_texture(*rect)));
        }

        (atlas, indexes)
    }
}

pub struct SpriteSheetBundleBuilder<T: PartialEq> {
    texture_atlas: Handle<TextureAtlas>,
    indexes: Vec<(T, usize)>,
}

impl<T: PartialEq> SpriteSheetBundleBuilder<T> {
    pub fn new(texture_atlas: Handle<TextureAtlas>, indexes: Vec<(T, usize)>) -> Self {
        Self {
            texture_atlas,
            indexes,
        }
    }

    pub fn get(&self, typ: T) -> Option<SpriteSheetBundle> {
        for (t, index) in &self.indexes {
            if typ == *t {
                return Some(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(*index),
                    texture_atlas: self.texture_atlas.clone(),
                    ..Default::default()
                });
            }
        }
        None
    }
}