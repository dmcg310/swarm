use macroquad::prelude::Conf;

/// Window title.
const TITLE: &str = "swarm";

/// Window width.
const WIDTH: i32 = 1280;

/// Window height.
const HEIGHT: i32 = 720;

/// Create a window configuration for macroquad.
pub fn create_window() -> Conf {
    Conf {
        window_title: TITLE.to_string(),
        window_width: WIDTH,
        window_height: HEIGHT,
        ..Default::default()
    }
}
