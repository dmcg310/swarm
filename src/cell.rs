/// A cell in the spatial hash grid. Each cell contains a list of indices of particles that are
/// located within that cell.
pub struct Cell {
    particle_indices: Vec<usize>,
}

/// Implement methods for the Cell struct.
impl Cell {
    /// Creates a new cell with an empty list of particle indices.
    pub fn new() -> Self {
        Self {
            particle_indices: Vec::new(),
        }
    }

    /// Returns a reference to the list of particle indices in the cell, as a slice.
    pub fn particle_indices(&self) -> &[usize] {
        &self.particle_indices
    }

    /// Adds a particle index to the cell's list of particle indices.
    pub fn add_particle(&mut self, particle_index: usize) {
        self.particle_indices.push(particle_index);
    }

    /// Empties the cell's particle list for reuse on the next frame, without deallocating the
    /// underlying buffer.
    pub fn clear(&mut self) {
        self.particle_indices.clear();
    }
}
