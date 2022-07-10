use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::utils::SpriteSheetBundleBuilder;

use super::*;

// Resource
pub type EdgeSize = f32;

// Resource
pub type EdgePadding = f32;

#[derive(Component)]
pub(crate) enum EdgeSide {
    Top,
    Left,
    Bottom,
    Right,
}

impl EdgeSide {
    fn index(&self) -> usize {
        match self {
            Self::Top => { 0 }
            Self::Left => { 1 }
            Self::Bottom => { 2 }
            Self::Right => { 3 }
        }
    }
}

#[derive(Component)]
pub(crate) enum EdgeCorner {
    BottomLeft,
    TopRight,
}

impl EdgeCorner {
    fn index(&self) -> usize {
        match self {
            Self::BottomLeft => { 0 }
            Self::TopRight => { 1 }
        }
    }
}

#[derive(Component)]
pub(crate) struct Edge {
    sides: [Entity; 4],
    corners: [Entity; 2],
}

impl Edge {
    #[inline]
    pub fn spawn(cmd: &mut Commands, colors: &Colors, ui_sprites: &SpriteSheetBundleBuilder<UiComponent>) -> Entity {
        let top = cmd.spawn()
            .insert(EdgeSide::Top)
            .insert_bundle(side_sprite(colors.light, Anchor::TopLeft))
            .id();
        let left = cmd.spawn()
            .insert(EdgeSide::Left)
            .insert_bundle(side_sprite(colors.light, Anchor::BottomLeft))
            .id();
        let bottom = cmd.spawn()
            .insert(EdgeSide::Bottom)
            .insert_bundle(side_sprite(colors.dark, Anchor::BottomLeft))
            .id();
        let right = cmd.spawn()
            .insert(EdgeSide::Right)
            .insert_bundle(side_sprite(colors.dark, Anchor::BottomRight))
            .id();

        let mut sprite = ui_sprites.get(UiComponent::EdgeCorner).unwrap();
        sprite.sprite.anchor = Anchor::BottomLeft;
        let bottom_left = cmd.spawn_bundle(sprite)
            .insert(EdgeCorner::BottomLeft)
            .insert_bundle(TransformBundle::default())
            .id();

        let mut sprite = ui_sprites.get(UiComponent::EdgeCorner).unwrap();
        sprite.sprite.anchor = Anchor::TopRight;
        let top_right = cmd.spawn_bundle(sprite)
            .insert(EdgeCorner::TopRight)
            .insert_bundle(TransformBundle::default())
            .id();

        cmd.spawn()
            .insert(Self {
                sides: [top, left, bottom, right],
                corners: [bottom_left, top_right],
            })
            .id()
    }
}

#[inline(always)]
fn side_sprite(color: Color, anchor: Anchor) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color,
            anchor,
            ..default()
        },
        ..default()
    }
}
