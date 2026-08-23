use macroquad::prelude::Conf;

/// Returns a `Conf` struct with the window configuration for the application.
pub fn window_conf() -> Conf {
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
