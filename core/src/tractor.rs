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
    pub const MAX_DURATION: f32 = 2.0;

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
}

impl Default for TractorBeam {
    fn default() -> Self {
        Self::new()
    }
}