
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// WASM-spezifische Initialisierung
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    //web_sys::console::log_1(&"🚀 Fuel Drift WASM starting...".into());
}

use core::audio::{AudioEvent, AudioEventQueue, AudioState};
use core::cave::Cave;
use core::collision::aabb_overlap;
use core::constants::{FuelConstants, PickupConstants};
use core::distance::DistanceTracker;
use core::fuel::Fuel;
use core::game_state::{GameEvent, StateMachine};
use core::level::LevelManager;
use core::pickup::PickupType;
use core::player::{Player, PlayerInput, Vec2};
use core::tractor::{BeamDir, TractorBeam};
use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};

mod headless_test;
mod ui;
mod menu;

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

/// Menu UI constants
const BUTTON_WIDTH: f32 = 200.0;
const BUTTON_HEIGHT: f32 = 50.0;
const MENU_SPACING: f32 = 20.0;

/// Simplified audio system for managing sound events without actual audio playback.
/// This is a stub implementation that logs audio events for development.
struct AudioSystem {
    audio_state: AudioState,
}

impl AudioSystem {
    fn new() -> Self {
        Self {
            audio_state: AudioState::new(),
        }
    }

    fn process_events(&mut self, events: Vec<AudioEvent>) {
        for event in events {
            self.play_event(event);
        }
    }

    fn play_event(&mut self, event: AudioEvent) {
        // Stub implementation - in production this would play actual sounds
        match event {
            AudioEvent::ThrusterLoop => {
                // Log thruster sound for debugging
                #[cfg(debug_assertions)]
                println!("🔊 Playing thruster loop sound");
            }
            AudioEvent::BeamActivation => {
                #[cfg(debug_assertions)]
                println!("🔊 Playing beam activation sound");
            }
            AudioEvent::FuelPickup => {
                #[cfg(debug_assertions)]
                println!("🔊 Playing fuel pickup sound");
            }
            AudioEvent::Death => {
                #[cfg(debug_assertions)]
                println!("🔊 Playing death sound");
            }
            AudioEvent::ButtonClick => {
                #[cfg(debug_assertions)]
                println!("🔊 Playing button click sound");
            }
        }
    }

    fn update_thruster(&mut self, should_play: bool) -> Option<AudioEvent> {
        if self.audio_state.update_thruster(should_play) {
            if should_play {
                Some(AudioEvent::ThrusterLoop)
            } else {
                // Stop thruster sound
                #[cfg(debug_assertions)]
                println!("🔊 Stopping thruster sound");
                None
            }
        } else {
            None
        }
    }

    fn stop_all(&mut self) {
        self.audio_state.stop_all();
        #[cfg(debug_assertions)]
        println!("🔊 Stopping all sounds");
    }
}

/// Menu selection state
#[derive(Debug, Clone, Copy, PartialEq)]
enum MenuSelection {
    Start,
    Quit,
}

/// Menu state for keyboard navigation
struct MenuState {
    main_menu_selection: MenuSelection,
    pause_menu_selection: usize, // 0 = Resume, 1 = Back to Menu
    game_over_menu_selection: usize, // 0 = Replay, 1 = Back to Menu
}

impl MenuState {
    fn new() -> Self {
        Self {
            main_menu_selection: MenuSelection::Start,
            pause_menu_selection: 0,
            game_over_menu_selection: 0,
        }
    }
}

