use macroquad::prelude::*;
use core::game_state::{GameEvent, StateMachine};
use core::player::{Player, PlayerInput, Vec2};
use core::cave::Cave;

/// Window configuration constants
const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
const WINDOW_TITLE: &str = "Fuel Drift";

/// Game constants
const SCROLL_SPEED: f32 = 120.0; // pixels/second
const PLAYER_SIZE: (f32, f32) = (30.0, 18.0);

/// Game state container following Single Responsibility Principle
struct GameWorld {
    state_machine: StateMachine,
    player: Player,
    cave: Cave,
    camera_offset_x: f32,
}

impl GameWorld {
    fn new() -> Self {
        Self {
            state_machine: StateMachine::new(),
            player: Player::new(Vec2::new(100.0, 300.0)),
            cave: Cave::new(42), // Fixed seed for consistent cave
            camera_offset_x: 0.0,
        }
    }
}

/// Window configuration following Single Responsibility Principle
fn window_conf() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

/// Handles input for state transitions with low cyclomatic complexity
fn handle_state_input(state_machine: &mut StateMachine) {
    let current_state = state_machine.current();

    match current_state {
        core::game_state::GameState::Menu => {
            if is_key_pressed(KeyCode::Enter) {
                state_machine.handle_event(GameEvent::Start);
            }
        }
        core::game_state::GameState::Playing => {
            if is_key_pressed(KeyCode::P) || is_key_pressed(KeyCode::Escape) {
                state_machine.handle_event(GameEvent::PauseToggle);
            }
        }
        core::game_state::GameState::Paused => {
            if is_key_pressed(KeyCode::P) || is_key_pressed(KeyCode::Escape) {
                state_machine.handle_event(GameEvent::PauseToggle);
            }
        }
        core::game_state::GameState::GameOver => {
            if is_key_pressed(KeyCode::Enter) {
                state_machine.handle_event(GameEvent::Start);
            }
            if is_key_pressed(KeyCode::R) {
                state_machine.handle_event(GameEvent::Reset);
            }
        }
    }
}

/// Collects player input for physics simulation
fn collect_player_input() -> PlayerInput {
    PlayerInput {
        up: is_key_down(KeyCode::Up),
        down: is_key_down(KeyCode::Down),
        left: is_key_down(KeyCode::Left),
        right: is_key_down(KeyCode::Right),
    }
}

/// Updates game world physics
fn update_game_world(world: &mut GameWorld, dt: f32) {
    match world.state_machine.current() {
        core::game_state::GameState::Playing => {
            // Update camera scroll
            world.camera_offset_x += SCROLL_SPEED * dt;

            // Update player physics
            let input = collect_player_input();
            world.player.tick(dt, input);
        }
        _ => {
            // No physics updates in other states
        }
    }
}

/// Renders the cave segments
fn render_cave(cave: &mut Cave, camera_offset_x: f32) {
    let view_start = camera_offset_x;
    let view_end = camera_offset_x + WINDOW_WIDTH as f32;

    let segments = cave.segments_in_view(view_start, view_end);
    for segment in segments {
        let screen_x = segment.x_start - camera_offset_x;

        // Draw ceiling (black rectangle from top to ceiling height)
        draw_rectangle(
            screen_x,
            0.0,
            segment.width,
            segment.ceiling,
            BLACK,
        );

        // Draw floor (black rectangle from floor height to bottom)
        draw_rectangle(
            screen_x,
            segment.floor,
            segment.width,
            WINDOW_HEIGHT as f32 - segment.floor,
            BLACK,
        );
    }
}

/// Renders the player
fn render_player(player: &Player, camera_offset_x: f32) {
    let screen_x = player.pos.x - camera_offset_x;
    let screen_y = player.pos.y;

    draw_rectangle(
        screen_x - PLAYER_SIZE.0 / 2.0,
        screen_y - PLAYER_SIZE.1 / 2.0,
        PLAYER_SIZE.0,
        PLAYER_SIZE.1,
        RED,
    );
}

/// Renders UI overlays based on game state
fn render_ui(state_machine: &StateMachine) {
    let current_state = state_machine.current();

    match current_state {
        core::game_state::GameState::Menu => {
            draw_text(
                "FUEL DRIFT",
                WINDOW_WIDTH as f32 / 2.0 - 80.0,
                WINDOW_HEIGHT as f32 / 2.0 - 50.0,
                40.0,
                WHITE,
            );
            draw_text(
                "Press ENTER to start",
                WINDOW_WIDTH as f32 / 2.0 - 90.0,
                WINDOW_HEIGHT as f32 / 2.0,
                20.0,
                GRAY,
            );
        }
        core::game_state::GameState::Playing => {
            draw_text("Playing", 10.0, 30.0, 20.0, WHITE);
        }
        core::game_state::GameState::Paused => {
            // Semi-transparent overlay
            draw_rectangle(
                0.0,
                0.0,
                WINDOW_WIDTH as f32,
                WINDOW_HEIGHT as f32,
                Color::new(0.0, 0.0, 0.0, 0.5),
            );

            draw_text(
                "PAUSED",
                WINDOW_WIDTH as f32 / 2.0 - 50.0,
                WINDOW_HEIGHT as f32 / 2.0,
                30.0,
                WHITE,
            );
            draw_text(
                "Press P or ESC to resume",
                WINDOW_WIDTH as f32 / 2.0 - 100.0,
                WINDOW_HEIGHT as f32 / 2.0 + 40.0,
                16.0,
                GRAY,
            );
        }
        core::game_state::GameState::GameOver => {
            draw_text(
                "GAME OVER",
                WINDOW_WIDTH as f32 / 2.0 - 70.0,
                WINDOW_HEIGHT as f32 / 2.0 - 20.0,
                30.0,
                RED,
            );
            draw_text(
                "ENTER to restart, R to menu",
                WINDOW_WIDTH as f32 / 2.0 - 110.0,
                WINDOW_HEIGHT as f32 / 2.0 + 20.0,
                16.0,
                GRAY,
            );
        }
    }
}

/// Main game loop with clear separation of concerns
#[macroquad::main(window_conf)]
async fn main() {
    let mut world = GameWorld::new();

    loop {
        let dt = get_frame_time();

        // Handle input
        handle_state_input(&mut world.state_machine);

        // Update game world
        update_game_world(&mut world, dt);

        // Render
        clear_background(DARKBLUE);

        // Render cave only during gameplay states
        match world.state_machine.current() {
            core::game_state::GameState::Playing | core::game_state::GameState::Paused => {
                render_cave(&mut world.cave, world.camera_offset_x);
                render_player(&world.player, world.camera_offset_x);
            }
            _ => {}
        }

        // Render UI
        render_ui(&world.state_machine);

        next_frame().await;
    }
}