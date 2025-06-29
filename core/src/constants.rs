/// Game constants module for centralized configuration values.
///
/// Contains all magic numbers and configuration constants used throughout the game.

/// Pickup system constants
pub struct PickupConstants;

impl PickupConstants {
    /// Size of pickup items in pixels
    pub const SIZE: f32 = 20.0;
    
    /// Offset from cave walls when placing pickups
    pub const WALL_OFFSET: f32 = 5.0;
    
    /// Variation percentage for spawn distance randomization (±30%)
    pub const SPAWN_DISTANCE_VARIATION: f32 = 0.3;
    
    /// RNG seed offset to differentiate from cave generation
    pub const RNG_SEED_OFFSET: u32 = 42;
    
    /// Initial delay before first pickup spawns (pixels)
    pub const INITIAL_SPAWN_DELAY: f32 = 800.0;
    
    /// Default fuel spawn distance for collision detection and fallbacks
    pub const DEFAULT_FUEL_SPAWN_DISTANCE: f32 = 300.0;
}

/// Fuel refill constants
pub struct FuelConstants;

impl FuelConstants {
    /// Percentage of max fuel restored when collecting a fuel pickup
    pub const REFILL_PERCENTAGE: f32 = 0.275; // 27.5% (average of 25-30%)
}

/// Cave generation constants
pub struct CaveConstants;

impl CaveConstants {
    /// Default fuel spawn distance for cave operations
    pub const DEFAULT_FUEL_SPAWN_DISTANCE: f32 = 1000.0;
}