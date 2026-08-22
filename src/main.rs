use macroquad::prelude::*;

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

const N: usize = 10_000;

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

// -- Main loop functions

fn input() {}

fn update(particles: &mut Vec<Particle>) {
    // O(n) update of all particles
    for p in particles.iter_mut() {
        update_particle(p);
    }
}

fn render(particles: &Vec<Particle>) {
    clear_background(DARKGRAY);

    for p in particles.iter() {
        draw_particle(p);
    }

    draw_metrics();
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

// -- Draw functions

fn draw_particle(p: &Particle) {
    // A particle is just a cricle at the end of the day
    draw_circle(p.pos.x, p.pos.y, p.radius, p.color);
}

fn draw_metrics() {
    draw_text(
        &format!(
            "fps: {:.2}, frame_time: {:.4}s, particles: {}",
            macroquad::time::get_fps(),
            macroquad::time::get_frame_time(),
            N
        ),
        20.0,
        30.0,
        30.0,
        YELLOW,
    );
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
