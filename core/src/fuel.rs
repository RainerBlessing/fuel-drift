/// Fuel system for the Fuel Drift game.
///
/// Manages fuel consumption, refilling, and empty state detection
/// following the Single Responsibility Principle.

/// Fuel container with consumption and refilling capabilities.
///
/// Tracks current fuel level, maximum capacity, and burn rate.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fuel {
    pub current: f32,
    pub max: f32,
    pub burn_rate: f32,
}

impl Fuel {
    /// Creates a new fuel container with specified parameters.
    ///
    /// # Arguments
    /// * `max` - Maximum fuel capacity
    /// * `burn_rate` - Fuel consumption per second when active
    pub fn new(max: f32, burn_rate: f32) -> Self {
        Self {
            current: max,
            max,
            burn_rate,
        }
    }

    /// Burns fuel over time when consuming is active.
    ///
    /// Returns true if fuel becomes empty during this burn cycle.
    ///
    /// # Arguments
    /// * `dt` - Delta time in seconds
    /// * `consuming` - Whether fuel should be consumed this frame
    pub fn burn(&mut self, dt: f32, consuming: bool) -> bool {
        if !consuming {
            return false;
        }

        let burn_amount = self.burn_rate * dt;
        let was_empty = self.current <= 0.0;

        self.current = (self.current - burn_amount).max(0.0);

        // Return true if fuel becomes empty this frame
        !was_empty && self.current <= 0.0
    }

    /// Refills fuel by the specified amount, capped at maximum.
    ///
    /// # Arguments
    /// * `amount` - Amount of fuel to add
    pub fn refill(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }

    /// Checks if fuel is empty.
    pub fn is_empty(&self) -> bool {
        self.current <= 0.0
    }

    /// Gets the fuel ratio (current/max) for UI display.
    pub fn ratio(&self) -> f32 {
        if self.max <= 0.0 {
            0.0
        } else {
            self.current / self.max
        }
    }
}
