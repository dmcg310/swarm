use macroquad::prelude::*;

#[macroquad::main("swarm")]
async fn main() {
    loop {
        input();
        update();
        render();

        next_frame().await
    }
}

fn input() {}

fn update() {}

fn render() {
    clear_background(DARKGRAY);

    draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, BLUE);
    draw_text(
        &format!(
            "fps: {:.2} - frame_time: {:.4}s",
            macroquad::time::get_fps(),
            macroquad::time::get_frame_time()
        ),
        20.0,
        30.0,
        30.0,
        WHITE,
    );
}
