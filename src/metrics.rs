use macroquad::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use sysinfo::{ProcessesToUpdate, System, get_current_pid};

/// A struct to hold CPU and memory usage metrics.
#[derive(Clone, Copy, Default)]
pub struct Metrics {
    cpu_pct: f32,
    mem_mb: f32,
}

/// Implement methods for the Metrics struct.
impl Metrics {
    /// Creates a new Metrics instance with the given CPU and memory usage values.
    pub fn new(cpu_pct: f32, mem_mb: f32) -> Self {
        Self { cpu_pct, mem_mb }
    }

    /// Returns the CPU usage percentage.
    pub fn cpu_pct(&self) -> f32 {
        self.cpu_pct
    }

    /// Returns the memory usage in megabytes.
    pub fn mem_mb(&self) -> f32 {
        self.mem_mb
    }

    /// Draws the metrics on the screen. This includes FPS, number of particles, frame time, update
    /// time, draw time, CPU usage, and memory usage.
    pub fn draw(&self, update_ms: f64, draw_ms: f64) {
        let lines = [
            format!(
                "fps: {:.2}  particles: {}  frame_time: {:.4}s",
                get_fps(),
                crate::particle_system::N,
                get_frame_time()
            ),
            format!("update: {:.3}ms  draw: {:.3}ms", update_ms, draw_ms),
            format!("cpu: {:.1}%  mem: {:.1}MB", self.cpu_pct(), self.mem_mb()),
        ];

        // Draw a semi-transparent black rectangle as the background for the metrics text.
        draw_rectangle(
            10.0,
            10.0,
            635.0,
            lines.len() as f32 * 25.0 + 15.0,
            macroquad::color::Color::new(0.0, 0.0, 0.0, 0.6),
        );

        for (i, line) in lines.iter().enumerate() {
            draw_text(
                line,
                20.0,
                30.0 + i as f32 * 25.0,
                30.0,
                macroquad::color::YELLOW,
            );
        }
    }
}

/// Spawns a background thread to monitor CPU and memory usage metrics.
/// Returns an `Arc<Mutex<Metrics>>` that can be shared across threads.
pub fn spawn_metrics_monitor() -> Arc<Mutex<Metrics>> {
    let shared = Arc::new(Mutex::new(Metrics::default()));
    let shared_clone = shared.clone();

    thread::spawn(move || {
        let remove_dead_processes = true;

        let pid = get_current_pid().expect("failed to get current pid");

        let mut sys = System::new();
        sys.refresh_processes(ProcessesToUpdate::Some(&[pid]), remove_dead_processes);

        // Wait a bit to allow the system to gather initial metrics before entering the loop.
        thread::sleep(Duration::from_millis(200));

        loop {
            sys.refresh_processes(ProcessesToUpdate::Some(&[pid]), remove_dead_processes);

            if let Some(process) = sys.process(pid) {
                *shared_clone.lock().unwrap() = Metrics::new(
                    process.cpu_usage(),
                    process.memory() as f32 / 1024.0 / 1024.0,
                );
            }

            thread::sleep(Duration::from_millis(500)); // 1/2 second.
        }
    });

    shared
}
