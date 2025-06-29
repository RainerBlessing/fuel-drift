/// 2D vector for position and velocity calculations.
///
/// Simple structure following the principle of least surprise.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    /// Creates a new Vec2.
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Zero vector constant.
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };
}

/// Player input state for a single frame.
///
/// Contains only boolean flags for clean input handling.
#[derive(Debug, Clone, Copy, Default)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub tractor_up: bool,   // W key for upward tractor beam
    pub tractor_down: bool, // S key for downward tractor beam
}

/// Player physics constants.
///
/// Centralized constants for easy tuning and testing.
pub struct PlayerConstants;

impl PlayerConstants {
    pub const GRAVITY: f32 = 0.0; // pixels/sec² (disabled)
    pub const THRUST: f32 = -400.0; // pixels/sec² (negative = upward)
    pub const MAX_HORIZONTAL_SPEED: f32 = 200.0; // pixels/sec
    pub const DOWN_THRUST_MULTIPLIER: f32 = 0.5;
}

/// Player entity with position and velocity.
///
/// Handles physics calculations without rendering concerns,
/// following the Single Responsibility Principle.
#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub pos: Vec2,
    pub vel: Vec2,
}

impl Player {
    /// Creates a new player at the specified position.
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            vel: Vec2::ZERO,
        }
    }

    /// Updates player physics for one frame.
    ///
    /// Separated into smaller methods to reduce cyclomatic complexity.
    /// scroll_speed: The horizontal scroll speed of the world (pixels/sec)
    /// camera_offset_x: The current camera offset for boundary checking
    pub fn tick(&mut self, dt: f32, input: PlayerInput, scroll_speed: f32, camera_offset_x: f32) {
        self.apply_gravity(dt);
        self.apply_thrust(dt, input);
        self.apply_horizontal_movement(dt, input, scroll_speed, camera_offset_x);
        self.update_position(dt);
    }

    /// Applies gravity to vertical velocity.
    fn apply_gravity(&mut self, dt: f32) {
        self.vel.y += PlayerConstants::GRAVITY * dt;
    }

    /// Applies thrust based on input.
    fn apply_thrust(&mut self, dt: f32, input: PlayerInput) {
        if input.up {
            self.vel.y += PlayerConstants::THRUST * dt;
        }

        if input.down {
            self.vel.y += -PlayerConstants::THRUST * PlayerConstants::DOWN_THRUST_MULTIPLIER * dt;
        }
    }

    /// Applies horizontal movement with speed clamping.
    /// Includes base scroll speed compensation and boundary checks.
    fn apply_horizontal_movement(&mut self, dt: f32, input: PlayerInput, scroll_speed: f32, camera_offset_x: f32) {
        const HORIZONTAL_ACCELERATION: f32 = 800.0; // pixels/sec²
        const PLAYER_HALF_WIDTH: f32 = 15.0; // Half of player sprite width (30.0 / 2)
        const SCREEN_WIDTH: f32 = 800.0;
        const MIN_SCREEN_X: f32 = PLAYER_HALF_WIDTH;  // Player center when left edge touches screen
        const MAX_SCREEN_X: f32 = SCREEN_WIDTH - PLAYER_HALF_WIDTH; // Player center when right edge touches screen
        
        // Apply base scroll speed to maintain position relative to scrolling world
        self.pos.x += scroll_speed * dt;
        
        // Apply acceleration based on input
        if input.left {
            self.vel.x -= HORIZONTAL_ACCELERATION * dt;
        }

        if input.right {
            self.vel.x += HORIZONTAL_ACCELERATION * dt;
        }

        // Clamp horizontal speed
        self.vel.x = self.vel.x.clamp(
            -PlayerConstants::MAX_HORIZONTAL_SPEED,
            PlayerConstants::MAX_HORIZONTAL_SPEED,
        );
        
        // Apply boundary constraints based on screen position
        let screen_x = self.pos.x - camera_offset_x;
        
        if screen_x < MIN_SCREEN_X {
            // Player is too far left on screen
            self.pos.x = camera_offset_x + MIN_SCREEN_X;
            self.vel.x = self.vel.x.max(0.0); // Stop leftward movement
        } else if screen_x > MAX_SCREEN_X {
            // Player is too far right on screen
            self.pos.x = camera_offset_x + MAX_SCREEN_X;
            self.vel.x = self.vel.x.min(0.0); // Stop rightward movement
        }
    }

    /// Updates position based on current velocity.
    fn update_position(&mut self, dt: f32) {
        self.pos.x += self.vel.x * dt;
        self.pos.y += self.vel.y * dt;
    }
}
