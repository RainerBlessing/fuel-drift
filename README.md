# Fuel Drift

A Rust cave flying game built with Macroquad, following clean code principles and maintainable architecture.

## Project Structure

This project uses a Cargo workspace with two crates:

- **`core`** - Pure logic library crate containing all game mechanics without graphics dependencies
  - `game_state` - State machine for menu, playing, paused, and game over states
  - `player` - Player physics with gravity, thrust, and movement
  - `cave` - Procedural cave generation for endless gameplay
  - `collision` - AABB collision detection system
  - `math` - Mathematical utilities (placeholder for future expansion)
- **`game`** - Binary crate that handles graphics and user interface using Macroquad

## Prerequisites

- Rust 1.60.0 or later
- Cargo (comes with Rust)

## Building

To build the entire workspace:

```bash
cargo build
```

To build in release mode:

```bash
cargo build --release
```

## Running

To run the game:

```bash
cargo run --bin game
```

Or from the game directory:

```bash
cd game
cargo run
```

## Testing

Run all tests:

```bash
cargo test
```

Run tests for a specific crate:

```bash
cargo test --package core
```

## Gameplay

### Controls

**Menu State:**
- **ENTER** - Start game

**Playing State:**
- **↑** - Thrust upward
- **↓** - Thrust downward (reduced power)
- **←** - Move left
- **→** - Move right
- **P** or **ESC** - Pause game

**Paused State:**
- **P** or **ESC** - Resume game
- **R** - Return to menu

**Game Over State:**
- **ENTER** - Restart game
- **R** - Return to menu

### Game Mechanics

- **Physics**: Direct thrust control without gravity
- **Cave Generation**: Procedural endless cave with guaranteed minimum gap
- **Scrolling**: Automatic horizontal scrolling at 120 pixels/second
- **Player**: 30×18 pixel rectangle with thrust-based movement
- **Collision Detection**: AABB collision system with immediate game over on wall contact
- **Visual Feedback**: Red flash effect for 0.3 seconds when collision occurs

### Objective

Navigate through the endless cave without hitting the walls. The cave automatically scrolls, and touching any wall results in immediate game over with visual feedback.

## Development

### Architecture

This project follows clean code principles:

- **Single Responsibility Principle**: Each module and function has one clear purpose
- **Low Cyclomatic Complexity**: Simple control flow with minimal branching
- **Testable Code**: Pure logic separated from graphics for easy unit testing
- **State Management**: Clear state machine with well-defined transitions
- **Collision System**: Pure AABB collision detection with comprehensive test coverage

### Code Quality

Format code:

```bash
cargo fmt
```

Run linter:

```bash
cargo clippy -- -D warnings
```

Run tests:

```bash
cargo test
```

### Testing Strategy

- **Unit Tests**: All core logic is thoroughly tested
- **State Machine**: All valid and invalid state transitions tested
- **Player Physics**: Gravity, thrust, and movement mechanics tested
- **Cave Generation**: Gap constraints and segment continuity tested
- **Collision Detection**: Overlapping and non-overlapping cases extensively tested
- **Integration Tests**: Player-cave collision scenarios and game state transitions
- **Smoke Tests**: Basic arithmetic validation

### Collision Detection Features

The collision system includes:

- **AABB Algorithm**: Axis-Aligned Bounding Box collision detection
- **Edge Cases**: Proper handling of touching boundaries, zero-sized rectangles
- **Floating Point**: Robust handling of floating-point precision
- **Performance**: Efficient collision checking for multiple cave segments
- **Integration**: Seamless integration with game state and visual feedback

## Contributing

1. Ensure all tests pass: `cargo test`
2. Format your code: `cargo fmt`
3. Check for linting issues: `cargo clippy -- -D warnings`
4. Create meaningful commit messages following the established patterns

## License

This project is licensed under the MIT License.