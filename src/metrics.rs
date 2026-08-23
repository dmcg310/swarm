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
    /// Returns the CPU usage percentage.
    pub fn cpu_pct(&self) -> f32 {
        self.cpu_pct
    }

    /// Returns the memory usage in megabytes.
    pub fn mem_mb(&self) -> f32 {
        self.mem_mb
    }
}

/// Spawns a background thread to monitor CPU and memory usage metrics.
/// Returns an `Arc<Mutex<Metrics>>` that can be shared across threads.
pub fn spawn_metrics_monitor() -> Arc<Mutex<Metrics>> {
    let shared = Arc::new(Mutex::new(Metrics::default()));
    let shared_clone = shared.clone();

    thread::spawn(move || {
        let pid = get_current_pid().expect("failed to get current pid");

        let mut sys = System::new();
        sys.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);

        thread::sleep(Duration::from_millis(200));

        loop {
            sys.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);

            if let Some(process) = sys.process(pid) {
                *shared_clone.lock().unwrap() = Metrics {
                    cpu_pct: process.cpu_usage(),
                    mem_mb: process.memory() as f32 / 1024.0 / 1024.0,
                };
            }

            thread::sleep(Duration::from_millis(500)); // 1/2 second
        }
    });

    shared
}
