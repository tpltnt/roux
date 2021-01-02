//! `roux` provides data-structures and functions
//! to implement Cellular Automata.
//#![no_std]
use buddy_alloc::{BuddyAllocParam, FastAllocParam, NonThreadsafeAlloc};

const FAST_HEAP_SIZE: usize = 32; // 32B
const HEAP_SIZE: usize = 10 * 1024; // 10KB
const LEAF_SIZE: usize = 16;

/// The state of a cell.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CellState {
    Dead,
    Alive,
}

/// A structure to encode a grid with cells.
/// Cell positions start at top left corner.
pub struct Grid {
    // size allows for 256x256 cells -> enough for embedded
    // -> for more adjust the data types
    horizontal_size: u8,
    vertical_size: u8,
    cells: Vec<Vec<CellState>>,
}

impl Grid {
    /// Create a new grid with the given dimensions and
    /// fill it with default (dead) cells.
    ///
    /// # Arguments
    /// * `h_size`: horizontal dimension/size as number of cells
    /// * `v_size`: vertical dimension/size as number of cells
    ///
    /// # Remarks
    ///
    /// `u8` was chosen to stay below `usize::MAX` for a `u8` x `u8`
    /// grid. 256x256 are currently enough cells for embedded applications.
    /// Larger grid sizes have to keep the target usize (thus architecture)
    /// in mind and can be adjusted appropriately.
    pub fn new(h_size: u8, v_size: u8) -> Grid {
        Grid {
            horizontal_size: h_size,
            vertical_size: v_size,
            cells: vec![vec![CellState::Dead; v_size as usize]; h_size as usize],
        }
    }

    /// Retrieve a cell state (for modification).
    ///
    /// # Arguments
    /// * `h`: horizontal coordinate
    /// * `v`: vertical coordinate
    pub fn get_cellstate(&self, h: u8, v: u8) -> CellState {
        if h >= self.horizontal_size {
            panic!("horizontal coordinate too large")
        }
        if v >= self.vertical_size {
            panic!("vertical coordinate too large")
        }
        return self.cells[h as usize][v as usize];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // check grid creation values
    fn grid_new() {
        let g = Grid::new(5, 23);
        assert_eq!(g.horizontal_size, 5);
        assert_eq!(g.vertical_size, 23);
    }

    #[test]
    // check grid creation values
    fn grid_get_cellstate() {
        let g = Grid::new(3, 17);
        let c = g.get_cellstate(1, 8);
        assert_eq!(c, CellState::Dead);
    }

    #[test]
    #[should_panic]
    fn grid_get_cell_v_too_large() {
        let g = Grid::new(3, 17);
        let _c = g.get_cellstate(1, 17);
    }

    #[test]
    #[should_panic]
    fn grid_get_cell_h_too_large() {
        let g = Grid::new(3, 1);
        let _c = g.get_cellstate(3, 0);
    }
}
