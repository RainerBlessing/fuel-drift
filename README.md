# Fuel Drift

A Rust cave flying game built with Macroquad, following clean code principles and maintainable architecture.

## Project Structure

This project uses a Cargo workspace with two crates:

- **`core`** - Pure logic library crate containing all game mechanics without graphics dependencies
  - `game_state` - State machine for menu, playing, paused, and game over states
  - `player` - Player physics with gravity, thrust, and movement
  - `cave` - Procedural cave generation for endless gameplay
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

**Game Over State:**
- **ENTER** - Restart game
- **R** - Return to menu

### Game Mechanics

- **Physics**: Direct thrust control without gravity
- **Cave Generation**: Procedural endless cave with guaranteed minimum gap
- **Scrolling**: Automatic horizontal scrolling at 120 pixels/second
- **Player**: 30×18 pixel rectangle with thrust-based movement

## Development

### Architecture

This project follows clean code principles:

- **Single Responsibility Principle**: Each module and function has one clear purpose
- **Low Cyclomatic Complexity**: Simple control flow with minimal branching
- **Testable Code**: Pure logic separated from graphics for easy unit testing
- **State Management**: Clear state machine with well-defined transitions

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
- **Smoke Tests**: Basic arithmetic validation

## Contributing

1. Ensure all tests pass: `cargo test`
2. Format your code: `cargo fmt`
3. Check for linting issues: `cargo clippy -- -D warnings`
4. Create meaningful commit messages following the established patterns

## License

This project is licensed under the MIT License.