use macroquad::prelude::Vec2;
use std::collections::HashMap;

use crate::cell::Cell;

/// A spatial hash grid for efficient collision detection. The grid is divided into cells of a fixed
/// size, and each cell contains a list of indices of particles that are located within that cell.
/// It can be visualized as a 2D grid where each cell is identified by its (x, y) coordinates. For
/// example:
///
/// cells: {
///     coordinates: (0, 0), Cell { particle_indices: [0, 1, 2] },
///     coordinates: (0, 1), Cell { particle_indices: [3, 4] },
///     coordinates: (1, 0), Cell { particle_indices: [5] },
/// }
///
/// The speed comes from the fact that we can quickly find which cell a particle belongs to based on
/// its position, and then only check for collisions with particles in the same cell or neighboring
/// cells, rather than checking against all particles in the system.
pub struct Grid {
    cells: HashMap<(i32, i32), Cell>,
    cell_size: f32,
}

/// Implement methods for the Grid struct.
impl Grid {
    /// Creates a new spatial hash grid with the specified cell size.
    pub fn new(cell_size: f32) -> Self {
        Self {
            cells: HashMap::new(),
            cell_size,
        }
    }

    /// Clears all cells in the grid, effectively removing all particles from the grid. This is
    /// typically called at the beginning of each update cycle to prepare for re-adding particles based
    /// on their new positions.
    pub fn clear(&mut self) {
        for cell in self.cells.values_mut() {
            cell.clear();
        }
    }

    /// Calculates the cell coordinates for a given position. The position is divided by the cell
    /// size and floored to get the integer coordinates of the cell that contains the position.
    pub fn cell_coords(&self, pos: Vec2) -> (i32, i32) {
        let x = (pos.x / self.cell_size).floor() as i32;
        let y = (pos.y / self.cell_size).floor() as i32;

        (x, y)
    }

    /// Retrieves the list of particle indices in the cell at the specified coordinates. Returns an
    /// empty slice if the cell does not exist.
    pub fn particles_in_cell(&self, coords: (i32, i32)) -> &[usize] {
        self.cells
            .get(&coords)
            .map(|cell| cell.particle_indices())
            .unwrap_or(&[])
    }

    /// Returns only the "forward half" of the neighborhood (4 of the 8 neighbors: east, south-west,
    /// south, south-east) plus the cell's own contents are handled separately by the caller. Pairing
    /// this with a walk over every occupied cell means each pair of particles in neighboring cells
    /// is examined exactly once with no need to de-duplicate pairs afterward. Particles within the same
    /// cell still need to be checked against each other separately (e.g. all i < j within
    /// `particles_in_cell`).
    pub fn forward_neighboring_cells(&self, coords: (i32, i32)) -> [(i32, i32); 4] {
        let (x, y) = coords;

        [(x + 1, y), (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)]
    }

    /// Adds a particle to the grid by calculating its cell coordinates and adding its index to the
    /// corresponding cell. If the cell does not exist, it is created.
    pub fn add_particle(&mut self, pos: Vec2, particle_index: usize) {
        let coords = self.cell_coords(pos);

        self.cells
            .entry(coords)
            .or_insert_with(Cell::new)
            .add_particle(particle_index);
    }
}
