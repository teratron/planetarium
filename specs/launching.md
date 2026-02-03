# Launching Module Specification

## Overview

The Launching Module is designed as a universal, modular template for Bevy games. It handles the initial stages of a game application, from the very first frame to the moment the player enters the game world.

The primary goal is to provide a standardized structure that can be easily reused across different projects (both 2D and 3D) while maintaining a strict separation between the platform/launch logic and the specific game content.

## Launch Sequence Overview

```plaintext
┌──────────────────────────────────────────────────────────────────────────────┐
│                            LAUNCH SEQUENCE FLOW                              │
├──────────────────────────────────────────────────────────────────────────────┤
│  Booting ───────► Splash ───────► MainMenu ───────► Loading ───────► InGame  │
│     │                │               │                 │                │    │
│     └─► Updates      └─► Shaders     ├─► Settings      └─► Assets       │    │
│     └─► Auth         └─► Hints       └─► Credits       └─► World        │    │
│                                                                         │    │
│                                InGame (Gameplay) ◄───► Paused           │    │
└──────────────────────────────────────────────────────────────────────────────┘
```

## Design Principles

- **Modularity**: Implemented as a set of Bevy Plugins.
- **State-Driven**: Uses Bevy's `States` to manage transitions between phases.
- **Extensibility**: Core components should be easy to override or extend without modifying the module's core logic.
- **Generic Support**: Core logic is agnostic to 2D or 3D rendering. UI components (Menu/Settings) are designed to be adaptable or conditional based on the project's requirements (e.g., automatically hiding 3D-specific graphics settings for 2D games).

## Launching Sequence

The module manages the following states:

1. **Boot**:
    - Loads and validates configuration files (TOML/JSON).
    - Initializes the window and rendering engine.
    - Sets up global resources (logger, etc.).
    - **Update Check**: Verifies if a newer version of the application is available.
    - **Authentication**: Handles user login/session restoration if required.
    - **Shader Pre-compilation**: Loads and compiles necessary shaders to prevent stutters during gameplay.
2. **Splash**:
    - Displays a sequence of splash screens (e.g., Engine, Studio, Partners).
    - **Transitions**: Support for smooth fade-in/fade-out between screens.
    - **Progress & Feedback**: Ability to see loading progress for the next state and random gameplay tips/hints.
3. **MainMenu**:
    - Provides a standard UI for "Start Game", "Settings", "Credits", and "Exit".
    - **Comprehensive Settings**: Includes a ready-to-use GUI for all options defined in `default.toml` (Graphics quality, AA, VSync, Audio volumes, Input rebinding).
    - **Adaptive UI**: The menu and settings dynamically adjust their content based on whether the game is 2D or 3D, hiding irrelevant options.
    - **Extensibility**: Design allows easy addition/removal of menu items and re-styling of the UI.
4. **Loading**:
    - Handles asynchronous loading of assets required for the main game world/level.
    - Displays a detailed loading bar and current task status.
    - Transitions to the `InGame` state once all critical assets are ready.

## Technical Architecture

### State Machine

`AppState` (Enumeration):

- `Booting`: System initialization, updates, and auth.
- `Splash`: Sequence of brand screens.
- `MainMenu`: Interaction and configuration.
- `Loading`: Preparing game assets.
- `InGame`: Active gameplay (Handled by external game logic).

### Plugins

- `BootPlugin`: Handles initial setup, update checks, and authentication logic.
- `SplashPlugin`: Manages the queue of splash screens, including progress indicators and hint systems.
- `MenuPlugin`: Core menu functionality and the settings sub-system.
- `LoadingPlugin`: Resource orchestration via manifest tags, progress tracking, and transition to the game.

## State-Driven Scene Management

The project follows the "Professional Game Loop" standard, separating logic from data using **Bevy States** and **Scenes**:

1. **Finite State Machine (FSM)**: `AppState` acts as the application's "brain," controlling the lifecycle, memory cleanup (`OnExit`), and system scheduling.
2. **Scene Controller**: Each state manages its own data container (Scene).
    - Entering a state triggers the asynchronous loading of a specific scene (UI layouts, lighting, background assets).
    - Exiting a state ensures efficient resource unloading.
3. **Decoupling**: Logic (Rust systems) remains independent of visual presentation (Scene data), allowing for easy iteration by both developers and designers.

## Asset Orchestration (AAA Approach)

The module employs a **Manifest-Driven Asset Management** system to ensure high performance and scalability:

- **Manifest File**: All game assets are defined in external manifests (e.g., `assets/assets.toml`). This decoupling allows changing assets without recompiling the code.
- **Asset Bundles & Tagging**: Resources are grouped into logical bundles identified by tags (e.g., `"Environment_Mars"`, `"Characters"`, `"Global_SFX"`).
- **Asynchronous Orchestration**:
  - The `LoadingPlugin` interprets tags requested by the game logic (e.g., `loading_api.load_tag("Intro")`).
  - It handles resource dependencies automatically.
  - Progress is reported as a normalized float (0.0 to 1.0) and a status string for the UI.
- **Optimized Loading**: Integration with asynchronous IO and support for pre-compiling GPU-ready data (shaders, textures).

### Configuration

Integration with `default.toml` to control:

- Splash screen durations.
- Menu style presets.
- Resolution and windowing options.

## User Stories

- **As a Developer**, I want to drop this module into a new project and have a working main menu and loading screen within minutes.
- **As a Player**, I want a smooth transition from clicking the executable to seeing the main menu.
- **As a Developer**, I want to easily add my own custom splash screen before the main menu.
- **As a Game Designer**, I want to add new assets to a level by simply editing a text manifest, without touching the core loading logic.

## Future Considerations

- Support for localization.
- Controller/Gamepad support for menus out of the box.
- Persistent user settings.
