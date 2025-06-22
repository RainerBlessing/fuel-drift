# Fuel Drift

A Rust game built with Macroquad, following clean code principles and maintainable architecture.

## Project Structure

This project uses a Cargo workspace with two crates:

- **`core`** - Pure logic library crate containing all game mechanics without graphics dependencies
- **`game`** - Binary crate that handles graphics and user interface using Macroquad

## Prerequisites

- Rust 1.70.0 or later
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

## Development

### Code Quality

This project follows clean code principles:

- Single Responsibility Principle
- Low cyclomatic complexity
- Testable code architecture
- Clear separation between pure logic (`core`) and graphics (`game`)

### Formatting and Linting

Format code:

```bash
cargo fmt
```

Run linter:

```bash
cargo clippy -- -D warnings
```

### Controls

- **ESC** - Exit the game

## Contributing

1. Ensure all tests pass: `cargo test`
2. Format your code: `cargo fmt`
3. Check for linting issues: `cargo clippy -- -D warnings`
4. Create meaningful commit messages

## License

This project is licensed under the MIT License.