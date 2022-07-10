pub(crate) use grid::*;
pub(crate) use tile::*;

mod grid;
mod tile;

#[derive(Clone, Copy, Debug)]
pub(crate) struct GridSize(usize, usize);

impl GridSize {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self { 0: cols, 1: rows }
    }

    #[inline(always)]
    pub fn columns(&self) -> usize { self.0 }

    #[inline(always)]
    pub fn rows(&self) -> usize { self.1 }

    #[inline(always)]
    pub fn capacity(&self) -> usize { self.0 * self.1 }

    #[inline(always)]
    pub fn index_of(&self, col: usize, row: usize) -> usize {
        col + (row * self.columns())
    }
}
