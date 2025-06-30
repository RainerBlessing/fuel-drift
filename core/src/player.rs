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
        
        // Check boundary constraints before applying acceleration
        let screen_x = self.pos.x - camera_offset_x;
        let at_left_boundary = screen_x <= MIN_SCREEN_X;
        let at_right_boundary = screen_x >= MAX_SCREEN_X;
        
        // Apply acceleration based on input, but only if not pushing against boundary
        if input.left && (!at_left_boundary || self.vel.x > 0.0) {
            self.vel.x -= HORIZONTAL_ACCELERATION * dt;
        }

        if input.right && (!at_right_boundary || self.vel.x < 0.0) {
            self.vel.x += HORIZONTAL_ACCELERATION * dt;
        }

        // Clamp horizontal speed
        self.vel.x = self.vel.x.clamp(
            -PlayerConstants::MAX_HORIZONTAL_SPEED,
            PlayerConstants::MAX_HORIZONTAL_SPEED,
        );
        
        // Apply boundary constraints to position and velocity
        if at_left_boundary && self.vel.x < 0.0 {
            // Player is at left boundary and trying to move left
            self.pos.x = camera_offset_x + MIN_SCREEN_X;
            self.vel.x = 0.0; // Stop leftward movement
        } else if at_right_boundary && self.vel.x > 0.0 {
            // Player is at right boundary and trying to move right
            self.pos.x = camera_offset_x + MAX_SCREEN_X;
            self.vel.x = 0.0; // Stop rightward movement
        }
    }

    /// Updates position based on current velocity.
    fn update_position(&mut self, dt: f32) {
        self.pos.x += self.vel.x * dt;
        self.pos.y += self.vel.y * dt;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_boundary_release() {
        // Test that player can move away from left boundary
        let mut player = Player::new(Vec2::new(100.0, 100.0));
        let dt = 0.016; // ~60 FPS
        let camera_offset = 0.0;
        let scroll_speed = 0.0;
        
        // Force player to left boundary
        player.pos.x = 15.0; // MIN_SCREEN_X position
        player.vel.x = -50.0; // Moving left
        
        // Try to move right from boundary
        let input = PlayerInput {
            right: true,
            ..Default::default()
        };
        
        player.tick(dt, input, scroll_speed, camera_offset);
        
        // Player should be able to accelerate right
        assert!(player.vel.x > -50.0, "Player should be able to accelerate right from left boundary");
    }
    
    #[test]
    fn test_right_boundary_release() {
        // Test that player can move away from right boundary
        let mut player = Player::new(Vec2::new(785.0, 100.0));
        let dt = 0.016;
        let camera_offset = 0.0;
        let scroll_speed = 0.0;
        
        // Force player to right boundary
        player.pos.x = 785.0; // MAX_SCREEN_X position
        player.vel.x = 50.0; // Moving right
        
        // Try to move left from boundary
        let input = PlayerInput {
            left: true,
            ..Default::default()
        };
        
        player.tick(dt, input, scroll_speed, camera_offset);
        
        // Player should be able to accelerate left
        assert!(player.vel.x < 50.0, "Player should be able to accelerate left from right boundary");
    }
    
    #[test]
    fn test_boundary_prevents_movement_through() {
        // Test that boundaries still prevent movement through them
        let mut player = Player::new(Vec2::new(15.0, 100.0));
        let dt = 0.016;
        let camera_offset = 0.0;
        let scroll_speed = 0.0;
        
        // At left boundary, trying to move left
        player.vel.x = -100.0;
        let input = PlayerInput {
            left: true,
            ..Default::default()
        };
        
        player.tick(dt, input, scroll_speed, camera_offset);
        
        // Player should remain at boundary
        assert_eq!(player.pos.x, 15.0, "Player should stay at left boundary");
        assert_eq!(player.vel.x, 0.0, "Leftward velocity should be stopped");
    }
}
