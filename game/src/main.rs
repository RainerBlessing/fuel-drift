// game/src/main.rs

use macroquad::prelude::*;
use core::game_state::{GameEvent, StateMachine};
use core::player::{Player, PlayerInput, Vec2};
use core::cave::Cave;
use core::collision::aabb_overlap;
use core::fuel::Fuel;
use core::tractor::{TractorBeam, BeamDir};
use core::distance::DistanceTracker;

/// Window configuration constants
const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
const WINDOW_TITLE: &str = "Fuel Drift";

/// Game constants
const SCROLL_SPEED: f32 = 120.0; // pixels/second
const PLAYER_SIZE: (f32, f32) = (30.0, 18.0);

/// Fuel constants
const INITIAL_FUEL: f32 = 100.0;
const FUEL_BURN_RATE: f32 = 20.0; // fuel per second when consuming

/// Collision flash constants
const COLLISION_FLASH_DURATION: f32 = 0.3; // seconds

/// UI constants
const FUEL_BAR_HEIGHT: f32 = 20.0;
const FUEL_BAR_Y: f32 = 10.0;
const FUEL_BAR_MARGIN: f32 = 10.0;
const LOW_FUEL_THRESHOLD: f32 = 0.2;
const MEDIUM_FUEL_THRESHOLD: f32 = 0.5;
const BEAM_ICON_SIZE: f32 = 16.0;

/// Game state container following Single Responsibility Principle
struct GameWorld {
    state_machine: StateMachine,
    player: Player,
    fuel: Fuel,
    cave: Cave,
    tractor_beam: TractorBeam,
    distance_tracker: DistanceTracker,
    camera_offset_x: f32,
    collision_flash_timer: f32,
}

impl GameWorld {
    fn new() -> Self {
        Self {
            state_machine: StateMachine::new(),
            player: Player::new(Vec2::new(100.0, 300.0)),
            fuel: Fuel::new(INITIAL_FUEL, FUEL_BURN_RATE),
            cave: Cave::new(42), // Fixed seed for consistent cave
            tractor_beam: TractorBeam::new(),
            distance_tracker: DistanceTracker::new(),
            camera_offset_x: 0.0,
            collision_flash_timer: 0.0,
        }
    }

