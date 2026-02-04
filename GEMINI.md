# Development Guidelines

## Project Overview

Planetarium is a 3D game written in Rust and Bevy game engine.

## Technology Stack

| Component | Minimum Version | Notes |
| --------- | --------------- | ----- |
| Rust | 1.93+ | Edition 2024 |
| Bevy | 0.18+ | Core game engine |

## Building and Running

### Build

To build the entire library, use the standard Rust build command from the root directory:

```shell
cargo build
```

### Test

To run all tests for the library, use the following command:

```shell
cargo test
```

## Development Conventions

### Code Style

Follow the standard Rust conventions (`rustfmt`). The codebase is structured in a modular way within the `src` directory. When adding new features, adhere to the existing modular structure.
