// core/src/tractor.rs

/// Tractor beam system for the Fuel Drift game.
///
/// Manages tractor beam activation, direction, and timing
/// following the Single Responsibility Principle.

/// Direction of the tractor beam.
///
/// Simple enum for clear beam direction specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeamDir {
    Up,
    Down,
}

/// Tractor beam with activation state and timer.
///
/// Handles beam activation, direction, and automatic deactivation
/// following clean code principles with low cyclomatic complexity.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TractorBeam {
    pub active: bool,
    pub dir: BeamDir,
    pub timer: f32,
}

impl TractorBeam {
    /// Maximum duration the beam can remain active.
    pub const MAX_DURATION: f32 = crate::constants::TractorBeamConstants::MAX_DURATION;
    
    /// Maximum range of the tractor beam.
    pub const MAX_RANGE: f32 = crate::constants::TractorBeamConstants::MAX_RANGE;

    /// Creates a new inactive tractor beam.
    pub fn new() -> Self {
        Self {
            active: false,
            dir: BeamDir::Up, // Default direction
            timer: 0.0,
        }
    }

    /// Activates the tractor beam with specified direction.
    ///
    /// Only activates if beam is not already active.
    /// Sets timer to maximum duration when activated.
    pub fn activate(&mut self, dir: BeamDir) {
        if !self.active {
            self.active = true;
            self.dir = dir;
            self.timer = Self::MAX_DURATION;
        }
    }

    /// Updates the tractor beam timer.
    ///
    /// Decrements timer and deactivates beam when timer reaches zero.
    /// Call this method every frame during game updates.
    pub fn tick(&mut self, dt: f32) {
        if self.active {
            self.timer -= dt;
            if self.timer <= 0.0 {
                self.active = false;
                self.timer = 0.0;
            }
        }
    }

    /// Checks if beam is currently active.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Gets remaining time for active beam.
    ///
    /// Returns 0.0 if beam is inactive.
    pub fn remaining_time(&self) -> f32 {
        if self.active {
            self.timer.max(0.0)
        } else {
            0.0
        }
    }

    /// Checks if a point is within the tractor beam's influence for initial activation.
    ///
    /// Uses rectangular beam area that matches the visual beam rendering.
    ///
    /// # Arguments
    /// * `player_pos` - Player position (x, y)
    /// * `target_pos` - Target position to check (x, y)
    ///
    /// # Returns
    /// True if the target is within beam area and in correct direction
    pub fn is_point_in_beam(&self, player_pos: (f32, f32), target_pos: (f32, f32)) -> bool {
        self.is_point_in_beam_area(player_pos, target_pos, crate::constants::TractorBeamConstants::BEAM_WIDTH)
    }

    /// Checks if a point should remain attracted (wider area to prevent oscillation).
    ///
    /// # Arguments
    /// * `player_pos` - Player position (x, y)
    /// * `target_pos` - Target position to check (x, y)
    ///
    /// # Returns
    /// True if the target should continue being attracted
    pub fn should_maintain_attraction(&self, player_pos: (f32, f32), target_pos: (f32, f32)) -> bool {
        self.is_point_in_beam_area(player_pos, target_pos, crate::constants::TractorBeamConstants::ATTRACTION_HOLD_WIDTH)
    }

    /// Internal method for beam area checking with configurable width.
    fn is_point_in_beam_area(&self, player_pos: (f32, f32), target_pos: (f32, f32), beam_width: f32) -> bool {
        if !self.active {
            return false;
        }

        let dx = target_pos.0 - player_pos.0;
        let dy = target_pos.1 - player_pos.1;
        
        // Check if target is in the correct direction
        let in_direction = match self.dir {
            BeamDir::Up => dy < 0.0, // Target is above player (lower y)
            BeamDir::Down => dy > 0.0, // Target is below player (higher y)
        };
        
        if !in_direction {
            return false;
        }
        
        // Check if within rectangular beam area
        let beam_half_width = beam_width / 2.0;
        let horizontal_distance = dx.abs();
        let vertical_distance = dy.abs();
        
        // Must be within beam width horizontally and within range vertically
        horizontal_distance <= beam_half_width && vertical_distance <= Self::MAX_RANGE
    }

    /// Calculates attraction force for a point within the beam.
    ///
    /// Returns normalized direction vector pointing toward the player.
    /// Returns (0.0, 0.0) if point is not in beam or beam is inactive.
    ///
    /// # Arguments
    /// * `player_pos` - Player position (x, y)
    /// * `target_pos` - Target position (x, y)
    pub fn get_attraction_force(&self, player_pos: (f32, f32), target_pos: (f32, f32)) -> (f32, f32) {
        if !self.is_point_in_beam(player_pos, target_pos) {
            return (0.0, 0.0);
        }

        let dx = player_pos.0 - target_pos.0;
        let dy = player_pos.1 - target_pos.1;
        let distance = (dx * dx + dy * dy).sqrt();
        
        if distance > 0.0 {
            (dx / distance, dy / distance)
        } else {
            (0.0, 0.0)
        }
    }
}

