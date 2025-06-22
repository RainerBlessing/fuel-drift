use macroquad::prelude::*;

/// Window configuration constants
const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
const WINDOW_TITLE: &str = "Fuel Drift";

/// Window configuration following Single Responsibility Principle
fn window_conf() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

/// Handles user input with low cyclomatic complexity
fn handle_input() -> bool {
    is_key_pressed(KeyCode::Escape)
}

/// Main game loop with clear separation of concerns
#[macroquad::main(window_conf)]
async fn main() {
    loop {
        if handle_input() {
            break;
        }

        clear_background(BLACK);
        next_frame().await;
    }
}