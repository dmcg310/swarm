use macroquad::prelude::*;

/// A struct that holds the data for a collection of particles. It uses separate vectors for
/// positions, velocities, radii, and colors to improve cache locality and performance.
pub struct ParticleData {
    positions: Vec<Vec2>,
    velocities: Vec<Vec2>,
    radii: Vec<f32>,
    colors: Vec<Color>,
}

/// Implement methods for the ParticleData struct.
impl ParticleData {
    /// Creates a new ParticleData with the given capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            positions: Vec::with_capacity(capacity),
            velocities: Vec::with_capacity(capacity),
            radii: Vec::with_capacity(capacity),
            colors: Vec::with_capacity(capacity),
        }
    }

    /// Adds a new particle to the ParticleData.
    pub fn push(&mut self, pos: Vec2, vel: Vec2, radius: f32, color: Color) {
        self.positions.push(pos);
        self.velocities.push(vel);
        self.radii.push(radius);
        self.colors.push(color);
    }

    /// Returns the number of particles stored in the ParticleData.
    pub fn len(&self) -> usize {
        // Can be any of the vectors since they are all the same length.
        self.positions.len()
    }

    /// Returns the position of the particle at the given index.
    pub fn position(&self, index: usize) -> Vec2 {
        self.positions[index]
    }

    /// Returns the radius of the particle at the given index.
    pub fn radius(&self, index: usize) -> f32 {
        self.radii[index]
    }

    /// Returns the velocity of the particle at the given index.
    pub fn velocity(&self, index: usize) -> Vec2 {
        self.velocities[index]
    }

    /// Sets the velocity of the particle at the given index.
    pub fn set_velocity(&mut self, index: usize, vel: Vec2) {
        self.velocities[index] = vel;
    }

    /// Returns an iterator over the positions of all particles.
    pub fn positions_iter(&self) -> impl Iterator<Item = &Vec2> {
        self.positions.iter()
    }

    /// Updates the positions of all particles based on their velocities and simulates bouncing off
    /// the walls.
    pub fn update(&mut self) {
        for i in 0..self.len() {
            self.positions[i] += self.velocities[i];

            if self.positions[i].x < 0.0 || self.positions[i].x > screen_width() {
                self.velocities[i].x *= -1.0;
            }
            if self.positions[i].y < 0.0 || self.positions[i].y > screen_height() {
                self.velocities[i].y *= -1.0;
            }
        }
    }

    /// Draws all particles as circles on the screen.
    pub fn draw(&self) {
        for i in 0..self.len() {
            let pos = self.positions[i];
            let radius = self.radii[i];
            let color = self.colors[i];

            draw_circle(pos.x, pos.y, radius, color);
        }
    }
}
