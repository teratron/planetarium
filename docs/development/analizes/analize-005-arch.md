# Planetarium Framework: Architecture & Asset Management Guide

**AI Agent Reference Document**  
**Version:** 1.0  
**Last Updated:** 2026-02-15

This document provides comprehensive architectural guidelines for the Planetarium project, covering workspace structure, crate organization, asset management, and migration strategies.

## Table of Contents

1. [Overview](#overview)
2. [Project Structure](#project-structure)
3. [Workspace Configuration](#workspace-configuration)
4. [Crate Architecture](#crate-architecture)
5. [Asset Management](#asset-management)
6. [Build System](#build-system)
7. [Migration Guide](#migration-guide)
8. [Project Templates](#project-templates)
9. [Best Practices](#best-practices)
10. [Code Examples](#code-examples)

## Overview

### Architecture Principles

The Planetarium project follows a **two-layer architecture**:

1. **Framework Layer** (`crates/*`) - Reusable components for any Bevy game
2. **Game Layer** (`src/`) - Planetarium-specific game logic

### Key Benefits

✅ **Reusability** - Framework components can be used in other projects  
✅ **Isolation** - Framework knows nothing about game-specific logic  
✅ **Testability** - Each crate can be tested independently  
✅ **Modularity** - Clear separation of concerns  
✅ **Distribution** - Framework crates can be published to crates.io  
✅ **Rapid Prototyping** - New games can be created in minutes

### Architecture Layers

```
┌─────────────────────────────────────────────────┐
│         GAME LAYER (src/)                       │
│  ├── Planetarium-specific logic                 │
│  ├── Planet physics & orbits                    │
│  ├── Game UI (HUD, info panels)                 │
│  └── Game assets (models, textures, sounds)     │
└─────────────────────────────────────────────────┘
                      ▼ uses
┌─────────────────────────────────────────────────┐
│      FRAMEWORK LAYER (crates/*)                 │
│  ├── launcher/    - Boot, splash, loading       │
│  ├── menu/        - Settings, UI widgets        │
│  ├── theme/       - Colors, fonts, styling      │
│  ├── localization/- Fluent translations         │
│  └── transitions/ - Screen fading, effects      │
└─────────────────────────────────────────────────┘
                      ▼ uses
┌─────────────────────────────────────────────────┐
│             BEVY ENGINE                         │
└─────────────────────────────────────────────────┘
```

## Project Structure

### Recommended Directory Layout

```
planetarium/
├── Cargo.toml                      # Workspace root manifest
├── README.md
├── LICENSE
├── .gitignore
│
├── crates/                         # 🔧 FRAMEWORK LAYER (reusable)
│   │
│   ├── launcher/                   # Application lifecycle management
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   ├── ASSETS.md              # Asset documentation
│   │   ├── build.rs               # Asset copying script
│   │   ├── assets/                # Framework assets
│   │   │   ├── fonts/
│   │   │   │   └── fallback.ttf   # Embedded
│   │   │   └── locales/
│   │   │       ├── en-US/
│   │   │       │   └── launcher.ftl
│   │   │       └── ru-RU/
│   │   │           └── launcher.ftl
│   │   ├── examples/
│   │   │   └── basic_launcher.rs
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── boot/
│   │       │   ├── mod.rs
│   │       │   ├── config.rs
│   │       │   └── paths.rs
│   │       ├── splash/
│   │       │   ├── mod.rs
│   │       │   └── player.rs
│   │       ├── loading/
│   │       │   ├── mod.rs
│   │       │   └── tracker.rs
│   │       ├── diagnostics/
│   │       │   ├── mod.rs
│   │       │   └── overlay.rs
│   │       └── embedded.rs        # Embedded assets
│   │
│   ├── menu/                       # Menu system with settings
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   ├── ASSETS.md
│   │   ├── build.rs
│   │   ├── assets/
│   │   │   ├── audio/ui/          # UI sounds
│   │   │   │   ├── click.ogg      # Embedded
│   │   │   │   ├── hover.ogg      # Embedded
│   │   │   │   ├── back.ogg
│   │   │   │   ├── error.ogg
│   │   │   │   └── scroll.ogg
│   │   │   ├── icons/             # UI icons (32x32)
│   │   │   │   ├── play.png
│   │   │   │   ├── settings.png
│   │   │   │   ├── exit.png
│   │   │   │   ├── volume.png
│   │   │   │   └── fullscreen.png
│   │   │   └── locales/
│   │   │       ├── en-US/
│   │   │       │   └── menu.ftl
│   │   │       └── ru-RU/
│   │   │           └── menu.ftl
│   │   ├── examples/
│   │   │   └── simple_menu.rs
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── widgets/
│   │       │   ├── mod.rs
│   │       │   ├── button.rs
│   │       │   ├── slider.rs
│   │       │   └── dropdown.rs
│   │       ├── settings/
│   │       │   ├── mod.rs
│   │       │   ├── graphics.rs
│   │       │   ├── audio.rs
│   │       │   └── controls.rs
│   │       └── embedded.rs
│   │
│   ├── theme/                      # Theming system
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   ├── build.rs
│   │   ├── assets/
│   │   │   ├── fonts/
│   │   │   │   ├── ui-regular.ttf
│   │   │   │   └── ui-bold.ttf
│   │   │   └── textures/ui/
│   │   │       ├── button.9.png
│   │   │       └── panel.9.png
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── colors.rs
│   │       ├── fonts.rs
│   │       ├── metrics.rs
│   │       └── embedded.rs
│   │
│   ├── localization/               # Fluent localization system
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── fluent.rs
│   │       ├── bundle.rs
│   │       └── components.rs
│   │
│   └── transitions/                # Screen transitions (fading, etc)
│       ├── Cargo.toml
│       ├── README.md
│       └── src/
│           ├── lib.rs
│           ├── fade.rs
│           └── wipe.rs
│
├── src/                            # 🎮 GAME LAYER (Planetarium-specific)
│   ├── main.rs                     # Application entry point
│   ├── lib.rs                      # Game library
│   ├── world/
│   │   ├── mod.rs
│   │   ├── planet.rs
│   │   └── camera.rs
│   ├── physics/
│   │   ├── mod.rs
│   │   ├── gravity.rs
│   │   └── orbits.rs
│   └── ui/
│       └── hud.rs
│
├── assets/                         # 🎨 GAME ASSETS (Planetarium-specific)
│   ├── models/
│   │   └── planets/
│   │       ├── earth.glb
│   │       ├── mars.glb
│   │       └── jupiter.glb
│   ├── textures/
│   │   └── planets/
│   │       ├── earth_diffuse.png
│   │       ├── earth_normal.png
│   │       └── starfield.hdr
│   ├── audio/
│   │   ├── music/
│   │   │   └── ambient_space.ogg
│   │   └── sfx/
│   │       ├── warp.ogg
│   │       └── collision.ogg
│   ├── shaders/
│   │   └── planet_atmosphere.wgsl
│   └── locales/
│       ├── en-US/
│       │   ├── planets.ftl
│       │   └── tutorial.ftl
│       └── ru-RU/
│           ├── planets.ftl
│           └── tutorial.ftl
│
├── templates/                      # Project templates
│   └── game-template/
│       ├── Cargo.toml
│       ├── src/
│       │   └── main.rs
│       └── README.md
│
├── docs/
│   ├── architecture.md
│   └── migration.md
│
└── target/
    └── assets/                     # 🎯 Assembled assets (build-time)
        ├── framework/              # From crates/*/assets/
        │   ├── launcher/
        │   ├── menu/
        │   └── theme/
        └── game/                   # From assets/
            ├── models/
            ├── textures/
            └── audio/
```

### Key Directories

| Directory | Purpose | Layer |
|-----------|---------|-------|
| `crates/` | Reusable framework components | Framework |
| `src/` | Planetarium game logic | Game |
| `assets/` | Game-specific assets | Game |
| `crates/*/assets/` | Framework assets | Framework |
| `target/assets/` | Assembled assets (build-time) | Build |
| `templates/` | Project templates for new games | Template |

## Workspace Configuration

### Root `Cargo.toml`

```toml
[workspace]
resolver = "2"
members = [
    "crates/launcher",
    "crates/menu",
    "crates/localization",
    "crates/theme",
    "crates/transitions",
]

# Default members - only build game by default
default-members = ["."]

[package]
name = "planetarium"
version = "0.3.2"
edition = "2024"
rust-version = "1.93"
license = "MIT"
authors = ["Oleg Alexandrov <alexandrovoleg.ru@gmail.com>"]
description = "A planetarium simulation built with Bevy"
keywords = ["bevy", "game", "simulation", "planetarium"]
build = "build.rs"

[workspace.package]
version = "0.3.2"
edition = "2024"
rust-version = "1.93"
license = "MIT"
authors = ["Oleg Alexandrov <alexandrovoleg.ru@gmail.com>"]

[workspace.dependencies]
# Core dependencies
bevy = { version = "0.18.0", default-features = true }
serde = { version = "1.0", features = ["derive"] }

# Framework crates (internal)
launcher = { path = "crates/launcher" }
menu = { path = "crates/menu" }
localization = { path = "crates/localization" }
theme = { path = "crates/theme" }
transitions = { path = "crates/transitions" }

# Specialized dependencies
fluent-bundle = "0.16.0"
intl-memoizer = "0.5"
unic-langid = "0.9.5"
clap = { version = "4.5.58", features = ["derive"] }

[dependencies]
# Bevy
bevy.workspace = true
serde.workspace = true

# Framework crates
launcher.workspace = true
menu.workspace = true
localization.workspace = true
theme.workspace = true
transitions.workspace = true

# CLI
clap.workspace = true

[dev-dependencies]
proptest = "1.10.0"
tempfile = "3.14.0"

[features]
default = []
dev = ["launcher/diagnostics"]  # Enable diagnostics in dev mode

[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3

[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
strip = true
panic = "abort"

[build-dependencies]
glob = "0.3"
```

### Framework Crate `Cargo.toml` Template

Example: `crates/launcher/Cargo.toml`

```toml
[package]
name = "launcher"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
authors.workspace = true
description = "Reusable launcher framework for Bevy games"
keywords = ["bevy", "game", "launcher", "framework"]
categories = ["game-development"]
build = "build.rs"

[dependencies]
bevy.workspace = true
serde.workspace = true
clap.workspace = true

# Optional internal dependencies
theme = { workspace = true, optional = true }
localization = { workspace = true, optional = true }

[dev-dependencies]
bevy = { workspace = true, features = ["bevy_dev_tools"] }

[features]
default = ["theme", "localization"]
theme = ["dep:theme"]
localization = ["dep:localization"]
diagnostics = []  # Enable debug overlay

[build-dependencies]
glob = "0.3"
```

## Crate Architecture

### Framework Crates Overview

#### 1. `launcher` - Application Lifecycle

**Responsibility:**

- Boot sequence (paths, config, metadata)
- Splash screen system
- Asset loading with progress tracking
- Diagnostics overlay (FPS, entity count)

**Public API:**

```rust
pub struct LauncherPlugin {
    pub skip_splash: bool,
    pub initial_state: Option<String>,
}

pub mod prelude {
    pub use crate::boot::{BootPlugin, BootConfig, AppPaths};
    pub use crate::splash::{SplashPlugin, SplashSequence};
    pub use crate::loading::{LoadingPlugin, LoadingTracker};
    pub use crate::LauncherPlugin;
}
```

**Does NOT know about:**

- Game-specific logic
- Planets, physics, orbits
- Game assets

#### 2. `menu` - Menu System

**Responsibility:**

- Main menu layout
- Settings screens (Graphics, Audio, Controls)
- UI widgets (buttons, sliders, dropdowns)
- Reactive settings updates

**Public API:**

```rust
pub struct MenuPlugin {
    pub show_main_menu: bool,
    pub default_tab: SettingsTab,
}

pub mod widgets {
    pub use crate::widgets::{Button, Slider, Dropdown};
}

pub mod settings {
    pub use crate::settings::{
        GraphicsSettings,
        AudioSettings,
        ControlSettings,
    };
}
```

**Does NOT know about:**

- Game-specific menu items
- Planet selection UI
- Game HUD

#### 3. `theme` - Theming System

**Responsibility:**

- Color schemes
- Font management with fallbacks
- UI metrics (spacing, border radius)
- Embedded fallback assets

**Public API:**

```rust
pub struct ThemePlugin;

#[derive(Resource)]
pub struct Theme {
    pub colors: ColorScheme,
    pub fonts: FontCollection,
    pub metrics: UiMetrics,
}

pub struct ColorScheme {
    pub background: Color,
    pub surface: Color,
    pub accent: Color,
    pub text: Color,
    // ...
}
```

#### 4. `localization` - Translation System

**Responsibility:**

- Fluent bundle management
- Language switching
- Runtime text updates
- Locale-specific formatting

**Public API:**

```rust
pub struct LocalizationPlugin {
    pub default_locale: String,
    pub supported_locales: Vec<String>,
}

#[derive(Component)]
pub struct LocalizedText {
    pub key: String,
    pub args: Option<HashMap<String, String>>,
}
```

#### 5. `transitions` - Screen Transitions

**Responsibility:**

- Fade in/out effects
- Wipe transitions
- Custom transition animations

**Public API:**

```rust
pub struct TransitionsPlugin;

pub enum TransitionType {
    Fade { duration: f32 },
    Wipe { direction: WipeDirection, duration: f32 },
}

pub fn trigger_transition(
    transition: TransitionType,
    next_state: AppState,
) -> impl Command;
```

### Game Layer (`src/`)

**Responsibility:**

- Planetarium-specific logic
- Planet physics & orbital mechanics
- Camera controls
- Game HUD and info panels
- Game-specific assets

**Structure:**

```rust
// src/main.rs
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        
        // Framework layer
        .add_plugins((
            LauncherPlugin::default(),
            MenuPlugin::default(),
            ThemePlugin,
            LocalizationPlugin::default(),
            TransitionsPlugin,
        ))
        
        // Game layer
        .add_plugins((
            WorldPlugin,
            PhysicsPlugin,
            HudPlugin,
        ))
        
        .run();
}

// src/world/mod.rs
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_world)
            .add_systems(Update, (
                update_planet_positions,
                handle_camera_controls,
            ).run_if(in_state(AppState::InGame)));
    }
}
```

## Asset Management

### Asset Categories

#### Framework Assets (Reusable)

| Category | Location | Embedded | Usage |
|----------|----------|----------|-------|
| **Fonts** | `crates/theme/assets/fonts/` | fallback.ttf | UI text rendering |
| **UI Audio** | `crates/menu/assets/audio/ui/` | click.ogg, hover.ogg | Button interactions |
| **Icons** | `crates/menu/assets/icons/` | No | Menu buttons |
| **UI Textures** | `crates/theme/assets/textures/ui/` | No | Buttons, panels (9-patch) |
| **Locales** | `crates/*/assets/locales/` | No | Framework UI strings |

#### Game Assets (Specific to Planetarium)

| Category | Location | Embedded | Usage |
|----------|----------|----------|-------|
| **3D Models** | `assets/models/planets/` | No | Planet meshes |
| **Textures** | `assets/textures/planets/` | No | Planet materials |
| **Audio** | `assets/audio/music/`, `assets/audio/sfx/` | No | Game sounds |
| **Shaders** | `assets/shaders/` | No | Custom rendering |
| **Locales** | `assets/locales/` | No | Game-specific strings |

### Asset Loading Strategy

#### 1. Build-Time Asset Assembly

Each crate with assets includes a `build.rs` script that copies assets to `target/assets/`:

**Example: `crates/launcher/build.rs`**

```rust
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=assets");
    
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let workspace_root = Path::new(&manifest_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    
    let target_dir = workspace_root
        .join("target")
        .join("assets")
        .join("framework")
        .join("launcher");
    
    fs::create_dir_all(&target_dir).unwrap();
    copy_dir_recursive("assets", &target_dir).unwrap();
    
    println!("cargo:warning=Copied launcher assets to {:?}", target_dir);
}

fn copy_dir_recursive(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dst_path = dst.as_ref().join(entry.file_name());
        
        if ty.is_dir() {
            copy_dir_recursive(entry.path(), dst_path)?;
        } else {
            fs::copy(entry.path(), dst_path)?;
        }
    }
    
    Ok(())
}
```

**Root `build.rs` (for game assets):**

```rust
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=assets");
    
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let target_dir = Path::new(&manifest_dir)
        .join("target")
        .join("assets")
        .join("game");
    
    fs::create_dir_all(&target_dir).unwrap();
    copy_dir_recursive("assets", &target_dir).unwrap();
}

// ... copy_dir_recursive as above
```

#### 2. Embedded Critical Assets

Framework crates embed critical assets to ensure functionality even if `assets/` folder is missing:

**Example: `crates/theme/src/embedded.rs`**

```rust
//! Embedded assets for theme system
//!
//! These assets are compiled into the binary to ensure
//! the framework works even without an assets folder.

use bevy::prelude::*;

/// Embedded fallback font (always available)
pub const EMBEDDED_FONT: &[u8] = include_bytes!("../assets/fonts/fallback.ttf");

/// Load embedded assets into Bevy
pub fn load_embedded_assets(
    mut commands: Commands,
    mut fonts: ResMut<Assets<Font>>,
) {
    let font = Font::try_from_bytes(EMBEDDED_FONT.to_vec())
        .expect("Embedded font is valid");
    
    let font_handle = fonts.add(font);
    
    commands.insert_resource(EmbeddedAssets {
        fallback_font: font_handle,
    });
    
    info!("[Theme] Embedded assets loaded");
}

#[derive(Resource)]
pub struct EmbeddedAssets {
    pub fallback_font: Handle<Font>,
}
```

**Usage in theme plugin:**

```rust
// crates/theme/src/lib.rs
pub struct ThemePlugin;

impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, load_embedded_assets)
            .add_systems(Startup, setup_theme);
    }
}

fn setup_theme(
    asset_server: Res<AssetServer>,
    embedded: Res<EmbeddedAssets>,
    mut theme: ResMut<Theme>,
) {
    // Try loading from disk first
    let main_font = asset_server.load("framework/theme/fonts/ui-regular.ttf");
    
    // Check if loaded successfully
    match asset_server.get_load_state(main_font.id()) {
        Some(LoadState::Loaded) => {
            theme.fonts.main = main_font;
            info!("[Theme] Loaded font from disk");
        }
        _ => {
            // Fallback to embedded
            theme.fonts.main = embedded.fallback_font.clone();
            warn!("[Theme] Using embedded fallback font");
        }
    }
}
```

#### 3. Asset Paths Convention

All asset loading uses the following path structure:

```rust
// Framework assets (from crates/*/assets/)
asset_server.load("framework/<crate_name>/<path>")

// Examples:
asset_server.load("framework/launcher/fonts/fallback.ttf")
asset_server.load("framework/menu/audio/ui/click.ogg")
asset_server.load("framework/theme/textures/ui/button.9.png")

// Game assets (from assets/)
asset_server.load("game/<path>")

// Examples:
asset_server.load("game/models/planets/earth.glb")
asset_server.load("game/textures/planets/earth_diffuse.png")
asset_server.load("game/audio/music/ambient_space.ogg")
```

#### 4. AssetPlugin Configuration

Configure Bevy's `AssetPlugin` to use the assembled assets folder:

```rust
// src/main.rs
use std::path::PathBuf;

fn main() {
    let assets_path = get_assets_path();
    
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: assets_path.to_string_lossy().to_string(),
            ..default()
        }))
        .run();
}

fn get_assets_path() -> PathBuf {
    if cfg!(debug_assertions) {
        // Dev mode: use target/assets
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("assets")
    } else {
        // Release: assets folder next to executable
        PathBuf::from("assets")
    }
}
```

### Asset Manifests

Each crate with assets should include a manifest documenting its assets:

**Example: `crates/menu/ASSETS.md`**

```markdown
# Menu Crate Assets

## Audio (framework/menu/audio/ui/)

| File | Size | Embedded | Description |
|------|------|----------|-------------|
| `click.ogg` | 8KB | Yes | Button click sound |
| `hover.ogg` | 6KB | Yes | Hover sound |
| `back.ogg` | 7KB | No | Back navigation |
| `error.ogg` | 8KB | No | Error notification |
| `scroll.ogg` | 5KB | No | Scroll interaction |

## Icons (framework/menu/icons/)

| File | Size | Description |
|------|------|-------------|
| `play.png` | 2KB | Play button icon (32x32) |
| `settings.png` | 2KB | Settings icon (32x32) |
| `exit.png` | 2KB | Exit button icon (32x32) |
| `volume.png` | 2KB | Volume control icon (32x32) |
| `fullscreen.png` | 2KB | Fullscreen toggle icon (32x32) |

## Locales (framework/menu/locales/)

| Language | File | Keys |
|----------|------|------|
| English | `en-US/menu.ftl` | 25 |
| Russian | `ru-RU/menu.ftl` | 25 |

## Total Size: ~55KB
```

## Build System

### Build Scripts Overview

The build system uses `build.rs` scripts to:

1. Copy assets from crates to `target/assets/`
2. Validate asset integrity
3. Generate asset manifests
4. Handle platform-specific paths

### Build Dependencies

Add to each crate's `Cargo.toml`:

```toml
[build-dependencies]
glob = "0.3"          # For recursive file discovery
walkdir = "2"         # Alternative to glob
```

### Advanced Build Script

**Example: `crates/menu/build.rs`**

```rust
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    // Rebuild if assets change
    println!("cargo:rerun-if-changed=assets");
    
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_root = Path::new(&manifest_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    
    let src_dir = Path::new(&manifest_dir).join("assets");
    let dst_dir = workspace_root
        .join("target")
        .join("assets")
        .join("framework")
        .join("menu");
    
    // Create destination
    fs::create_dir_all(&dst_dir).unwrap();
    
    // Copy assets
    match copy_assets(&src_dir, &dst_dir) {
        Ok(count) => {
            println!("cargo:warning=Copied {} menu assets to target/assets", count);
        }
        Err(e) => {
            println!("cargo:warning=Failed to copy menu assets: {}", e);
        }
    }
    
    // Validate critical assets
    validate_critical_assets(&dst_dir);
}

fn copy_assets(src: &Path, dst: &Path) -> std::io::Result<usize> {
    let mut count = 0;
    
    for entry in walkdir::WalkDir::new(src).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        
        if path.is_file() {
            let relative = path.strip_prefix(src).unwrap();
            let dst_path = dst.join(relative);
            
            // Create parent directories
            if let Some(parent) = dst_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            // Copy file
            fs::copy(path, &dst_path)?;
            count += 1;
        }
    }
    
    Ok(count)
}

fn validate_critical_assets(assets_dir: &Path) {
    let critical = vec![
        "audio/ui/click.ogg",
        "audio/ui/hover.ogg",
        "icons/play.png",
        "icons/settings.png",
    ];
    
    for asset in critical {
        let path = assets_dir.join(asset);
        if !path.exists() {
            println!("cargo:warning=MISSING CRITICAL ASSET: {}", asset);
        }
    }
}
```

### Release Build Script

For release builds, copy assembled assets next to the executable:

**Root `build.rs` addition:**

```rust
fn main() {
    // ... existing code ...
    
    // In release mode, prepare assets for distribution
    if env::var("PROFILE").unwrap() == "release" {
        prepare_release_assets();
    }
}

fn prepare_release_assets() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    
    let assembled_assets = target_dir.join("assets");
    let release_assets = target_dir.join("release").join("assets");
    
    if assembled_assets.exists() {
        fs::create_dir_all(&release_assets).ok();
        copy_dir_recursive(&assembled_assets, &release_assets).ok();
        println!("cargo:warning=Prepared release assets");
    }
}
```

## Migration Guide

### Phase 1: Workspace Setup (Week 1)

**Goal:** Create workspace structure without moving code.

**Steps:**

1. **Create workspace structure:**

```bash
mkdir -p crates/launcher
mkdir -p crates/menu
mkdir -p crates/theme
mkdir -p crates/localization
mkdir -p crates/transitions
```

1. **Create workspace `Cargo.toml`:**

Follow the structure shown in [Workspace Configuration](#workspace-configuration).

1. **Initialize crates:**

```bash
cd crates/launcher
cargo init --lib
cd ../menu
cargo init --lib
# ... repeat for other crates
```

1. **Test workspace builds:**

```bash
cd ../../  # back to root
cargo build --workspace
```

### Phase 2: Migrate Launcher (Week 2)

**Goal:** Extract launcher code into its own crate.

**Steps:**

1. **Move code:**

```bash
# From src/launcher/* to crates/launcher/src/
mv src/launcher/boot crates/launcher/src/
mv src/launcher/splash crates/launcher/src/
mv src/launcher/loading crates/launcher/src/
mv src/launcher/diagnostics crates/launcher/src/
```

1. **Move assets:**

```bash
# Create assets directory
mkdir -p crates/launcher/assets

# Move launcher-specific assets (if any)
# Currently minimal, mostly just locales
```

1. **Update imports:**

```rust
// Before (in src/):
use crate::launcher::boot::BootPlugin;

// After (in src/):
use launcher::boot::BootPlugin;
```

1. **Create `lib.rs`:**

```rust
// crates/launcher/src/lib.rs
pub mod boot;
pub mod splash;
pub mod loading;

#[cfg(feature = "diagnostics")]
pub mod diagnostics;

pub mod prelude {
    pub use crate::boot::{BootPlugin, BootConfig, AppPaths};
    pub use crate::splash::{SplashPlugin, SplashSequence};
    pub use crate::loading::{LoadingPlugin, LoadingTracker};
}
```

1. **Update `Cargo.toml`:**

```toml
# Root Cargo.toml
[dependencies]
launcher = { path = "crates/launcher" }
```

1. **Test:**

```bash
cargo build
cargo run
```

### Phase 3: Migrate Menu (Week 3)

Similar process as launcher:

1. Move `src/launcher/menu/*` → `crates/menu/src/`
2. Move `assets/audio/ui/*` → `crates/menu/assets/audio/ui/`
3. Move `assets/icons/*` → `crates/menu/assets/icons/`
4. Create `build.rs` for asset copying
5. Update imports
6. Test

### Phase 4: Migrate Theme (Week 3-4)

1. Move `src/ui/theme/*` → `crates/theme/src/`
2. Move `assets/fonts/*` → `crates/theme/assets/fonts/`
3. Create embedded assets module
4. Create `build.rs`
5. Update imports
6. Test

### Phase 5: Migrate Localization (Week 4)

1. Move `src/core/localization/*` → `crates/localization/src/`
2. Update framework locale paths
3. Keep game locales in `assets/locales/`
4. Test language switching

### Phase 6: Migrate Transitions (Week 4)

1. Move `src/ui/fading.rs` → `crates/transitions/src/fade.rs`
2. Expand with additional transition types
3. Update imports
4. Test

### Phase 7: Cleanup & Documentation (Week 5)

1. Remove old code from `src/`
2. Add README.md to each crate
3. Add examples to each crate
4. Write ASSETS.md for each crate
5. Update root README.md
6. Run full test suite

### Migration Checklist

```markdown
## Launcher Crate
- [ ] Code moved to crates/launcher/src/
- [ ] Assets moved to crates/launcher/assets/
- [ ] build.rs created
- [ ] lib.rs with public API
- [ ] Cargo.toml configured
- [ ] Imports updated in game code
- [ ] Builds successfully
- [ ] Examples work
- [ ] README.md written
- [ ] ASSETS.md written

## Menu Crate
- [ ] Code moved
- [ ] Assets moved
- [ ] build.rs created
- [ ] Embedded assets configured
- [ ] lib.rs with public API
- [ ] Cargo.toml configured
- [ ] Imports updated
- [ ] Builds successfully
- [ ] Examples work
- [ ] Documentation complete

## Theme Crate
- [ ] Code moved
- [ ] Assets moved
- [ ] Embedded fallbacks
- [ ] build.rs created
- [ ] lib.rs with public API
- [ ] Cargo.toml configured
- [ ] Imports updated
- [ ] Builds successfully
- [ ] Examples work
- [ ] Documentation complete

## Localization Crate
- [ ] Code moved
- [ ] Framework locales separated
- [ ] Game locales remain in assets/
- [ ] lib.rs with public API
- [ ] Cargo.toml configured
- [ ] Imports updated
- [ ] Language switching works
- [ ] Documentation complete

## Transitions Crate
- [ ] Code moved
- [ ] Additional effects added
- [ ] lib.rs with public API
- [ ] Cargo.toml configured
- [ ] Imports updated
- [ ] Transitions work correctly
- [ ] Documentation complete

## Final Cleanup
- [ ] Old code removed from src/
- [ ] All imports updated
- [ ] Full workspace builds
- [ ] All examples run
- [ ] Tests pass
- [ ] Documentation complete
- [ ] Template created
```

## Project Templates

### Game Template Structure

Create a template for new games that use the Planetarium framework:

```
templates/game-template/
├── .gitignore
├── Cargo.toml
├── README.md
├── src/
│   └── main.rs
└── assets/
    ├── models/
    ├── textures/
    ├── audio/
    └── locales/
        ├── en-US/
        └── ru-RU/
```

### Template `Cargo.toml`

```toml
[package]
name = "my-game"
version = "0.1.0"
edition = "2024"
rust-version = "1.93"
license = "MIT"

[dependencies]
bevy = "0.18.0"

# Planetarium framework crates
launcher = { git = "https://github.com/teratron/planetarium", package = "launcher" }
menu = { git = "https://github.com/teratron/planetarium", package = "menu" }
theme = { git = "https://github.com/teratron/planetarium", package = "theme" }
localization = { git = "https://github.com/teratron/planetarium", package = "localization" }
transitions = { git = "https://github.com/teratron/planetarium", package = "transitions" }

[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3

[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
strip = true
```

### Template `src/main.rs`

```rust
//! My Bevy Game
//!
//! Built with Planetarium Framework

use bevy::prelude::*;
use launcher::prelude::*;
use menu::MenuPlugin;
use theme::ThemePlugin;
use localization::LocalizationPlugin;
use transitions::TransitionsPlugin;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AppState {
    #[default]
    Booting,
    Splash,
    MainMenu,
    InGame,
}

fn main() {
    App::new()
        // Bevy defaults
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "My Game".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        
        // State management
        .init_state::<AppState>()
        
        // Framework layer (reusable from Planetarium)
        .add_plugins((
            LauncherPlugin::default(),
            MenuPlugin::default(),
            ThemePlugin,
            LocalizationPlugin::default(),
            TransitionsPlugin,
        ))
        
        // Game layer (your game logic)
        .add_systems(OnEnter(AppState::InGame), setup_game)
        .add_systems(Update, update_game.run_if(in_state(AppState::InGame)))
        
        .run();
}

fn setup_game(mut commands: Commands) {
    info!("Game starting...");
    
    // Spawn camera
    commands.spawn((
        Camera2d,
        Transform::default(),
    ));
    
    // TODO: Spawn your game entities
}

fn update_game() {
    // TODO: Your game logic
}
```

### Template `README.md`

```markdown
# My Bevy Game

Built with [Planetarium Framework](https://github.com/teratron/planetarium)

## Quick Start

```bash
# Build and run
cargo run

# Your game is ready! 🎮
```

## Project Structure

```
my-game/
├── src/
│   └── main.rs       # Your game entry point
└── assets/           # Your game assets
    ├── models/       # 3D models
    ├── textures/     # Textures
    ├── audio/        # Sounds & music
    └── locales/      # Game-specific translations
```

## Framework Features (Included)

✅ Professional launcher with boot sequence  
✅ Splash screen system  
✅ Main menu with settings (Graphics, Audio, Controls)  
✅ Localization system (Fluent)  
✅ Theme system (colors, fonts, metrics)  
✅ Screen transitions (fading)  
✅ Loading screen with progress  
✅ Diagnostics overlay (F1)  

## Add Your Game Logic

1. Edit `src/main.rs`
2. Add your assets to `assets/`
3. Enjoy! Framework handles the rest.

## License

MIT

```

### Using the Template

```bash
# Option 1: cargo-generate (if published)
cargo generate --git https://github.com/teratron/planetarium \
                --name my-awesome-game \
                templates/game-template

# Option 2: Manual copy
git clone https://github.com/teratron/planetarium
cp -r planetarium/templates/game-template my-awesome-game
cd my-awesome-game
cargo run
```

## Best Practices

### 1. Crate Design Principles

**✅ DO:**

- Keep crates focused and single-purpose
- Use feature flags for optional functionality
- Provide comprehensive examples
- Document all public APIs
- Include README.md and ASSETS.md
- Use semantic versioning
- Test each crate independently

**❌ DON'T:**

- Create circular dependencies between crates
- Expose internal implementation details
- Hardcode paths or magic strings
- Mix framework and game logic
- Skip error handling in public APIs

### 2. Asset Management Rules

**✅ DO:**

- Embed critical assets (fallback font, UI sounds)
- Use build.rs to assemble assets
- Document all assets in ASSETS.md
- Follow consistent path conventions
- Validate asset integrity in build scripts
- Keep asset files small (<100KB for UI)
- Use compressed formats (ogg, webp)

**❌ DON'T:**

- Embed large assets (>50KB) unless critical
- Use absolute paths
- Rely solely on disk assets for framework
- Mix framework and game assets in same directory
- Forget to update build.rs when adding assets

### 3. Dependency Management

**✅ DO:**

- Use workspace dependencies
- Minimize external dependencies
- Keep dependency tree shallow
- Document why each dependency is needed
- Use optional dependencies with features
- Pin critical dependency versions

**❌ DON'T:**

- Duplicate dependencies across crates
- Add dependencies "just in case"
- Use deprecated or unmaintained crates
- Ignore security advisories

### 4. API Design

**✅ DO:**

- Provide sane defaults
- Use builder patterns for complex configuration
- Return `Result` for fallible operations
- Use consistent naming conventions
- Provide both high-level and low-level APIs
- Document all edge cases

**Example:**

```rust
// Good: Builder pattern with defaults
let launcher = LauncherPlugin::builder()
    .skip_splash(false)
    .initial_state("MainMenu")
    .build();

// Good: Result for fallible operations
pub fn load_settings(path: &Path) -> Result<Settings, SettingsError>;

// Good: Consistent naming
pub trait LoadAsset {
    fn load(&self) -> Handle<Self::Asset>;
    fn load_async(&self) -> impl Future<Output = Result<Self::Asset>>;
}
```

**❌ DON'T:**

```rust
// Bad: No defaults, must specify everything
let launcher = LauncherPlugin {
    skip_splash: false,
    initial_state: Some("MainMenu".to_string()),
    boot_config: BootConfig::default(),
    splash_duration: 2.0,
    // ... 20 more required fields
};

// Bad: Panic on error
pub fn load_settings(path: &Path) -> Settings {
    std::fs::read_to_string(path).unwrap() // DON'T PANIC!
}

// Bad: Inconsistent naming
pub trait AssetLoader {
    fn get(&self) -> Handle<Asset>;  // Why "get" not "load"?
    fn async_load(&self) -> Future;  // Why "async_load" not "load_async"?
}
```

### 5. Documentation Standards

Each crate must include:

1. **README.md** - Overview, features, quick start
2. **ASSETS.md** - Asset inventory and documentation
3. **examples/** - Working examples for common use cases
4. **API docs** - Comprehensive rustdoc comments

**Example README.md structure:**

```markdown
# Crate Name

Brief description (1-2 sentences).

## Features

- Feature 1
- Feature 2
- Feature 3

## Quick Start

```rust
use crate_name::prelude::*;

fn main() {
    App::new()
        .add_plugins(CratePlugin::default())
        .run();
}
```

## Examples

See `examples/` directory for complete examples.

## License

MIT

```

### 6. Testing Strategy

**Unit Tests:**
- Test each crate independently
- Mock external dependencies
- Test error handling paths
- Use property-based testing for complex logic

**Integration Tests:**
- Test crate interactions
- Test asset loading
- Test state transitions
- Test UI workflows

**Example test structure:**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_boot_creates_paths() {
        let paths = AppPaths::from_env();
        assert!(paths.data_dir.exists() || paths.data_dir == PathBuf::from(""));
    }
    
    #[test]
    fn test_load_settings_handles_missing_file() {
        let result = load_settings(Path::new("nonexistent.ron"));
        assert!(result.is_err());
    }
}
```

### 7. Performance Guidelines

**✅ DO:**

- Profile before optimizing
- Cache expensive computations
- Use `Changed` queries to avoid unnecessary updates
- Batch asset loading
- Use appropriate data structures

**❌ DON'T:**

- Clone large data structures unnecessarily
- Query every frame without filters
- Load assets synchronously in update systems
- Allocate in hot paths

**Example optimizations:**

```rust
// Good: Only update when settings change
fn update_ui(
    settings: Res<Settings>,
    mut query: Query<&mut Text, With<SettingsDisplay>>,
) {
    if !settings.is_changed() {
        return;
    }
    
    for mut text in &mut query {
        text.sections[0].value = format!("Volume: {}", settings.volume);
    }
}

// Bad: Update every frame
fn update_ui_bad(
    settings: Res<Settings>,
    mut query: Query<&mut Text, With<SettingsDisplay>>,
) {
    // Runs every frame even if nothing changed!
    for mut text in &mut query {
        text.sections[0].value = format!("Volume: {}", settings.volume);
    }
}
```

## Code Examples

### Example 1: Framework Crate Library

**`crates/launcher/src/lib.rs`**

```rust
//! # Launcher Framework for Bevy
//!
//! Provides application lifecycle management including:
//! - Boot sequence (paths, config, metadata)
//! - Splash screen system
//! - Asset loading with progress tracking
//! - Diagnostics overlay (optional)
//!
//! ## Quick Start
//!
//! ```no_run
//! use bevy::prelude::*;
//! use launcher::prelude::*;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugins(LauncherPlugin::default())
//!         .run();
//! }
//! ```

use bevy::prelude::*;

pub mod boot;
pub mod splash;
pub mod loading;

#[cfg(feature = "diagnostics")]
pub mod diagnostics;

mod embedded;

/// Main launcher plugin
///
/// Orchestrates the boot sequence, splash screens, and asset loading.
#[derive(Default)]
pub struct LauncherPlugin {
    /// Skip splash screens (useful for development)
    pub skip_splash: bool,
    
    /// Initial application state after boot
    pub initial_state: Option<String>,
}

impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        // Boot systems (PreStartup)
        app.add_plugins(boot::BootPlugin);
        
        // Splash systems (Startup)
        if !self.skip_splash {
            app.add_plugins(splash::SplashPlugin);
        }
        
        // Loading systems (Update)
        app.add_plugins(loading::LoadingPlugin);
        
        // Diagnostics overlay (optional feature)
        #[cfg(feature = "diagnostics")]
        app.add_plugins(diagnostics::DiagnosticsPlugin);
        
        info!("[Launcher] Plugin initialized");
    }
}

/// Builder for LauncherPlugin with convenient defaults
impl LauncherPlugin {
    pub fn builder() -> LauncherPluginBuilder {
        LauncherPluginBuilder::default()
    }
}

#[derive(Default)]
pub struct LauncherPluginBuilder {
    skip_splash: bool,
    initial_state: Option<String>,
}

impl LauncherPluginBuilder {
    pub fn skip_splash(mut self, skip: bool) -> Self {
        self.skip_splash = skip;
        self
    }
    
    pub fn initial_state(mut self, state: impl Into<String>) -> Self {
        self.initial_state = Some(state.into());
        self
    }
    
    pub fn build(self) -> LauncherPlugin {
        LauncherPlugin {
            skip_splash: self.skip_splash,
            initial_state: self.initial_state,
        }
    }
}

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::boot::{BootPlugin, BootConfig, AppPaths, AppMetadata};
    pub use crate::splash::{SplashPlugin, SplashSequence};
    pub use crate::loading::{LoadingPlugin, LoadingTracker};
    pub use crate::{LauncherPlugin, LauncherPluginBuilder};
}
```

### Example 2: Game Main Entry Point

**`src/main.rs`**

```rust
//! Planetarium - A Space Simulation
//!
//! Built with custom Bevy framework for launcher, menu, and UI.

use bevy::prelude::*;

// Framework imports (from workspace crates)
use launcher::prelude::*;
use menu::MenuPlugin;
use theme::ThemePlugin;
use localization::LocalizationPlugin;
use transitions::TransitionsPlugin;

// Game imports (local modules)
mod world;
mod physics;
mod ui;

use world::WorldPlugin;
use physics::PhysicsPlugin;
use ui::HudPlugin;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Booting,
    Splash,
    MainMenu,
    Loading,
    InGame,
}

fn main() {
    App::new()
        // Bevy configuration
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Planetarium".into(),
                resolution: (1280.0, 720.0).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }).set(AssetPlugin {
            file_path: get_assets_path(),
            ..default()
        }))
        
        // State management
        .init_state::<AppState>()
        
        // 🔧 FRAMEWORK LAYER (reusable components)
        .add_plugins((
            LauncherPlugin::builder()
                .skip_splash(false)
                .initial_state("Splash")
                .build(),
            MenuPlugin::default(),
            ThemePlugin,
            LocalizationPlugin::new("en-US", vec!["en-US", "ru-RU"]),
            TransitionsPlugin,
        ))
        
        // 🎮 GAME LAYER (Planetarium-specific)
        .add_plugins((
            WorldPlugin,
            PhysicsPlugin,
            HudPlugin,
        ))
        
        .run();
}

fn get_assets_path() -> String {
    use std::path::PathBuf;
    
    let path = if cfg!(debug_assertions) {
        // Dev mode: use assembled assets from target/
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("assets")
    } else {
        // Release: assets folder next to executable
        PathBuf::from("assets")
    };
    
    path.to_string_lossy().to_string()
}
```

### Example 3: Using Framework Theme in Game

**`src/world/mod.rs`**

```rust
//! Planetarium world management
//!
//! Handles planet creation, orbital mechanics, and scene setup.

use bevy::prelude::*;
use theme::Theme;  // Framework crate

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(crate::AppState::InGame), setup_world)
            .add_systems(Update, (
                update_planet_orbits,
                handle_camera_controls,
            ).run_if(in_state(crate::AppState::InGame)));
    }
}

#[derive(Component)]
pub struct Planet {
    pub mass: f32,
    pub radius: f32,
    pub orbit_radius: f32,
    pub orbit_speed: f32,
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    theme: Res<Theme>,  // ✅ Use framework theme
) {
    info!("[World] Creating planetary system");
    
    // Sun (center)
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(5.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.9, 0.3),
            emissive: LinearRgba::rgb(10.0, 9.0, 3.0),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Planet {
            mass: 1000.0,
            radius: 5.0,
            orbit_radius: 0.0,
            orbit_speed: 0.0,
        },
        Name::new("Sun"),
    ));
    
    // Earth (orbiting)
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: theme.colors.accent,  // ✅ Use theme color
            metallic: 0.3,
            perceptual_roughness: 0.8,
            ..default()
        })),
        Transform::from_xyz(15.0, 0.0, 0.0),
        Planet {
            mass: 10.0,
            radius: 1.0,
            orbit_radius: 15.0,
            orbit_speed: 0.5,
        },
        Name::new("Earth"),
    ));
    
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 20.0, 30.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
    
    // Lighting
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -std::f32::consts::FRAC_PI_4,
            std::f32::consts::FRAC_PI_4,
            0.0,
        )),
    ));
}

fn update_planet_orbits(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Planet)>,
) {
    for (mut transform, planet) in &mut query {
        if planet.orbit_speed > 0.0 {
            let angle = time.elapsed_secs() * planet.orbit_speed;
            
            transform.translation.x = angle.cos() * planet.orbit_radius;
            transform.translation.z = angle.sin() * planet.orbit_radius;
        }
    }
}

fn handle_camera_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let Ok(mut transform) = camera.get_single_mut() else {
        return;
    };
    
    let speed = 10.0 * time.delta_secs();
    
    if keyboard.pressed(KeyCode::KeyW) {
        transform.translation.z -= speed;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        transform.translation.z += speed;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        transform.translation.x -= speed;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        transform.translation.x += speed;
    }
}
```

### Example 4: Crate Example File

**`crates/launcher/examples/basic_launcher.rs`**

```rust
//! Basic launcher example
//!
//! Demonstrates minimal usage of the launcher framework.
//!
//! Run with: cargo run --example basic_launcher --package launcher

use bevy::prelude::*;
use launcher::prelude::*;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Booting,
    Splash,
    MainMenu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        
        // Add launcher framework
        .add_plugins(LauncherPlugin::default())
        
        // Add simple game logic
        .add_systems(OnEnter(GameState::InGame), setup_game)
        .add_systems(Update, (
            handle_input,
        ).run_if(in_state(GameState::InGame)))
        
        .run();
}

fn setup_game(mut commands: Commands) {
    println!("\n=================================");
    println!("🎮 Game Started!");
    println!("=================================\n");
    
    // Spawn camera
    commands.spawn((
        Camera2d,
        Transform::default(),
    ));
    
    // Spawn some example text
    commands.spawn((
        Text::new("Launcher Framework Example\nPress ESC to exit"),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            top: Val::Px(20.0),
            ..default()
        },
    ));
}

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        println!("\nExiting...");
        exit.send(AppExit::Success);
    }
}
```

## Summary

This guide provides a comprehensive reference for AI agents working on the Planetarium project. Key takeaways:

1. **Two-Layer Architecture**: Framework (reusable) and Game (specific)
2. **Workspace Structure**: Crates for framework, src/ for game
3. **Asset Management**: Build scripts copy assets, critical assets embedded
4. **Migration Strategy**: Phased approach over 5 weeks
5. **Best Practices**: Focused crates, comprehensive docs, thorough testing

Follow these guidelines to maintain consistency, enable reusability, and create a professional codebase.

---

**Document Version:** 1.0  
**Last Updated:** 2026-02-15  
**Status:** Active
