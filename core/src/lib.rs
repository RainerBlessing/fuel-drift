/// Pure mathematical operations for the Fuel Drift game.
///
/// This module contains all mathematical computations without any graphics dependencies,
/// following the Single Responsibility Principle for mathematical operations.
pub mod math {
    // Module is intentionally empty as per requirements
    // Future mathematical functions will be added here
}

/// Game state management for the Fuel Drift game.
///
/// Handles state transitions and game flow without graphics dependencies.
pub mod game_state;

/// Player mechanics and physics.
///
/// Pure player logic without rendering concerns.
pub mod player;

/// Cave generation and management.
///
/// Procedural cave generation for endless gameplay.
pub mod cave;
pub mod collision;