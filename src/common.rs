use macroquad::prelude::*;

/// Returns a random position within the screen bounds.
pub fn get_random_pos() -> Vec2 {
    let low = 0.0;

    vec2(
        rand::gen_range(low, screen_width()),
        rand::gen_range(low, screen_height()),
    )
}

/// Returns a random velocity vector with components in the range [-1.0, 1.0].
pub fn get_random_vel() -> Vec2 {
    let low = -1.0;
    let high = 1.0;

    vec2(rand::gen_range(low, high), rand::gen_range(low, high))
}

/// Returns a random color with RGB components in the range [0.0, 1.0] and alpha set to 1.0.
pub fn get_random_color() -> Color {
    let low = 0.0;
    let high = 1.0;
    let alpha = 1.0;

    Color::new(
        rand::gen_range(low, high),
        rand::gen_range(low, high),
        rand::gen_range(low, high),
        alpha,
    )
}
