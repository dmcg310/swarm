use macroquad::prelude::*;

struct Particle {
    pos: Vec2,
    vel: Vec2,
    radius: f32,
}

impl Particle {
    fn new(pos: Vec2, vel: Vec2, radius: f32) -> Self {
        Self { pos, vel, radius }
    }
}

const N: usize = 100;

#[macroquad::main(window_conf)]
async fn main() {
    let mut particles = create_particles(N);

    loop {
        input();
        update(&mut particles);
        render(&particles);

        next_frame().await
    }
}

fn input() {}

fn update(particles: &mut Vec<Particle>) {
    // O(n) update of all particles
    for p in particles.iter_mut() {
        p.pos += p.vel;

        // Simulate "bouncing" off the walls by inverting the velocity when hitting a wall
        if p.pos.x < 0.0 || p.pos.x > screen_width() {
            p.vel.x *= -1.0;
        }
        if p.pos.y < 0.0 || p.pos.y > screen_height() {
            p.vel.y *= -1.0;
        }
    }
}

fn render(particles: &Vec<Particle>) {
    clear_background(DARKGRAY);

    for p in particles.iter() {
        draw_circle(p.pos.x, p.pos.y, p.radius, WHITE);
    }

    draw_text(
        &format!(
            "fps: {:.2} - frame_time: {:.4}s - particles: {}",
            macroquad::time::get_fps(),
            macroquad::time::get_frame_time(),
            N
        ),
        20.0,
        30.0,
        30.0,
        WHITE,
    );
}

fn create_particles(n: usize) -> Vec<Particle> {
    // perf: we know the size of the vec
    let mut particles = Vec::with_capacity(n);
    for _ in 0..n {
        particles.push(Particle::new(
            get_random_pos(),
            get_random_vel(),
            get_random_radius(),
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

fn get_random_radius() -> f32 {
    let low = 5.0;
    let high = 15.0;
    rand::gen_range(low, high)
}

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("swarm"),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}
