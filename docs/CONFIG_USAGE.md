# Configuration System Usage Guide

## Overview

Planetarium uses a dual-layer configuration system:
- **GameConfig**: User-facing settings stored in TOML format
- **DevConfig**: Developer-only settings stored in RON format (debug builds only)

## User Configuration

### Location

Configuration files are stored in platform-specific directories:
- **Windows**: `%APPDATA%\Teratron\Planetarium\config.toml`
- **Linux**: `~/.config/Planetarium/config.toml`
- **macOS**: `~/Library/Application Support/com.Teratron.Planetarium/config.toml`

### Structure

```toml
version = 1

[graphics]
resolution = [1280, 720]
fullscreen = false
vsync = true
quality = "High"  # Options: "Low", "Medium", "High", "Ultra"

[audio]
master_volume = 1.0  # Range: 0.0 - 1.0
music_volume = 0.7
sfx_volume = 0.8

language = "en"  # ISO 639-1 language code
```

### Features

- **Automatic Creation**: Config file is created with defaults on first launch
- **Validation**: Invalid values are automatically clamped to safe ranges
- **Hot Reload**: Press F5 in debug mode to reload config without restarting
- **Atomic Saves**: Uses temporary files to prevent corruption
- **Error Recovery**: Corrupt files are backed up to `.bak` and reset to defaults
- **Version Migration**: Automatic upgrade path for future config versions

## Developer Configuration (Debug Only)

### Location

`assets/config/dev_config.ron`

### Structure

```ron
(
    show_fps: true,
    show_gizmos: false,
    hot_reload: true,
)
```

### Features

- **Debug Only**: Completely stripped from release builds
- **RON Format**: Native Rust Object Notation for developer convenience
- **Auto-reload**: Changes are detected and applied automatically

## Programmatic Usage

### Accessing Configuration

```rust
use bevy::prelude::*;
use planetarium::config::GameConfig;

fn my_system(config: Res<GameConfig>) {
    let (width, height) = config.graphics.resolution;
    let volume = config.audio.master_volume;
    // Use config values...
}
```

### Modifying Configuration

```rust
fn change_settings(mut config: ResMut<GameConfig>) {
    config.graphics.fullscreen = true;
    config.audio.master_volume = 0.5;
    // Changes are automatically saved to disk (Memory Wins strategy)
}
```

### Reactive Systems

The configuration system uses Bevy's change detection:

```rust
fn react_to_config_changes(config: Res<GameConfig>) {
    if config.is_changed() {
        // Config was modified, apply changes
        info!("Configuration updated!");
    }
}
```

## Security Notes

- **Never store secrets** in config files
- Use `.env` files for development secrets (already in `.gitignore`)
- Use environment variables for production secrets
- Config files are user-editable and should only contain safe settings

## Testing

Run configuration tests:

```bash
cargo test --lib config::tests
```

Tests cover:
- Value validation and clamping
- Version migration
- Serialization/deserialization
- Default values
