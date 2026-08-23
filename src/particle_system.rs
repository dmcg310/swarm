use crate::common;
use crate::particle::Particle;

/// Number of particles to simulate.
pub const N: usize = 2_000;

/// A particle system is a collection of particles that can be updated and drawn to the screen.
pub struct ParticleSystem {
    particles: Vec<Particle>,
}

/// Implement methods for the ParticleSystem struct.
impl ParticleSystem {
    /// Creates a new ParticleSystem with the given number of particles.
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

        Self { particles }
    }

    /// Updates all particles in the system. This includes updating their positions and handling
    /// collisions with the walls / other particles.
    pub fn update(&mut self) {
        // O(n^2) particle collision detection.

        for p in self.particles.iter_mut() {
            p.update();
        }

        for i in 0..self.particles.len() {
            for j in (i + 1)..self.particles.len() {
                // If the distance between the two particles is less than the sum of their radii, they
                // are colliding. Imagine two circles. If the distance between their centers is less
                // than the sum of their radii, they must be overlapping.

                let dist = self.particles[i].pos().distance(self.particles[j].pos());

                if dist < self.particles[i].radius() + self.particles[j].radius() {
                    // Swap velocities.
                    let v1 = self.particles[i].vel();
                    let v2 = self.particles[j].vel();
                    self.particles[i].set_vel(v2);
                    self.particles[j].set_vel(v1);
                }
            }
        }
    }

    /// Draws all particles in the system to the screen.
    pub fn draw(&self) {
        for p in self.particles.iter() {
            p.draw();
        }
    }
}
