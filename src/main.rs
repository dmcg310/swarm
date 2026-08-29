use macroquad::prelude::*;
use std::time::Instant;

use crate::frame_time_history::FrameTimeHistory;
use crate::particle_system::ParticleSystem;
use crate::window::create_window;

mod cell;
mod common;
mod frame_time_history;
mod grid;
mod metrics;
mod particle;
mod particle_system;
mod window;

/// The main entry point for the application. This function initializes the metrics monitor, creates
/// a new particle system, and enters the main loop where it handles input, updates the particle
/// system, draws the particles, and displays performance metrics.
#[macroquad::main(create_window)]
async fn main() {
    let metrics = metrics::spawn_metrics_monitor();

    let mut particle_system = ParticleSystem::new();
    let mut frame_time_history = FrameTimeHistory::new();

    loop {
        let frame_time_ms = get_frame_time() as f64 * 1000.0;
        frame_time_history.push(frame_time_ms);

        input();

        let t0 = Instant::now();
        update(&mut particle_system);
        let update_ms = t0.elapsed().as_secs_f64() * 1000.0;

        let t1 = Instant::now();
        draw(&particle_system);
        let draw_ms = t1.elapsed().as_secs_f64() * 1000.0;

        // draw_metrics is deliberately outside both timers above,
        // so its own cost isn't counted as update/draw time.
        metrics
            .lock()
            .unwrap()
            .draw(update_ms, draw_ms, &frame_time_history);

        next_frame().await
    }
}

/// Handles user input. If the 'Q' key is pressed, the application will exit.
fn input() {
    if is_key_pressed(KeyCode::Q) {
        std::process::exit(0);
    }
}

/// Updates the particle system by calling its update method.
fn update(particle_system: &mut ParticleSystem) {
    particle_system.update();
}

/// Draws the particle system to the screen.
fn draw(particle_system: &ParticleSystem) {
    clear_background(DARKGRAY);

    particle_system.draw();
}