impl Default for TractorBeam {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beam_range_detection() {
        let mut beam = TractorBeam::new();
        beam.activate(BeamDir::Up);
        
        let player_pos = (100.0, 200.0);
        
        // Point within beam area and correct direction (above player)
        let target_above = (100.0, 150.0);
        assert!(beam.is_point_in_beam(player_pos, target_above));
        
        // Point within range but wrong direction (below player for upward beam)
        let target_below = (100.0, 250.0);
        assert!(!beam.is_point_in_beam(player_pos, target_below));
        
        // Point in correct direction but out of range vertically
        let target_far = (100.0, -110.0); // 310 units away vertically (outside 300px range)
        assert!(!beam.is_point_in_beam(player_pos, target_far));
        
        // Point at edge of range vertically
        let target_edge = (100.0, 0.0); // Exactly 300 units away
        assert!(beam.is_point_in_beam(player_pos, target_edge));
        
        // Point in correct direction but outside beam width (too far left)
        let target_too_left = (50.0, 150.0); // 50 pixels left, outside 32px beam width
        assert!(!beam.is_point_in_beam(player_pos, target_too_left));
        
        // Point at edge of beam width (just inside)
        let target_beam_edge = (115.0, 150.0); // 15 pixels right, within 16px half-width
        assert!(beam.is_point_in_beam(player_pos, target_beam_edge));
        
        // Point just outside beam width
        let target_beam_outside = (117.0, 150.0); // 17 pixels right, outside 16px half-width
        assert!(!beam.is_point_in_beam(player_pos, target_beam_outside));
    }

    #[test]
    fn test_beam_direction_down() {
        let mut beam = TractorBeam::new();
        beam.activate(BeamDir::Down);
        
        let player_pos = (100.0, 200.0);
        
        // Point below player (correct direction for downward beam)
        let target_below = (100.0, 250.0);
        assert!(beam.is_point_in_beam(player_pos, target_below));
        
        // Point above player (wrong direction for downward beam)
        let target_above = (100.0, 150.0);
        assert!(!beam.is_point_in_beam(player_pos, target_above));
    }

    #[test]
    fn test_beam_inactive() {
        let beam = TractorBeam::new(); // Inactive by default
        let player_pos = (100.0, 200.0);
        let target = (100.0, 150.0);
        
        // Inactive beam should not detect any points
        assert!(!beam.is_point_in_beam(player_pos, target));
    }

    #[test]
    fn test_attraction_force_calculation() {
        let mut beam = TractorBeam::new();
        beam.activate(BeamDir::Up);
        
        let player_pos = (100.0, 200.0);
        let target_pos = (100.0, 150.0);
        
        let force = beam.get_attraction_force(player_pos, target_pos);
        
        // Force should point from target toward player
        assert_eq!(force, (0.0, 1.0)); // Normalized vector pointing down (toward player)
    }

    #[test]
    fn test_attraction_force_diagonal() {
        let mut beam = TractorBeam::new();
        beam.activate(BeamDir::Up);
        
        let player_pos = (100.0, 200.0);
        let target_pos = (110.0, 160.0); // Diagonal position within beam width (10px right, 40px up)
        
        let force = beam.get_attraction_force(player_pos, target_pos);
        
        // Force should be normalized and point toward player
        let expected_magnitude = (force.0 * force.0 + force.1 * force.1).sqrt();
        assert!((expected_magnitude - 1.0).abs() < 0.001); // Should be normalized
        
        // Force components should point toward player
        assert!(force.0 < 0.0); // X component points left (toward player)
        assert!(force.1 > 0.0); // Y component points down (toward player)
    }

    #[test]
    fn test_no_force_when_out_of_beam() {
        let mut beam = TractorBeam::new();
        beam.activate(BeamDir::Up);
        
        let player_pos = (100.0, 200.0);
        let target_pos = (100.0, 250.0); // Below player (wrong direction)
        
        let force = beam.get_attraction_force(player_pos, target_pos);
        assert_eq!(force, (0.0, 0.0));
    }

    #[test]
    fn test_force_same_position() {
        let mut beam = TractorBeam::new();
        beam.activate(BeamDir::Up);
        
        let player_pos = (100.0, 200.0);
        let target_pos = (100.0, 200.0); // Same position
        
        let force = beam.get_attraction_force(player_pos, target_pos);
        assert_eq!(force, (0.0, 0.0)); // No force when at same position
    }

    #[test]
    fn test_beam_timer_and_activation() {
        let mut beam = TractorBeam::new();
        let player_pos = (100.0, 200.0);
        let target_pos = (100.0, 150.0);
        
        // Initially inactive
        assert!(!beam.is_point_in_beam(player_pos, target_pos));
        
        // Activate beam
        beam.activate(BeamDir::Up);
        assert!(beam.is_point_in_beam(player_pos, target_pos));
        
        // Tick until beam expires
        for _ in 0..100 {
            beam.tick(0.1);
            if !beam.is_active() {
                break;
            }
        }
        
        // Should be inactive after timer expires
        assert!(!beam.is_point_in_beam(player_pos, target_pos));
    }
}
