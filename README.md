# Fuel Drift v0.1.0

A Rust cave flying game built with Macroquad, following clean code principles and maintainable architecture.

## Project Structure

This project uses a Cargo workspace with two crates:

- **`core`** - Pure logic library crate containing all game mechanics without graphics dependencies
  - `game_state` - State machine for menu, playing, paused, and game over states
  - `player` - Player physics with gravity, thrust, and movement
  - `cave` - Procedural cave generation for endless gameplay
  - `collision` - AABB collision detection system
  - `fuel` - Fuel consumption and refilling mechanics
  - `tractor` - Tractor beam system for attraction effects
  - `distance` - Distance tracking for gameplay metrics
  - `audio` - Audio event system for sound effects
- **`game`** - Binary crate that handles graphics, audio, and user interface using Macroquad

## Prerequisites

- Rust 1.60.0 or later
- Cargo (comes with Rust)

### For WASM builds (optional):
- `wasm-pack`: `cargo install wasm-pack`
- `trunk`: `cargo install trunk`
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`

## Building

### Native Build

To build the entire workspace:

```bash
cargo build
```

To build in release mode:

```bash
cargo build --release
```

### WASM Build

To build for WebAssembly and serve locally:

```bash
# Install dependencies (first time only)
rustup target add wasm32-unknown-unknown
cargo install trunk

# Build and serve the WASM version
trunk serve

# Or build for production
trunk build --release
```

The WASM build will be available in the `dist/` directory and can be served by any web server.

## Running

### Native

To run the game natively:

```bash
cargo run --bin fuel-drift
```

Or from the game directory:

```bash
cd game
cargo run
```

### Headless Test (for CI)

Run a 5-second headless test to verify core logic:

```bash
cargo run --bin fuel-drift -- --headless-test
```

### WASM (Web)

After building with `trunk serve`, the game will be available at:
- Local: http://127.0.0.1:8080
- Network: http://[your-ip]:8080

## Testing

Run all tests:

```bash
cargo test
```

Run tests for a specific crate:

```bash
cargo test --package core
```

Run tests with verbose output:

```bash
cargo test -- --nocapture
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
- **W** - Activate upward tractor beam
- **S** - Activate downward tractor beam
- **ESC** - Pause game

**Paused State:**
- **ESC** - Resume game or return to menu
- **R** - Return to menu

**Game Over State:**
- **ENTER** - Restart game
- **R** - Return to menu

### Game Mechanics

- **Physics**: Direct thrust control without gravity
- **Cave Generation**: Procedural endless cave with guaranteed minimum gap
- **Scrolling**: Automatic horizontal scrolling at 120 pixels/second
- **Player**: 30×18 pixel rectangle with thrust-based movement
- **Fuel System**: Limited fuel that depletes during movement
- **Tractor Beam**: Limited-duration beam for attraction effects
- **Collision Detection**: AABB collision system with immediate game over on wall contact
- **Audio**: Sound effects for thruster, beam activation, fuel events, and death
- **Visual Feedback**: Red flash effect for 0.3 seconds when collision occurs
- **Distance Tracking**: Real-time distance measurement displayed on screen

### Objective

Navigate through the endless cave without hitting the walls or running out of fuel. The cave automatically scrolls, and touching any wall or depleting your fuel results in immediate game over with visual and audio feedback.

## Development

### Architecture

This project follows clean code principles:

- **Single Responsibility Principle**: Each module and function has one clear purpose
- **Low Cyclomatic Complexity**: Simple control flow with minimal branching
- **Testable Code**: Pure logic separated from graphics for easy unit testing
- **State Management**: Clear state machine with well-defined transitions
- **Collision System**: Pure AABB collision detection with comprehensive test coverage
- **Audio System**: Event-driven audio with separation of concerns

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
- **Fuel System**: Consumption, refilling, and empty state detection tested
- **Tractor Beam**: Activation, timing, and deactivation tested
- **Distance Tracking**: Distance accumulation and formatting tested
- **Audio System**: Event queuing and state management tested
- **Integration Tests**: Player-cave collision scenarios and game state transitions
- **Headless Tests**: Core logic validation for CI environments
- **Smoke Tests**: Basic arithmetic validation

### Audio System Features

The audio system includes:

- **Event-Driven Architecture**: Audio events are queued and processed separately
- **Looping Sounds**: Thruster sound loops while active
- **One-Shot Effects**: Beam activation, fuel pickup, death, and UI sounds
- **State Management**: Prevents audio overlap and manages looping sounds
- **Stub Implementation**: Audio stubs for development and testing

### Collision Detection Features

The collision system includes:

- **AABB Algorithm**: Axis-Aligned Bounding Box collision detection
- **Edge Cases**: Proper handling of touching boundaries, zero-sized rectangles
- **Floating Point**: Robust handling of floating-point precision
- **Performance**: Efficient collision checking for multiple cave segments
- **Integration**: Seamless integration with game state and visual feedback

## Deployment

### Native Distribution

Build release binaries:

```bash
cargo build --release
```

Binaries will be in `target/release/`:
- Linux: `fuel-drift`
- Windows: `fuel-drift.exe`
- macOS: `fuel-drift`

### Web Distribution

Build WASM for web deployment:

```bash
trunk build --release
```

Deploy the `dist/` directory to any web server that supports:
- Static file serving
- WASM MIME type (`application/wasm`)
- Cross-Origin-Embedder-Policy headers (recommended)

### CI/CD

The project includes GitHub Actions workflow (`.github/workflows/ci.yml`) that:
- Runs on Ubuntu latest
- Caches Cargo registry and build artifacts
- Checks formatting with `cargo fmt`
- Runs linter with `cargo clippy`
- Executes all tests with `cargo test`
- Runs headless test for smoke testing

## Version History

### v0.1.0 (Current)
- Initial release
- Core game mechanics: player physics, cave generation, collision detection
- Fuel system with consumption and empty state detection
- Tractor beam system with timing and visual effects
- Distance tracking and display
- Audio system with event-driven sound effects
- Complete menu system with state management
- WASM support for web deployment
- Comprehensive test suite with 95%+ coverage
- Headless test mode for CI environments

## Contributing

1. Ensure all tests pass: `cargo test`
2. Format your code: `cargo fmt`
3. Check for linting issues: `cargo clippy -- -D warnings`
4. Run headless test: `cargo run --bin fuel-drift -- --headless-test`
5. Test WASM build: `trunk build`
6. Create meaningful commit messages following the established patterns

### Commit Message Format

```
feat: add tractor beam visual effects

- Implement beam rendering with transparency
- Add beam collision detection with cave walls
- Update UI indicator for beam status
- Add comprehensive test coverage for beam mechanics

Resolves: #123
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) and [Macroquad](https://macroquad.rs/)
- Inspired by classic cave flying games
- Follows clean code principles from Robert C. Martin
- Uses procedural generation techniques for endless gameplay