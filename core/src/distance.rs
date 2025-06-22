/// Distance tracking system for the Fuel Drift game.
///
/// Tracks accumulated distance traveled during gameplay
/// following the Single Responsibility Principle.

/// Distance tracker with accumulated distance measurement.
///
/// Tracks the total distance traveled by the player during gameplay.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DistanceTracker {
    pub distance: f32,
}

impl DistanceTracker {
    /// Creates a new distance tracker starting at zero.
    pub fn new() -> Self {
        Self { distance: 0.0 }
    }

    /// Updates the distance based on scroll speed and delta time.
    ///
    /// # Arguments
    /// * `scroll_speed` - Speed of scrolling in pixels per second
    /// * `dt` - Delta time in seconds
    pub fn update(&mut self, scroll_speed: f32, dt: f32) {
        self.distance += scroll_speed * dt;
    }

    /// Resets the distance to zero.
    pub fn reset(&mut self) {
        self.distance = 0.0;
    }

    /// Gets the current distance as an integer for display.
    pub fn distance_as_int(&self) -> u32 {
        self.distance as u32
    }

    /// Gets the current distance as a formatted string.
    pub fn distance_formatted(&self) -> String {
        format!("{}m", self.distance_as_int())
    }
}

impl Default for DistanceTracker {
    fn default() -> Self {
        Self::new()
    }
}