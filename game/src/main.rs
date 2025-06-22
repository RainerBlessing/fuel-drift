use macroquad::prelude::*;
use core::game_state::{GameEvent, StateMachine};
use core::player::{Player, PlayerInput, Vec2};
use core::cave::Cave;
use core::collision::aabb_overlap;

/// Window configuration constants
const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
const WINDOW_TITLE: &str = "Fuel Drift";

/// Game constants
const SCROLL_SPEED: f32 = 120.0; // pixels/second
const PLAYER_SIZE: (f32, f32) = (30.0, 18.0);

/// Collision flash constants
const COLLISION_FLASH_DURATION: f32 = 0.3; // seconds

/// Game state container following Single Responsibility Principle
struct GameWorld {
    state_machine: StateMachine,
    player: Player,
    cave: Cave,
    camera_offset_x: f32,
    collision_flash_timer: f32,
}

impl GameWorld {
    fn new() -> Self {
        Self {
            state_machine: StateMachine::new(),
            player: Player::new(Vec2::new(100.0, 300.0)),
            cave: Cave::new(42), // Fixed seed for consistent cave
            camera_offset_x: 0.0,
            collision_flash_timer: 0.0,
        }
    }

    /// Resets the game world for a new game.
    fn reset(&mut self) {
        self.player = Player::new(Vec2::new(100.0, 300.0));
        self.camera_offset_x = 0.0;
        self.collision_flash_timer = 0.0;
        // Keep the same cave for consistency
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
fn handle_state_input(world: &mut GameWorld) {
    let current_state = world.state_machine.current();

    match current_state {
        core::game_state::GameState::Menu => {
            if is_key_pressed(KeyCode::Enter) {
                world.state_machine.handle_event(GameEvent::Start);
                world.reset();
            }
        }
        core::game_state::GameState::Playing => {
            if is_key_pressed(KeyCode::P) || is_key_pressed(KeyCode::Escape) {
                world.state_machine.handle_event(GameEvent::PauseToggle);
            }
        }
        core::game_state::GameState::Paused => {
            if is_key_pressed(KeyCode::P) || is_key_pressed(KeyCode::Escape) {
                world.state_machine.handle_event(GameEvent::PauseToggle);
            }
            if is_key_pressed(KeyCode::R) {
                world.state_machine.handle_event(GameEvent::Reset);
            }
        }
        core::game_state::GameState::GameOver => {
            if is_key_pressed(KeyCode::Enter) {
                world.state_machine.handle_event(GameEvent::Start);
                world.reset();
            }
            if is_key_pressed(KeyCode::R) {
                world.state_machine.handle_event(GameEvent::Reset);
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

/// Checks for collision between player and cave walls.
///
/// Returns true if collision detected, false otherwise.
fn check_player_collision(player: &Player, cave: &mut Cave, camera_offset_x: f32) -> bool {
    let player_pos = (
        player.pos.x - PLAYER_SIZE.0 / 2.0,
        player.pos.y - PLAYER_SIZE.1 / 2.0,
    );

    // Get visible cave segments
    let view_start = camera_offset_x;
    let view_end = camera_offset_x + WINDOW_WIDTH as f32;
    let segments = cave.segments_in_view(view_start, view_end);

    for segment in segments {
        // Check collision with ceiling
        let ceiling_pos = (segment.x_start, 0.0);
        let ceiling_size = (segment.width, segment.ceiling);

        if aabb_overlap(player_pos, PLAYER_SIZE, ceiling_pos, ceiling_size) {
            return true;
        }

        // Check collision with floor
        let floor_pos = (segment.x_start, segment.floor);
        let floor_size = (segment.width, WINDOW_HEIGHT as f32 - segment.floor);

        if aabb_overlap(player_pos, PLAYER_SIZE, floor_pos, floor_size) {
            return true;
        }
    }

    false
}

/// Updates collision flash timer.
fn update_collision_flash(world: &mut GameWorld, dt: f32) {
    if world.collision_flash_timer > 0.0 {
        world.collision_flash_timer -= dt;
        if world.collision_flash_timer < 0.0 {
            world.collision_flash_timer = 0.0;
        }
    }
}

/// Updates game world physics and collision detection
fn update_game_world(world: &mut GameWorld, dt: f32) {
    match world.state_machine.current() {
        core::game_state::GameState::Playing => {
            // Update camera scroll
            world.camera_offset_x += SCROLL_SPEED * dt;

            // Update player physics
            let input = collect_player_input();
            world.player.tick(dt, input);

            // Check for collisions
            if check_player_collision(&world.player, &mut world.cave, world.camera_offset_x) {
                world.state_machine.handle_event(GameEvent::Dead);
                world.collision_flash_timer = COLLISION_FLASH_DURATION;
            }
        }
        _ => {
            // No physics updates in other states
        }
    }

    // Always update collision flash timer
    update_collision_flash(world, dt);
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

/// Renders collision flash effect.
fn render_collision_flash(collision_flash_timer: f32) {
    if collision_flash_timer > 0.0 {
        let alpha = collision_flash_timer / COLLISION_FLASH_DURATION;
        let flash_color = Color::new(1.0, 0.0, 0.0, alpha * 0.5);

        draw_rectangle(
            0.0,
            0.0,
            WINDOW_WIDTH as f32,
            WINDOW_HEIGHT as f32,
            flash_color,
        );
    }
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
            draw_text(
                "Use arrow keys to control",
                WINDOW_WIDTH as f32 / 2.0 - 100.0,
                WINDOW_HEIGHT as f32 / 2.0 + 30.0,
                16.0,
                GRAY,
            );
        }
        core::game_state::GameState::Playing => {
            draw_text("Playing - Don't hit the walls!", 10.0, 30.0, 20.0, WHITE);
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
            draw_text(
                "Press R to return to menu",
                WINDOW_WIDTH as f32 / 2.0 - 95.0,
                WINDOW_HEIGHT as f32 / 2.0 + 60.0,
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
                "You hit the wall!",
                WINDOW_WIDTH as f32 / 2.0 - 65.0,
                WINDOW_HEIGHT as f32 / 2.0 + 10.0,
                18.0,
                WHITE,
            );
            draw_text(
                "ENTER to restart, R to menu",
                WINDOW_WIDTH as f32 / 2.0 - 110.0,
                WINDOW_HEIGHT as f32 / 2.0 + 40.0,
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
        handle_state_input(&mut world);

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

        // Render collision flash effect
        render_collision_flash(world.collision_flash_timer);

        // Render UI
        render_ui(&world.state_machine);

        next_frame().await;
    }
}