pub(crate) use grid::*;
pub(crate) use tile::*;

mod grid;
mod tile;

#[derive(Copy, Clone)]
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
}
