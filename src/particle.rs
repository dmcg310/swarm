use macroquad::prelude::*;

/// A particle is a simple struct that has a position, velocity, radius, and color. It can update
/// its position based on its velocity and draw itself to the screen.
pub struct Particle {
    pos: Vec2,
    vel: Vec2,
    radius: f32,
    color: Color,
}

/// Implement methods for the Particle struct.
impl Particle {
    /// Creates a new Particle with the given position, velocity, radius, and color.
    pub fn new(pos: Vec2, vel: Vec2, radius: f32, color: Color) -> Self {
        Self {
            pos,
            vel,
            radius,
            color,
        }
    }

    /// Returns the position of the particle.
    pub fn pos(&self) -> Vec2 {
        self.pos
    }

    /// Returns the velocity of the particle.
    pub fn vel(&self) -> Vec2 {
        self.vel
    }

    /// Sets the velocity of the particle.
    pub fn set_vel(&mut self, vel: Vec2) {
        self.vel = vel;
    }

    /// Returns the radius of the particle.
    pub fn radius(&self) -> f32 {
        self.radius
    }

    /// Updates the position of the particle based on its velocity and simulates bouncing off the
    /// walls.
    pub fn update(&mut self) {
        self.pos += self.vel;

        // Simulate "bouncing" off the walls by inverting the velocity when hitting a wall.
        if self.pos.x < 0.0 || self.pos.x > screen_width() {
            self.vel.x *= -1.0;
        }
        if self.pos.y < 0.0 || self.pos.y > screen_height() {
            self.vel.y *= -1.0;
        }
    }

    /// Draws the particle as a circle on the screen.
    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, self.color);
    }
}