    /// Resets the game world for a new game.
    fn reset(&mut self) {
        self.player = Player::new(Vec2::new(100.0, 300.0));
        self.fuel = Fuel::new(INITIAL_FUEL, FUEL_BURN_RATE);
        self.tractor_beam = TractorBeam::new();
        self.distance_tracker.reset();
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

/// Collects player input for physics simulation and tractor beam control
fn collect_player_input() -> PlayerInput {
    PlayerInput {
        up: is_key_down(KeyCode::Up),
        down: is_key_down(KeyCode::Down),
        left: is_key_down(KeyCode::Left),
        right: is_key_down(KeyCode::Right),
        tractor_up: is_key_pressed(KeyCode::W),
        tractor_down: is_key_pressed(KeyCode::S),
    }
}

/// Checks if player is currently consuming fuel.
///
/// Fuel is consumed when any movement input is active.
fn is_consuming_fuel(input: PlayerInput) -> bool {
    input.up || input.down || input.left || input.right
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

/// Updates game world physics, tractor beam, and collision detection
fn update_game_world(world: &mut GameWorld, dt: f32) {
    match world.state_machine.current() {
        core::game_state::GameState::Playing => {
            // Update camera scroll
            world.camera_offset_x += SCROLL_SPEED * dt;

            // Update distance tracker
            world.distance_tracker.update(SCROLL_SPEED, dt);

            // Collect input
            let input = collect_player_input();

            // Handle tractor beam activation
            if input.tractor_up {
                world.tractor_beam.activate(BeamDir::Up);
            }
            if input.tractor_down {
                world.tractor_beam.activate(BeamDir::Down);
            }

            // Update tractor beam timer
            world.tractor_beam.tick(dt);

            let consuming = is_consuming_fuel(input);

            // Update fuel and check for empty state
            let fuel_became_empty = world.fuel.burn(dt, consuming);
            if fuel_became_empty {
                world.state_machine.handle_event(GameEvent::Dead);
                world.collision_flash_timer = COLLISION_FLASH_DURATION;
                return; // Don't update player if fuel is empty
            }

            // Update player physics only if fuel is available
            if !world.fuel.is_empty() {
                world.player.tick(dt, input);
            }

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

/// Calculates fuel color based on fuel ratio with smooth gradient.
fn get_fuel_color(ratio: f32) -> Color {
    if ratio > MEDIUM_FUEL_THRESHOLD {
        // Green to yellow (ratio: 1.0 -> 0.5)
        let t = (1.0 - ratio) / (1.0 - MEDIUM_FUEL_THRESHOLD);
        Color::new(t, 1.0, 0.0, 1.0)
    } else if ratio > LOW_FUEL_THRESHOLD {
        // Yellow to red (ratio: 0.5 -> 0.2)
        let t = (ratio - LOW_FUEL_THRESHOLD) / (MEDIUM_FUEL_THRESHOLD - LOW_FUEL_THRESHOLD);
        Color::new(1.0, t, 0.0, 1.0)
    } else {
        // Pure red (ratio: 0.2 -> 0.0)
        RED
    }
}

/// Renders the fuel bar spanning the top of the screen.
fn render_fuel_bar(fuel: &Fuel) {
    let ratio = fuel.ratio();
    let bar_width = WINDOW_WIDTH as f32 - 2.0 * FUEL_BAR_MARGIN;

    // Background bar (dark gray)
    draw_rectangle(
        FUEL_BAR_MARGIN,
        FUEL_BAR_Y,
        bar_width,
        FUEL_BAR_HEIGHT,
        DARKGRAY,
    );

    // Fuel level bar with gradient color
    let fuel_width = bar_width * ratio;
    let fuel_color = get_fuel_color(ratio);

    draw_rectangle(
        FUEL_BAR_MARGIN,
        FUEL_BAR_Y,
        fuel_width,
        FUEL_BAR_HEIGHT,
        fuel_color,
    );

    // Border
    draw_rectangle_lines(
        FUEL_BAR_MARGIN,
        FUEL_BAR_Y,
        bar_width,
        FUEL_BAR_HEIGHT,
        2.0,
        WHITE,
    );

    // Fuel percentage text
    let fuel_text = format!("{}%", (ratio * 100.0) as u32);
    draw_text(
        &fuel_text,
        FUEL_BAR_MARGIN + 5.0,
        FUEL_BAR_Y + FUEL_BAR_HEIGHT - 5.0,
        14.0,
        WHITE,
    );
}

/// Renders the distance display in the top-right corner.
fn render_distance_display(distance_tracker: &DistanceTracker) {
    let distance_text = distance_tracker.distance_formatted();
    let text_size = 20.0;
    let margin = 15.0;

    // Calculate text position (right-aligned)
    let text_width = measure_text(&distance_text, None, text_size as u16, 1.0).width;
    let text_x = WINDOW_WIDTH as f32 - text_width - margin;
    let text_y = margin + text_size;

    draw_text(
        &distance_text,
        text_x,
        text_y,
        text_size,
        WHITE,
    );
}

/// Renders the beam ready indicator icon.
fn render_beam_indicator(tractor_beam: &TractorBeam) {
    let icon_x = FUEL_BAR_MARGIN + 5.0;
    let icon_y = FUEL_BAR_Y + FUEL_BAR_HEIGHT + 10.0;

    // Choose color based on beam state
    let icon_color = if tractor_beam.is_active() {
        GRAY // Grayed out when active
    } else {
        Color::new(0.5, 0.8, 1.0, 1.0) // Light blue when ready
    };

    // Draw simple beam icon (triangle pointing up)
    let points = [
        Vec2::new(icon_x + BEAM_ICON_SIZE / 2.0, icon_y),
        Vec2::new(icon_x, icon_y + BEAM_ICON_SIZE),
        Vec2::new(icon_x + BEAM_ICON_SIZE, icon_y + BEAM_ICON_SIZE),
    ];

    // Draw filled triangle
    for i in 0..3 {
        let start = points[i];
        let end = points[(i + 1) % 3];
        draw_line(start.x, start.y, end.x, end.y, 2.0, icon_color);
    }

    // Fill triangle (simple approximation)
    for y in 0..BEAM_ICON_SIZE as i32 {
        let progress = y as f32 / BEAM_ICON_SIZE;
        let line_width = BEAM_ICON_SIZE * progress;
        let line_start = icon_x + (BEAM_ICON_SIZE - line_width) / 2.0;

        draw_rectangle(
            line_start,
            icon_y + y as f32,
            line_width,
            1.0,
            Color::new(icon_color.r, icon_color.g, icon_color.b, 0.5),
        );
    }

    // Beam status text
    let status_text = if tractor_beam.is_active() {
        "BEAM ACTIVE"
    } else {
        "BEAM READY"
    };

    draw_text(
        status_text,
        icon_x + BEAM_ICON_SIZE + 5.0,
        icon_y + BEAM_ICON_SIZE / 2.0 + 4.0,
        12.0,
        icon_color,
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
fn render_ui(state_machine: &StateMachine, fuel: &Fuel, distance_tracker: &DistanceTracker, tractor_beam: &TractorBeam) {
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
                "Arrow keys: Move | W/S: Tractor Beam | Watch your fuel!",
                WINDOW_WIDTH as f32 / 2.0 - 180.0,
                WINDOW_HEIGHT as f32 / 2.0 + 30.0,
                16.0,
                GRAY,
            );
        }
        core::game_state::GameState::Playing => {
            render_fuel_bar(fuel);
            render_distance_display(distance_tracker);
            render_beam_indicator(tractor_beam);
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
            let death_message = if fuel.is_empty() {
                "OUT OF FUEL!"
            } else {
                "CRASHED!"
            };

            draw_text(
                "GAME OVER",
                WINDOW_WIDTH as f32 / 2.0 - 70.0,
                WINDOW_HEIGHT as f32 / 2.0 - 20.0,
                30.0,
                RED,
            );
            draw_text(
                death_message,
                WINDOW_WIDTH as f32 / 2.0 - 65.0,
                WINDOW_HEIGHT as f32 / 2.0 + 10.0,
                18.0,
                WHITE,
            );

            // Show final distance
            let final_distance = distance_tracker.distance_formatted();
            let distance_text = format!("Distance: {}", final_distance);
            draw_text(
                &distance_text,
                WINDOW_WIDTH as f32 / 2.0 - 80.0,
                WINDOW_HEIGHT as f32 / 2.0 + 35.0,
                16.0,
                YELLOW,
            );

            draw_text(
                "ENTER to restart, R to menu",
                WINDOW_WIDTH as f32 / 2.0 - 110.0,
                WINDOW_HEIGHT as f32 / 2.0 + 60.0,
                16.0,
                GRAY,
            );
        }
    }
}

/// Renders the tractor beam as a blue rectangle ending at cave walls
fn render_tractor_beam(player: &Player, tractor_beam: &TractorBeam, cave: &mut Cave, camera_offset_x: f32) {
    if !tractor_beam.is_active() {
        return;
    }

    let screen_x = player.pos.x - camera_offset_x;
    let beam_width = 32.0; // 16px on each side of player center
    let beam_x = screen_x - beam_width / 2.0;

    // Get cave segments at player position to find wall heights
    let wall_height = get_cave_wall_height_at_position(player.pos.x, tractor_beam.dir, cave);

    match tractor_beam.dir {
        BeamDir::Up => {
            // Beam from player to ceiling
            let beam_start_y = wall_height; // Ceiling height
            let beam_height = player.pos.y - wall_height;

            // Only draw if there's space between player and ceiling
            if beam_height > 0.0 {
                draw_rectangle(
                    beam_x,
                    beam_start_y,
                    beam_width,
                    beam_height,
                    Color::new(0.0, 0.5, 1.0, 0.6), // Semi-transparent blue
                );
            }
        }
        BeamDir::Down => {
            // Beam from player to floor
            let beam_start_y = player.pos.y;
            let beam_height = wall_height - player.pos.y; // Floor height - player position

            // Only draw if there's space between player and floor
            if beam_height > 0.0 {
                draw_rectangle(
                    beam_x,
                    beam_start_y,
                    beam_width,
                    beam_height,
                    Color::new(0.0, 0.5, 1.0, 0.6), // Semi-transparent blue
                );
            }
        }
    }
}

/// Gets the cave wall height (ceiling or floor) at the specified x position.
///
/// Returns the y-coordinate of the wall that the beam should hit.
/// For Up direction: returns ceiling height
/// For Down direction: returns floor height
fn get_cave_wall_height_at_position(x_pos: f32, beam_dir: BeamDir, cave: &mut Cave) -> f32 {
    // Get cave segments around player position
    let view_start = x_pos - 50.0; // Small buffer around player
    let view_end = x_pos + 50.0;
    let segments = cave.segments_in_view(view_start, view_end);

    // Find the segment that contains the player's x position
    for segment in segments {
        if x_pos >= segment.x_start && x_pos < segment.x_end() {
            return match beam_dir {
                BeamDir::Up => segment.ceiling,
                BeamDir::Down => segment.floor,
            };
        }
    }

    // Fallback if no segment found (shouldn't happen in normal gameplay)
    match beam_dir {
        BeamDir::Up => 0.0, // Top of window
        BeamDir::Down => WINDOW_HEIGHT as f32, // Bottom of window
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
                render_tractor_beam(&world.player, &world.tractor_beam, &mut world.cave, world.camera_offset_x);
            }
            _ => {}
        }

        // Render collision flash effect
        render_collision_flash(world.collision_flash_timer);

        // Render UI
        render_ui(&world.state_machine, &world.fuel, &world.distance_tracker, &world.tractor_beam);

        next_frame().await;
    }
}