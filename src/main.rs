use macroquad::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use sysinfo::{ProcessesToUpdate, System, get_current_pid};

struct Particle {
    pos: Vec2,
    vel: Vec2,
    radius: f32,
    color: Color,
}

impl Particle {
    fn new(pos: Vec2, vel: Vec2, radius: f32, color: Color) -> Self {
        Self {
            pos,
            vel,
            radius,
            color,
        }
    }
}

const N: usize = 2_000;

#[macroquad::main(window_conf)]
async fn main() {
    let metrics = spawn_metrics_monitor();

    let mut particles = create_particles(N);

    loop {
        input();

        let t0 = Instant::now();
        update(&mut particles);
        let update_ms = t0.elapsed().as_secs_f64() * 1000.0;

        let t1 = Instant::now();
        render(&particles);
        let render_ms = t1.elapsed().as_secs_f64() * 1000.0;

        // draw_metrics is deliberately outside both timers above,
        // so its own cost isn't counted as update/render time
        draw_metrics(&metrics, update_ms, render_ms);

        next_frame().await
    }
}

// -- Main loop functions

fn input() {
    if is_key_pressed(KeyCode::Q) {
        std::process::exit(0);
    }
}

fn update(particles: &mut Vec<Particle>) {
    // O(n^2) particle collision detection

    for p in particles.iter_mut() {
        update_particle(p);
    }

    // Collisions
    update_particles(particles);
}

fn render(particles: &Vec<Particle>) {
    clear_background(DARKGRAY);

    for p in particles.iter() {
        draw_particle(p);
    }
}

// -- Update functions

fn update_particle(p: &mut Particle) {
    p.pos += p.vel;

    // Simulate "bouncing" off the walls by inverting the velocity when hitting a wall
    if p.pos.x < 0.0 || p.pos.x > screen_width() {
        p.vel.x *= -1.0;
    }
    if p.pos.y < 0.0 || p.pos.y > screen_height() {
        p.vel.y *= -1.0;
    }
}

fn update_particles(particles: &mut Vec<Particle>) {
    let n = particles.len();

    for i in 0..n {
        for j in (i + 1)..n {
            // If the distance between the two particles is less than the sum of their radii, they
            // are colliding. Imagine two circles. If the distance between their centers is less
            // than the sum of their radii, they must be overlapping.

            let dist = particles[i].pos.distance(particles[j].pos);

            if dist < particles[i].radius + particles[j].radius {
                // Swap velocities
                let v1 = particles[i].vel;
                let v2 = particles[j].vel;
                particles[i].vel = v2;
                particles[j].vel = v1;
            }
        }
    }
}

// -- Draw functions

fn draw_particle(p: &Particle) {
    // A particle is just a cricle at the end of the day
    draw_circle(p.pos.x, p.pos.y, p.radius, p.color);
}

fn draw_metrics(metrics: &Arc<Mutex<Metrics>>, update_ms: f64, render_ms: f64) {
    let m = *metrics.lock().unwrap();

    let lines = [
        format!(
            "fps: {:.2}  particles: {}  frame_time: {:.4}s",
            macroquad::time::get_fps(),
            N,
            macroquad::time::get_frame_time()
        ),
        format!("update: {:.3}ms  render: {:.3}ms", update_ms, render_ms),
        format!("cpu: {:.1}%  mem: {:.1}MB", m.cpu_pct, m.mem_mb),
    ];

    draw_rectangle(
        10.0,
        10.0,
        635.0,
        lines.len() as f32 * 25.0 + 15.0,
        Color::new(0.0, 0.0, 0.0, 0.6),
    );

    for (i, line) in lines.iter().enumerate() {
        draw_text(line, 20.0, 30.0 + i as f32 * 25.0, 30.0, YELLOW);
    }
}

// -- Helper functions

fn create_particles(n: usize) -> Vec<Particle> {
    let radius = 2.0;

    // Perf: we know the size of the vec
    let mut particles = Vec::with_capacity(n);
    for _ in 0..n {
        particles.push(Particle::new(
            get_random_pos(),
            get_random_vel(),
            radius,
            get_random_color(),
        ));
    }

    particles
}

fn get_random_pos() -> Vec2 {
    let low = 0.0;
    vec2(
        rand::gen_range(low, screen_width()),
        rand::gen_range(low, screen_height()),
    )
}

fn get_random_vel() -> Vec2 {
    let low = -1.0;
    let high = 1.0;
    vec2(rand::gen_range(low, high), rand::gen_range(low, high))
}

fn get_random_color() -> Color {
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

// -- Metrics

#[derive(Clone, Copy, Default)]
struct Metrics {
    cpu_pct: f32,
    mem_mb: f32,
}

fn spawn_metrics_monitor() -> Arc<Mutex<Metrics>> {
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

// -- Window configuration

fn window_conf() -> Conf {
    let title = String::from("swarm");
    let width = 1280;
    let height = 720;
    Conf {
        window_title: title,
        window_width: width,
        window_height: height,
        ..Default::default()
    }
}
