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

/// Tractor beam constants
pub struct TractorBeamConstants;

impl TractorBeamConstants {
    /// Maximum range of the tractor beam in pixels (increased to reach ceiling from player position)
    pub const MAX_RANGE: f32 = 300.0;
    
    /// Width of the tractor beam in pixels (matches visual beam)
    pub const BEAM_WIDTH: f32 = 32.0;
    
    /// Wider area for maintaining attraction once started (prevents oscillation)
    pub const ATTRACTION_HOLD_WIDTH: f32 = 48.0;
    
    /// Speed at which pickups are attracted toward the player (pixels per second)
    pub const ATTRACTION_SPEED: f32 = 200.0;
    
    /// Maximum duration the beam can remain active in seconds
    pub const MAX_DURATION: f32 = 2.0;
}