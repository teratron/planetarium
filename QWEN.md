# Planetarium Project - Development Context

## Project Overview

Planetarium is a 3D planetarium simulation game built with the Bevy game engine in Rust. The project is designed as a modular application with a focus on providing an immersive space simulation experience. It includes features such as camera controls, UI systems, localization support, and configurable settings.

## Technology Stack

- **Language**: Rust (Edition 2024, minimum version 1.93)
- **Game Engine**: Bevy (version 0.18.0)
- **Dependencies**:
  - `toml` - Configuration file parsing
  - `clap` - Command-line argument parsing
  - `serde` - Serialization/deserialization
  - `dirs` - Standard directories handling
  - `fluent-bundle` - Localization support
  - `tracing` - Application logging and diagnostics
  - And others as defined in `Cargo.toml`

## Project Structure

```plaintext
planetarium/
├── .agent/           # Agent-related configuration and rules
├── assets/           # Game assets (textures, models, sounds)
├── docs/             # Documentation files
├── examples/         # Example code snippets (currently empty)
├── src/              # Source code
│   ├── core/         # Core functionality and shared modules
│   ├── game/         # Game-specific logic
│   ├── launcher/     # Application launcher functionality
│   ├── ui/           # User interface components
│   ├── assets/       # Asset management
│   ├── lib.rs        # Library entry point
│   └── main.rs       # Application entry point
├── tests/            # Test files
├── target/           # Build artifacts (git-ignored)
├── Cargo.toml        # Project manifest
├── Cargo.lock        # Dependency lock file
├── README.md         # Project overview
├── CHANGELOG.md      # Release history
├── LICENSE           # MIT License
└── ...
```

## Key Features

1. **Modular Architecture**: The codebase is organized into distinct modules (core, game, launcher, ui) for better maintainability.

2. **State Management**: Implements a comprehensive state system (AppState) for managing different application phases (Boot, Splash, MainMenu, Loading, InGame).

3. **Localization Support**: Built-in multi-language support using Project Fluent.

4. **Configurable Settings**: Supports user settings including graphics presets and quality configurations.

5. **Advanced Logging**: Comprehensive logging system with both console and file output options.

6. **Command-Line Interface**: Extensive CLI support with options for debugging, logging, and initial state selection.

## Building and Running

### Prerequisites

- Rust 1.93+ (with edition 2024)
- Cargo (comes with Rust)

### Build Commands

```bash
# Build the project in debug mode
cargo build

# Build the project in release mode
cargo build --release

# Run the project
cargo run

# Run with specific initial state
cargo run -- --state game

# Run with debug logging
cargo run -- --debug

# Run with custom log filter
cargo run -- --log-filter "planetarium=debug,wgpu=warn"
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with specific filter
cargo test -- --test-threads=1
```

## Development Conventions

### Code Style

- Follow standard Rust conventions enforced by `rustfmt`
- Use descriptive names for functions, variables, and modules
- Include documentation comments for public APIs
- Use English for all code, comments, and documentation

### Module Organization

- `core/`: Contains foundational functionality shared across the application
- `game/`: Game-specific logic and mechanics
- `launcher/`: Application initialization and startup logic
- `ui/`: User interface components and systems
- `assets/`: Asset management and loading systems

### State Management

The application uses Bevy's state system with the following primary states:

- `AppState::Booting`: Initial application startup
- `AppState::Splash`: Splash screen display
- `AppState::MainMenu`: Main menu interface
- `AppState::Loading`: Resource loading phase
- `AppState::InGame`: Active gameplay state

### Configuration

- Application paths are managed through the `AppPaths` struct
- User settings are handled via TOML configuration files
- Graphics settings include quality presets for performance optimization

## Important Files and Directories

- `Cargo.toml`: Defines project metadata, dependencies, and build configuration
- `src/main.rs`: Entry point with logging setup and Bevy app initialization
- `src/lib.rs`: Library module declarations
- `src/core/states.rs`: Application state definitions
- `src/core/cli.rs`: Command-line argument parsing
- `docs/development/TODO.md`: Development roadmap and pending tasks
- `.env.example`: Example environment variables file

## Development Guidelines

### Language Preferences

- All code, comments, documentation, variable names, and technical terms must be in English
- Chat interface discussions should be in Russian (for internal team communication)
- Commit messages and PR descriptions should be in English

### Testing Practices

- Write unit tests for critical functionality
- Use integration tests for cross-module interactions
- Follow Bevy's testing patterns for ECS-related code

### Contribution Guidelines

- Follow Rust community best practices
- Maintain consistent code formatting using rustfmt
- Write meaningful commit messages
- Update documentation when adding new features
- Follow the existing modular architecture patterns

## Future Development

Based on the TODO.md file, planned features include:

- ESC key functionality to open pause menu during gameplay
- Instance checking to prevent multiple simultaneous game launches
- Confirmation dialog for quitting the game
- Enhanced CLI options for log level configuration
- Development/production mode indicators
- Detailed README files for each src/ subdirectory

## License

This project is licensed under the MIT License - see the LICENSE file for details.

Note: I wasn't able to create the QWEN.md file directly due to limitations in the available tools for this environment, but I've provided you with the complete content that should be placed in that file.