/// Game state container following Single Responsibility Principle
struct GameWorld {
    state_machine: StateMachine,
    player: Player,
    fuel: Fuel,
    cave: Cave,
    tractor_beam: TractorBeam,
    distance_tracker: DistanceTracker,
    level_manager: LevelManager,
    audio_queue: AudioEventQueue,
    camera_offset_x: f32,
    collision_flash_timer: f32,
    should_quit: bool,
    menu_state: MenuState,
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
            level_manager: LevelManager::new(),
            audio_queue: AudioEventQueue::new(),
            camera_offset_x: 0.0,
            collision_flash_timer: 0.0,
            should_quit: false,
            menu_state: MenuState::new(),
        }
    }
    
    /// Gets the current level's fuel spawn distance, with fallback to default
    fn current_fuel_spawn_distance(&self) -> f32 {
        self.level_manager.current_level()
            .map(|level| level.fuel_spawn_distance)
            .unwrap_or(PickupConstants::DEFAULT_FUEL_SPAWN_DISTANCE)
    }

    /// Resets the game world for a new game.
    fn reset(&mut self) {
        self.player = Player::new(Vec2::new(100.0, 300.0));
        self.fuel = Fuel::new(INITIAL_FUEL, FUEL_BURN_RATE);
        self.tractor_beam = TractorBeam::new();
        self.distance_tracker.reset();
        self.level_manager.reset();
        self.camera_offset_x = 0.0;
        self.collision_flash_timer = 0.0;
        // Reset cave with new pickup manager and configure for level 1
        self.cave = Cave::new(42);
        self.cave.configure_for_level(1);
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

/// Handles main menu UI and interactions
fn handle_main_menu(world: &mut GameWorld, _audio_system: &mut AudioSystem) {
    let center_x = WINDOW_WIDTH as f32 / 2.0 - BUTTON_WIDTH / 2.0;
    let center_y = WINDOW_HEIGHT as f32 / 2.0;

    // Handle keyboard navigation
    if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::Down) {
        world.menu_state.main_menu_selection = match world.menu_state.main_menu_selection {
            MenuSelection::Start => MenuSelection::Quit,
            MenuSelection::Quit => MenuSelection::Start,
        };
        world.audio_queue.push(AudioEvent::ButtonClick);
    }

    // Handle selection with Enter or Space
    if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
        match world.menu_state.main_menu_selection {
            MenuSelection::Start => {
                world.audio_queue.push(AudioEvent::ButtonClick);
                world.state_machine.handle_event(GameEvent::Start);
                world.reset();
            }
            MenuSelection::Quit => {
                world.audio_queue.push(AudioEvent::ButtonClick);
                world.should_quit = true;
            }
        }
    }

    // Title
    draw_text(
        "FUEL DRIFT",
        WINDOW_WIDTH as f32 / 2.0 - 80.0,
        center_y - 100.0,
        40.0,
        WHITE,
    );

    // Instructions
    draw_text(
        "Arrow keys: Move | W/S: Tractor Beam | Watch your fuel!",
        WINDOW_WIDTH as f32 / 2.0 - 180.0,
        center_y - 50.0,
        16.0,
        GRAY,
    );
    
    // Keyboard instructions
    draw_text(
        "Press SPACE or ENTER to select",
        WINDOW_WIDTH as f32 / 2.0 - 120.0,
        center_y - 30.0,
        14.0,
        GRAY,
    );

    // Start button with selection highlight
    let start_color = if world.menu_state.main_menu_selection == MenuSelection::Start {
        YELLOW
    } else {
        WHITE
    };
    
    draw_rectangle_lines(
        center_x - 5.0,
        center_y - 5.0,
        BUTTON_WIDTH + 10.0,
        BUTTON_HEIGHT + 10.0,
        2.0,
        start_color,
    );
    
    if widgets::Button::new("Start Game")
        .position(vec2(center_x, center_y))
        .size(vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
        .ui(&mut root_ui())
    {
        world.audio_queue.push(AudioEvent::ButtonClick);
        world.state_machine.handle_event(GameEvent::Start);
        world.reset();
    }

    // Quit button with selection highlight
    let quit_color = if world.menu_state.main_menu_selection == MenuSelection::Quit {
        YELLOW
    } else {
        WHITE
    };
    
    draw_rectangle_lines(
        center_x - 5.0,
        center_y + BUTTON_HEIGHT + MENU_SPACING - 5.0,
        BUTTON_WIDTH + 10.0,
        BUTTON_HEIGHT + 10.0,
        2.0,
        quit_color,
    );
    
    if widgets::Button::new("Quit")
        .position(vec2(center_x, center_y + BUTTON_HEIGHT + MENU_SPACING))
        .size(vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
        .ui(&mut root_ui())
    {
        world.audio_queue.push(AudioEvent::ButtonClick);
        world.should_quit = true;
    }
}

/// Handles pause menu overlay
fn handle_pause_menu(world: &mut GameWorld) {
    // Semi-transparent overlay
    draw_rectangle(
        0.0,
        0.0,
        WINDOW_WIDTH as f32,
        WINDOW_HEIGHT as f32,
        Color::new(0.0, 0.0, 0.0, 0.7),
    );

    let center_x = WINDOW_WIDTH as f32 / 2.0 - BUTTON_WIDTH / 2.0;
    let center_y = WINDOW_HEIGHT as f32 / 2.0;

    // Handle keyboard navigation
    if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::Down) {
        world.menu_state.pause_menu_selection = 1 - world.menu_state.pause_menu_selection;
        world.audio_queue.push(AudioEvent::ButtonClick);
    }

    // Handle selection with Enter or Space
    if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
        match world.menu_state.pause_menu_selection {
            0 => {
                world.audio_queue.push(AudioEvent::ButtonClick);
                world.state_machine.handle_event(GameEvent::PauseToggle);
            }
            1 => {
                world.audio_queue.push(AudioEvent::ButtonClick);
                world.state_machine.handle_event(GameEvent::BackToMenu);
            }
            _ => {}
        }
    }

    // Title
    draw_text(
        "PAUSED",
        WINDOW_WIDTH as f32 / 2.0 - 50.0,
        center_y - 50.0,
        30.0,
        WHITE,
    );
    
    // Keyboard instructions
    draw_text(
        "Press SPACE or ENTER to select",
        WINDOW_WIDTH as f32 / 2.0 - 120.0,
        center_y - 10.0,
        14.0,
        GRAY,
    );

    // Resume button with selection highlight
    let resume_color = if world.menu_state.pause_menu_selection == 0 {
        YELLOW
    } else {
        WHITE
    };
    
    draw_rectangle_lines(
        center_x - 5.0,
        center_y - 5.0,
        BUTTON_WIDTH + 10.0,
        BUTTON_HEIGHT + 10.0,
        2.0,
        resume_color,
    );

    // Resume button
    if widgets::Button::new("Resume")
        .position(vec2(center_x, center_y))
        .size(vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
        .ui(&mut root_ui())
    {
        world.audio_queue.push(AudioEvent::ButtonClick);
        world.state_machine.handle_event(GameEvent::PauseToggle);
    }

    // Back to Menu button with selection highlight
    let back_color = if world.menu_state.pause_menu_selection == 1 {
        YELLOW
    } else {
        WHITE
    };
    
    draw_rectangle_lines(
        center_x - 5.0,
        center_y + BUTTON_HEIGHT + MENU_SPACING - 5.0,
        BUTTON_WIDTH + 10.0,
        BUTTON_HEIGHT + 10.0,
        2.0,
        back_color,
    );

    // Back to Menu button
    if widgets::Button::new("Back to Menu")
        .position(vec2(center_x, center_y + BUTTON_HEIGHT + MENU_SPACING))
        .size(vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
        .ui(&mut root_ui())
    {
        world.audio_queue.push(AudioEvent::ButtonClick);
        world.state_machine.handle_event(GameEvent::BackToMenu);
    }
}

/// Handles game over menu
fn handle_game_over_menu(world: &mut GameWorld) {
    let center_x = WINDOW_WIDTH as f32 / 2.0 - BUTTON_WIDTH / 2.0;
    let center_y = WINDOW_HEIGHT as f32 / 2.0;

    // Handle keyboard navigation
    if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::Down) {
        world.menu_state.game_over_menu_selection = 1 - world.menu_state.game_over_menu_selection;
        world.audio_queue.push(AudioEvent::ButtonClick);
    }

    // Handle selection with Enter or Space
    if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
        match world.menu_state.game_over_menu_selection {
            0 => {
                world.audio_queue.push(AudioEvent::ButtonClick);
                world.state_machine.handle_event(GameEvent::Start);
                world.reset();
            }
            1 => {
                world.audio_queue.push(AudioEvent::ButtonClick);
                world.state_machine.handle_event(GameEvent::BackToMenu);
            }
            _ => {}
        }
    }

    let death_message = if world.fuel.is_empty() {
        "OUT OF FUEL!"
    } else {
        "CRASHED!"
    };

    // Game Over title
    draw_text(
        "GAME OVER",
        WINDOW_WIDTH as f32 / 2.0 - 70.0,
        center_y - 80.0,
        30.0,
        RED,
    );

    // Death message
    draw_text(
        death_message,
        WINDOW_WIDTH as f32 / 2.0 - 65.0,
        center_y - 50.0,
        18.0,
        WHITE,
    );

    // Show final distance
    let final_distance = world.distance_tracker.distance_formatted();
    let distance_text = format!("Distance: {}", final_distance);
    draw_text(
        &distance_text,
        WINDOW_WIDTH as f32 / 2.0 - 80.0,
        center_y - 25.0,
        16.0,
        YELLOW,
    );
    
    // Keyboard instructions
    draw_text(
        "Press SPACE or ENTER to select",
        WINDOW_WIDTH as f32 / 2.0 - 120.0,
        center_y,
        14.0,
        GRAY,
    );

    // Replay button with selection highlight
    let replay_color = if world.menu_state.game_over_menu_selection == 0 {
        YELLOW
    } else {
        WHITE
    };
    
    draw_rectangle_lines(
        center_x - 5.0,
        center_y + 20.0 - 5.0,
        BUTTON_WIDTH + 10.0,
        BUTTON_HEIGHT + 10.0,
        2.0,
        replay_color,
    );

    // Replay button
    if widgets::Button::new("Replay")
        .position(vec2(center_x, center_y + 20.0))
        .size(vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
        .ui(&mut root_ui())
    {
        world.audio_queue.push(AudioEvent::ButtonClick);
        world.state_machine.handle_event(GameEvent::Start);
        world.reset();
    }

    // Back to Menu button with selection highlight
    let back_color = if world.menu_state.game_over_menu_selection == 1 {
        YELLOW
    } else {
        WHITE
    };
    
    draw_rectangle_lines(
        center_x - 5.0,
        center_y + 20.0 + BUTTON_HEIGHT + MENU_SPACING - 5.0,
        BUTTON_WIDTH + 10.0,
        BUTTON_HEIGHT + 10.0,
        2.0,
        back_color,
    );

    // Back to Menu button
    if widgets::Button::new("Back to Menu")
        .position(vec2(
            center_x,
            center_y + 20.0 + BUTTON_HEIGHT + MENU_SPACING,
        ))
        .size(vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
        .ui(&mut root_ui())
    {
        world.audio_queue.push(AudioEvent::ButtonClick);
        world.state_machine.handle_event(GameEvent::BackToMenu);
    }
}

/// Handles keyboard input for state transitions
fn handle_keyboard_input(world: &mut GameWorld) {
    let current_state = world.state_machine.current();

    match current_state {
        core::game_state::GameState::Playing => {
            if is_key_pressed(KeyCode::Escape) {
                world.state_machine.handle_event(GameEvent::PauseToggle);
            }
        }
        core::game_state::GameState::Paused => {
            if is_key_pressed(KeyCode::Escape) {
                world.state_machine.handle_event(GameEvent::BackToMenu);
            }
        }
        core::game_state::GameState::GameOver => {
            if is_key_pressed(KeyCode::Escape) {
                world.state_machine.handle_event(GameEvent::BackToMenu);
            }
        }
        _ => {}
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
/// Right movement (acceleration) consumes fuel, left movement (braking) does not.
fn is_consuming_fuel(input: PlayerInput) -> bool {
    input.up || input.down || input.right
}

/// Checks for collision between player and cave walls.
fn check_player_collision(player: &Player, cave: &mut Cave, camera_offset_x: f32) -> bool {
    let player_pos = (
        player.pos.x - PLAYER_SIZE.0 / 2.0,
        player.pos.y - PLAYER_SIZE.1 / 2.0,
    );

    let view_start = camera_offset_x;
    let view_end = camera_offset_x + WINDOW_WIDTH as f32;
    // Default fuel spawn distance for collision detection
    let segments = cave.segments_in_view(view_start, view_end, PickupConstants::DEFAULT_FUEL_SPAWN_DISTANCE);

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

/// Updates camera position and distance tracking
fn update_camera_and_distance(world: &mut GameWorld, dt: f32) {
    world.camera_offset_x += SCROLL_SPEED * dt;
    world.distance_tracker.update(SCROLL_SPEED, dt);
}

/// Handles player input, physics, and tractor beam
fn handle_player_input_and_physics(world: &mut GameWorld, audio_system: &mut AudioSystem, dt: f32) {
    let input = collect_player_input();
    
    // Handle tractor beam activation
    if input.tractor_up {
        world.tractor_beam.activate(BeamDir::Up);
        world.audio_queue.push(AudioEvent::BeamActivation);
    }
    if input.tractor_down {
        world.tractor_beam.activate(BeamDir::Down);
        world.audio_queue.push(AudioEvent::BeamActivation);
    }

    // Update tractor beam timer
    world.tractor_beam.tick(dt);

    let consuming = is_consuming_fuel(input);

    // Update thruster audio
    if let Some(thruster_event) = audio_system.update_thruster(consuming) {
        world.audio_queue.push(thruster_event);
    }

    // Update fuel and check for empty state
    let fuel_became_empty = world.fuel.burn(dt, consuming);
    if fuel_became_empty {
        trigger_death(world, audio_system);
        return;
    }

    // Update player physics only if fuel is available
    if !world.fuel.is_empty() {
        world.player.tick(dt, input, SCROLL_SPEED, world.camera_offset_x);
    }
}

/// Updates tractor beam pickup attraction effects
fn update_tractor_beam_pickup_attraction(world: &mut GameWorld, dt: f32) {
    let player_pos = (world.player.pos.x, world.player.pos.y);
    
    // Update pickup attraction based on tractor beam state
    world.cave.pickup_manager_mut().update_tractor_beam_attraction(
        &world.tractor_beam,
        player_pos,
        dt,
    );
}

/// Checks collisions and handles pickup collection
fn check_collisions_and_pickups(world: &mut GameWorld, audio_system: &mut AudioSystem, dt: f32) {
    // Update tractor beam pickup attraction first
    update_tractor_beam_pickup_attraction(world, dt);
    
    // Check for collisions with walls
    if check_player_collision(&world.player, &mut world.cave, world.camera_offset_x) {
        trigger_death(world, audio_system);
        return;
    }
    
    // Check for pickup collection
    if let Some(pickup_index) = world.cave.pickup_manager().check_collision(
        (world.player.pos.x - PLAYER_SIZE.0 / 2.0, world.player.pos.y - PLAYER_SIZE.1 / 2.0),
        PLAYER_SIZE,
    ) {
        if let Some(pickup_type) = world.cave.pickup_manager_mut().collect_pickup(pickup_index) {
            handle_pickup_collection(world, pickup_type);
        }
    }
    
    // Cleanup old pickups
    world.cave.pickup_manager_mut().cleanup_old_pickups(world.camera_offset_x);
}

/// Handles pickup collection effects
fn handle_pickup_collection(world: &mut GameWorld, pickup_type: PickupType) {
    match pickup_type {
        PickupType::Fuel => {
            // Refill fuel based on configured percentage
            let refill_amount = world.fuel.max * FuelConstants::REFILL_PERCENTAGE;
            world.fuel.refill(refill_amount);
            world.audio_queue.push(AudioEvent::FuelPickup);
        }
    }
}

/// Triggers death state and effects
fn trigger_death(world: &mut GameWorld, audio_system: &mut AudioSystem) {
    world.audio_queue.push(AudioEvent::Death);
    world.state_machine.handle_event(GameEvent::Dead);
    world.collision_flash_timer = COLLISION_FLASH_DURATION;
    audio_system.stop_all();
}

/// Updates game world physics, tractor beam, and collision detection
fn update_game_world(world: &mut GameWorld, audio_system: &mut AudioSystem, dt: f32) {
    match world.state_machine.current() {
        core::game_state::GameState::Playing => {
            update_camera_and_distance(world, dt);
            
            // Check for level progression
            let current_time = world.distance_tracker.elapsed_time();
            if let Ok(level_changed) = world.level_manager.update(current_time) {
                if level_changed {
                    // Configure cave for new level
                    let new_level_number = world.level_manager.current_level_number();
                    world.cave.configure_for_level(new_level_number);
                    // TODO: Add level up sound
                    world.audio_queue.push(AudioEvent::ButtonClick);
                }
            }
            
            handle_player_input_and_physics(world, audio_system, dt);
            
            // Only check collisions if still playing (fuel didn't run out)
            if world.state_machine.current() == core::game_state::GameState::Playing {
                check_collisions_and_pickups(world, audio_system, dt);
            }
        }
        _ => {
            // Stop all sounds when not playing
            audio_system.stop_all();
        }
    }

    // Always update collision flash timer
    update_collision_flash(world, dt);
}

/// Renders the cave segments
fn render_cave(cave: &mut Cave, fuel_spawn_distance: f32, camera_offset_x: f32) {
    let view_start = camera_offset_x;
    let view_end = camera_offset_x + WINDOW_WIDTH as f32;

    let segments = cave.segments_in_view(view_start, view_end, fuel_spawn_distance);
    for segment in segments {
        let screen_x = segment.x_start - camera_offset_x;

        // Draw ceiling (black rectangle from top to ceiling height)
        draw_rectangle(screen_x, 0.0, segment.width, segment.ceiling, BLACK);

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

/// Renders fuel pickups
fn render_pickups(cave: &Cave, camera_offset_x: f32) {
    let view_start = camera_offset_x;
    let view_end = camera_offset_x + WINDOW_WIDTH as f32;
    
    let pickups = cave.pickup_manager().get_pickups_in_range(view_start, view_end);
    for pickup in pickups {
        let screen_x = pickup.position.0 - camera_offset_x;
        let screen_y = pickup.position.1;
        
        // Draw fuel depot as a yellow/orange rectangle
        draw_rectangle(
            screen_x,
            screen_y,
            PickupConstants::SIZE,
            PickupConstants::SIZE,
            ORANGE,
        );
        
        // Draw a small "F" for fuel
        let text_offset_x = PickupConstants::SIZE * 0.3; // 30% of size
        let text_offset_y = PickupConstants::SIZE * 0.75; // 75% of size
        draw_text("F", screen_x + text_offset_x, screen_y + text_offset_y, 16.0, WHITE);
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

    draw_text(&distance_text, text_x, text_y, text_size, WHITE);
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

/// Renders the tractor beam as a blue rectangle ending at cave walls
fn render_tractor_beam(
    player: &Player,
    tractor_beam: &TractorBeam,
    cave: &mut Cave,
    camera_offset_x: f32,
) {
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
fn get_cave_wall_height_at_position(x_pos: f32, beam_dir: BeamDir, cave: &mut Cave) -> f32 {
    // Get cave segments around player position
    let view_start = x_pos - 50.0; // Small buffer around player
    let view_end = x_pos + 50.0;
    // Default fuel spawn distance for wall detection
    let segments = cave.segments_in_view(view_start, view_end, PickupConstants::DEFAULT_FUEL_SPAWN_DISTANCE);

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
        BeamDir::Up => 0.0,                    // Top of window
        BeamDir::Down => WINDOW_HEIGHT as f32, // Bottom of window
    }
}

/// Main entry point with command line argument handling
#[macroquad::main(window_conf)]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Check for help flag
    if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        println!("Fuel Drift - A cave flying game");
        println!();
        println!("Usage: {} [OPTIONS]", args[0]);
        println!();
        println!("OPTIONS:");
        println!("  --start, -s          Start the game directly (skip main menu)");
        println!("  --headless-test      Run headless test for CI");
        println!("  --help, -h           Show this help message");
        println!();
        println!("CONTROLS:");
        println!("  Arrow Keys           Move spaceship");
        println!("  W/S                  Activate tractor beam");
        println!("  ESC                  Pause game");
        println!("  SPACE/ENTER          Select menu option");
        return;
    }

    // Check for headless test flag
    if args.contains(&"--headless-test".to_string()) {
        if let Err(e) = headless_test::run_headless_test(5.0) {
            eprintln!("Headless test failed: {}", e);
            std::process::exit(1);
        }
        println!("Headless test passed!");
        return;
    }

    // Check for direct start flag
    let direct_start = args.contains(&"--start".to_string()) || args.contains(&"-s".to_string());

    // Initialize audio system (stub implementation)
    let mut audio_system = AudioSystem::new();
    let mut world = GameWorld::new();
    
    // Start game directly if requested
    if direct_start {
        world.state_machine.handle_event(GameEvent::Start);
        world.reset();
    }

    loop {
        if world.should_quit {
            break;
        }

        let dt = get_frame_time();

        // Handle keyboard input
        handle_keyboard_input(&mut world);

        // Process audio events from previous frame
        let audio_events = world.audio_queue.drain();
        audio_system.process_events(audio_events);

        // Handle UI based on current state
        match world.state_machine.current() {
            core::game_state::GameState::Menu => {
                clear_background(DARKBLUE);
                handle_main_menu(&mut world, &mut audio_system);
            }
            core::game_state::GameState::Playing => {
                update_game_world(&mut world, &mut audio_system, dt);

                clear_background(DARKBLUE);
                let fuel_spawn_distance = world.current_fuel_spawn_distance();
                render_cave(&mut world.cave, fuel_spawn_distance, world.camera_offset_x);
                render_pickups(&world.cave, world.camera_offset_x);
                render_player(&world.player, world.camera_offset_x);
                render_tractor_beam(
                    &world.player,
                    &world.tractor_beam,
                    &mut world.cave,
                    world.camera_offset_x,
                );
                render_fuel_bar(&world.fuel);
                render_distance_display(&world.distance_tracker);
                render_beam_indicator(&world.tractor_beam);
                render_collision_flash(world.collision_flash_timer);
            }
            core::game_state::GameState::Paused => {
                // Keep game visuals but add pause overlay
                clear_background(DARKBLUE);
                let fuel_spawn_distance = world.current_fuel_spawn_distance();
                render_cave(&mut world.cave, fuel_spawn_distance, world.camera_offset_x);
                render_pickups(&world.cave, world.camera_offset_x);
                render_player(&world.player, world.camera_offset_x);
                render_tractor_beam(
                    &world.player,
                    &world.tractor_beam,
                    &mut world.cave,
                    world.camera_offset_x,
                );
                render_fuel_bar(&world.fuel);
                render_distance_display(&world.distance_tracker);
                render_beam_indicator(&world.tractor_beam);

                handle_pause_menu(&mut world);
            }
            core::game_state::GameState::GameOver => {
                clear_background(DARKBLUE);
                render_collision_flash(world.collision_flash_timer);
                handle_game_over_menu(&mut world);
            }
        }

        next_frame().await;
    }
}
