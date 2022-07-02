pub use game::*;
pub use ui::*;

pub mod grid;
mod game;
mod ui;

/// Resource
#[allow(dead_code)]
pub(crate) enum Difficulty {
    Beginner,
    Intermediate,
    Expert,
    Custom {
        size: grid::GridSize,
        mines: usize,
    },
}

impl Difficulty {
    #[inline]
    pub fn size(&self) -> grid::GridSize {
        use Difficulty::*;
        return match self {
            Beginner => grid::GridSize::new(8, 8),
            Intermediate => grid::GridSize::new(16, 16),
            Expert => grid::GridSize::new(32, 16),
            Custom { size, mines: _ } => *size,
        };
    }

    #[inline]
    pub fn mines(&self) -> usize {
        use Difficulty::*;
        return match self {
            Beginner => 10,
            Intermediate => 40,
            Expert => 100,
            Custom { size: _, mines } => *mines,
        };
    }
}

impl Default for Difficulty {
    fn default() -> Self { Difficulty::Beginner }
}
