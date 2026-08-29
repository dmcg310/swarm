use std::collections::HashSet;

use crate::common;
use crate::grid::Grid;
use crate::particle::Particle;

/// Number of particles to simulate.
pub const N: usize = 5_000;

/// A particle system is a collection of particles that can be updated and drawn to the screen.
pub struct ParticleSystem {
    particles: Vec<Particle>,
    pairs: Vec<(usize, usize)>,
    grid: Grid, // spatial hash grid
}

/// Implement methods for the ParticleSystem struct.
impl ParticleSystem {
    /// Creates a new ParticleSystem with the defined number of particles.
    pub fn new() -> Self {
        let radius = 2.0;

        let mut particles = Vec::with_capacity(N);
        for _ in 0..N {
            particles.push(Particle::new(
                common::get_random_pos(),
                common::get_random_vel(),
                radius,
                common::get_random_color(),
            ));
        }

        let pairs = Vec::new();
        let grid = Grid::new(radius * 2.0); // cell size is 2x the particle radius

        Self {
            particles,
            pairs,
            grid,
        }
    }

    /// Resolves a collision between two particles by swapping their velocities.
    pub fn resolve_pair(&mut self, a: usize, b: usize) {
        // Get the positions of the two particles and check if they are colliding. Firstly, we
        // compute the squared distance between the two particles and compare it to the squared sum
        // of their radii. If they are colliding, we swap their velocities.

        let (pa, pb) = (self.particles[a].pos(), self.particles[b].pos());

        let dx = pb.x - pa.x;
        let dy = pb.y - pa.y;

        let dist_sq = dx * dx + dy * dy;
        let min_dist = self.particles[a].radius() + self.particles[b].radius();

        if dist_sq < min_dist * min_dist {
            // Swap velocities.
            let v1 = self.particles[a].vel();
            let v2 = self.particles[b].vel();
            self.particles[a].set_vel(v2);
            self.particles[b].set_vel(v1);
        }
    }

    /// Updates all particles in the system. This includes updating their positions and handling
    /// collisions with the walls / other particles.
    pub fn update(&mut self) {
        for p in self.particles.iter_mut() {
            p.update();
        }

        self.pairs.clear();
        self.grid.clear();

        for (i, p) in self.particles.iter().enumerate() {
            self.grid.add_particle(p.pos(), i);
        }

        let occupied_coords: Vec<(i32, i32)> = self
            .particles
            .iter()
            .map(|p| self.grid.cell_coords(p.pos()))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        for &coords in &occupied_coords {
            let here = self.grid.particles_in_cell(coords);

            // a. same-cell pairs
            for i in 0..here.len() {
                for j in (i + 1)..here.len() {
                    self.pairs.push((here[i], here[j]));
                }
            }

            // b. this cell vs the forward half of its neighbors
            for neighbor_coords in self.grid.forward_neighboring_cells(coords) {
                let there = self.grid.particles_in_cell(neighbor_coords);
                for &i in here {
                    for &j in there {
                        self.pairs.push((i, j));
                    }
                }
            }
        }

        for idx in 0..self.pairs.len() {
            let (a, b) = self.pairs[idx];
            self.resolve_pair(a, b);
        }
    }

    /// Draws all particles in the system to the screen.
    pub fn draw(&self) {
        for p in self.particles.iter() {
            p.draw();
        }
    }
}
