use bevy::prelude::*;
pub(crate) use digits::*;
pub(crate) use edge::*;

mod digits;
mod edge;

pub struct Colors {
    pub light: Color,
    pub dark: Color,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            light: Color::WHITE,
            dark: Color::rgb(140. / 255., 140. / 255., 140. / 255.),
        }
    }
}

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiComponent {
    EdgeCorner,
    SmileyButton,
    SmileyDead,
}
