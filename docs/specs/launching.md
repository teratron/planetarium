# Launching Module Specification

## Overview

The Launching Module is designed as a universal, modular template for Bevy games. It handles the initial stages of a game application, from the very first frame to the moment the player enters the game world.

The primary goal is to provide a standardized structure that can be easily reused across different projects (both 2D and 3D) while maintaining a strict separation between the platform/launch logic and the specific game content.

## Launch Sequence Overview

```mermaid
graph LR
    subgraph S1 [Stage 1: Initialization]
        Booting[Booting]
    end
    subgraph S2 [Stage 2: Brand/Feedback]
        Splash[Splash]
    end
    subgraph S3 [Stage 3: Interaction]
        MainMenu[Main Menu]
    end
    subgraph S4 [Stage 4: Orchestration]
        Loading[Loading]
    end
    subgraph S5 [Stage 5: Active]
        InGame[In-Game]
    end

    Booting --> Splash
    Splash --> MainMenu
    MainMenu --> Loading
    Loading --> InGame
    
    %% Background details
    Booting -.-> B_Init(Updates / Auth)
    Splash -.-> S_Pre(Shaders / Hints)
    MainMenu -.-> M_Opt(Settings / Saves)
    Loading -.-> L_Ast(Assets / World)
    
    InGame <--> Paused((Paused))
    
    %% Error Flow
    Booting & Splash & MainMenu & Loading -.-> Error([ERROR STATE])
```

## Design Principles

- **Modularity**: Implemented as a set of Bevy Plugins.
- **State-Driven**: Uses Bevy's `States` to manage transitions between phases.
- **Extensibility**: Core components should be easy to override or extend without modifying the module's core logic.
- **Generic Support**: Core logic is agnostic to 2D or 3D rendering. UI components (Menu/Settings) are designed to be adaptable or conditional based on the project's requirements.
- **Core-First Approach**: Maximize use of Bevy's standard library. Third-party plugins are only introduced when required functionality is absent from the engine's core.

## Launching Sequence

The module manages the following states:

1. **Boot**:
    - **Environment Setup**: Determines platform-specific paths (e.g., `%APPDATA%` on Windows, `~/.local/share` on Linux) for logs and saves.
    - Loads and validates configuration files (`settings.toml`).
    - Initializes the window and rendering engine (Graphic API, Input, Audio).
    - **Diagnostics**: Initializes Bevy's `LogPlugin` and non-blocking analytics/telemetry.
    - **Update & Security**: Verifies version with the server and performs background file integrity checks (hashes).
    - **Authentication**: Handles user login or session restoration if required.
    - **Shader Pre-compilation**: Loads and compiles necessary shaders.
    - **Localization**: Loads selected language files (ISO 639-1) before entering the menu.
2. **Splash**:
    - Displays a sequence of splash screens (Engine license requirements, Studio logo, Partners).
    - **Transitions**: Support for smooth fade-in/fade-out between screens.
    - **Background Tasks**: Continues non-blocking initialization tasks (e.g., verifying large asset manifests).
    - **Progress & Feedback**: Ability to see loading progress for the next state and random gameplay tips/hints.
3. **MainMenu**:
    - Provides a standard UI for:
        - **Play**: Mode selection (Campaign, Multiplayer, Sandbox).
        - **New Game**: Profile creation and initial slot selection.
        - **Load Game**: List of save slots with visual previews and metadata.
        - **Settings**: Comprehensive GUI for all options (Graphics, Sound, Controls, Language).
        - **Extras**: Achievements, Credits, or Store (if applicable).
        - **Exit**: Clean application shutdown.
    - **Save Management**: Verifies save slot integrity and metadata before enabling "Load Game".
    - **Adaptive UI**: The menu and settings dynamically adjust their content based on whether the game is 2D or 3D.
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
- `Error`: Critical failure state (shows error message and exit/retry options).
- `InGame`: Active gameplay (Handled by external game logic).

### Plugins

- `BootPlugin`: Handles initial setup, update checks, and authentication logic.
- `SplashPlugin`: Manages the queue of splash screens, including progress indicators and hint systems.
- `MenuPlugin`: Core menu functionality and the settings sub-system.
- `LoadingPlugin`: Resource orchestration via manifest tags, progress tracking, and transition to the game.

### Bevy Implementation Details

#### Core Resources

| Resource | Purpose |
| -------- | ------- |
| `GameSettings` | Graphics, audio, and controls configuration initialized from TOML. |
| `SplashTimer` | Timer resource for auto-transitioning between brand screens. |
| `LoadingAssets` | Tracks `Handle<T>` collections to calculate loading progress. |

#### Marker Components (State Management)

| Marker | Purpose | Cleanup Strategy |
| ------ | ------- | ---------------- |
| `SplashScreen` | Entities visible during Stage 2. | `OnExit(AppState::Splash)` despawn. |
| `MainMenuRoot` | The root UI node for the Main Menu. | `OnExit(AppState::MainMenu)` despawn. |
| `SettingsScreen` | Root node for the settings interface. | `OnExit(AppState::MainMenu)` despawn. |
| `LoadingScreen` | UI elements for the progress bar and tips. | `OnExit(AppState::Loading)` despawn. |
| `GameplayCamera` | Primary game camera (3D or 2D). | `OnExit(AppState::InGame)` cleanup. |

#### Best Practices

- **Modular Plugins**: Every stage is an independent `Plugin`.
- **StateScoped Cleanup**: Use `OnExit` systems or `StateScoped` components to ensure no "leaking" entities between states.
- **Marker Components**: Use empty structs (e.g., `struct SplashScreen;`) to filter entities in systems.
- **Event-Driven UI**: UI updates respond to events (e.g., `VolumeChangedEvent`) rather than polling.

## Error Handling Strategy

- **Missing Assets**: Use hardcoded placeholders (e.g., a magenta square for textures) OR skip non-essential assets with a log warning.
- **State Transition Failures**: Log an `ERROR`, but attempt to fallback to the `MainMenu` instead of crashing.
- **Minimal MVP**: Avoid complex modal dialogs for non-critical errors; keep the flow moving.

## State-Driven Scene Management

The project follows the "Professional Game Loop" standard, separating logic from data using **Bevy States** and **Scenes**:

1. **Finite State Machine (FSM)**: `AppState` acts as the application's "brain," controlling the lifecycle, memory cleanup (`OnExit`), and system scheduling.
2. **Scene Controller**: Each state manages its own data container (Scene).
    - Entering a state triggers the asynchronous loading of a specific scene (UI layouts, lighting, background assets).
    - Exiting a state ensures efficient resource unloading.
3. **Decoupling**: Logic (Rust systems) remains independent of visual presentation (Scene data).

## Asset Orchestration (AAA Approach)

- **Manifest File**: All game assets are defined in external manifests (e.g., `assets/assets.toml`).
- **Asset Bundles & Tagging**: Resources are grouped into logical bundles identified by tags.
- **Asynchronous Orchestration**: Processes tags requested by game logic and handles dependencies.
- **Optimized Loading**: Integration with asynchronous IO and support for pre-compiling GPU-ready data (shaders, textures).

## Proposed File Structure

For a Bevy project, it is essential to distinguish between **static content** (root assets folder) and **source code** (src folder):

### Root Project Structure

```plaintext
. (Project Root)
├── assets/                 # STATIC CONTENT (Physical files)
│   ├── images/             # Textures, sprites, icons
│   ├── fonts/              # Typography (.ttf, .otf)
│   ├── audio/              # Sound effects and music
│   ├── shaders/            # Custom GLSL/WGSL code
│   └── configs/            # assets.toml, default.toml
├── src/                    # SOURCE CODE (Rust files)
├── tests/                  # Integration tests
├── docs/                   # Project documentation and design specs
├── Cargo.toml
└── ...
```

### Source Code Detail (src/)

```plaintext
src/
├── main.rs                 # Minimal entry point (app builder)
├── lib.rs                  # Library root, primary plugin registration
├── launcher/               # THE MODULE (Reusable Template)
│   ├── mod.rs              # LauncherPlugin (aggregates all sub-plugins)
│   ├── boot.rs             # Stage 1: Logs, Config, Auth, Updates
│   ├── splash.rs           # Stage 2: Splash sequence, Hints
│   ├── menu/               # Stage 3: UI & Interaction
│   │   ├── mod.rs
│   │   ├── main_menu.rs    # Main Menu logic
│   │   └── settings.rs     # Settings GUI & Logic
│   ├── loading.rs          # Stage 4: Asset orchestration
│   └── systems/            # Shared internal launcher systems
├── game/                   # Project-specific logic (Game World)
│   ├── mod.rs
│   └── gameplay.rs         # Gameplay core states
├── ui/                     # Generic UI Framework
│   ├── mod.rs
│   ├── widgets.rs          # Styled buttons, sliders, panels
│   └── theme.rs            # Design tokens (Colors, Fonts, Sizes)
├── core/                   # Project-wide definitions
│   ├── mod.rs
│   ├── states.rs           # AppState & SubState definitions
│   └── resources.rs        # Global settings & common data
└── assets/                 # Rust code for Asset Management
    ├── mod.rs
    └── manifest.rs         # Logic for interpreting assets.toml
```

## UI Framework

To maximize engine utilization, the module strictly uses **Bevy's built-in UI system** (`bevy_ui`):

- **Flexbox Positioning**: Layouts are managed using Taffy.
- **Interactivity**: Standard Bevy `Interaction` components.
- **Styling**: Data-driven styles for easy theming.

## Live Configuration (AAA Approach)

Professional engines support mid-game updates without restart via **Hot-Reloading**:

- **File Watchers**: Detects changes in `default.toml` and manifests.
- **Event-Driven Updates**: `ConfigChangedEvent` triggers reactive updates in Audio, Graphics, and UI systems immediately.

## Technical Implementation Guidelines

| Aspect | Recommendation |
| ------ | -------------- |
| **Asset Loading** | Use asynchronous loading with caching; never block the main rendering thread. |
| **Interruption** | Allow exit via [Alt+F4] or system buttons at any stage except initial engine setup. |
| **Offline Mode** | Ensure the core game is playable without internet. |
| **Localization** | Language files must be loaded and ready before the MainMenu state. |
| **Save Integrity** | Check save slot data early (on entering the Load Menu) to prevent corrupted state crashes. |
| **Performance** | Instrument each stage with timers for internal analytics/optimization. |

## Diagnostics & Logging

The module leverages Bevy's built-in logging system (powered by the `tracing` crate) to ensure transparency and ease of debugging:

- **Log Levels**:
  - `ERROR`: Critical failures (e.g., "Failed to compile essential shaders", "Config file corrupted").
  - `WARN`: Non-blocking issues (e.g., "Update server unreachable", "Optional asset missing").
  - `INFO`: Significant milestones (e.g., "Entering MainMenu", "Authentication successful").
  - `DEBUG`: Detailed internal steps (e.g., "Loaded asset manifest in 14ms").
- **Output Targets**:
  - **Console**: Real-time feedback during development.
  - **File Log**: Automatically saved to the user's local data folder for troubleshooting production issues.
- **Contextual Data**: All logs include timestamps and the name of the originating plugin (e.g., `[BootPlugin]`).

## What to Avoid

- **Long "Black Screens"**: Never leave the player without visual feedback or a progress indicator.
- **Blocking Operations**: Do not perform heavy IO or computations on the main rendering thread.
- **Forced Updates**: Avoid mandatory updates without a "Skip" or "Cancel" option, unless they are critical for security or compatibility.
- **Missing Navigation**: Ensure every sub-menu or settings screen has a clear "Back" or "Cancel" button.

## UI/UX Interaction Standards

To achieve a professional "AAA feel," the following polish features must be implemented:

- **State Transitions**: Implement smooth fade-in/fade-out (alpha-blending) when switching between `AppState` scenes.
- **Splash Interaction**: Allow skipping any splash screen after a minimum of 1 second by pressing any key or clicking.
- **Informative Loading**: The progress bar must show a numerical percentage and the name of the current asset group being loaded.
- **Reactive UI**:
  - Buttons must have distinct hover and press animations (e.g., slight scaling or color shift).
  - Play subtle sound effects (SFX) on button hover and click.
- **Save/Load State**: The interaction logic for selecting slots and displaying metadata (Playtime, Date) must be handled within the `MenuPlugin`.

## User Stories

- **As a Developer**, I want to drop this module into a new project and have a working main menu and loading screen within minutes.
- **As a Player**, I want a smooth transition from clicking the executable to seeing the main menu.
- **As a Developer**, I want to easily add my own custom splash screen.
- **As a Game Designer**, I want to add new assets by simply editing a text manifest.

## Future Considerations

- Support for localization.
- Controller/Gamepad support for menus out of the box.
- Persistent user settings.

## Backlog / Future Discussions

- [ ] Detailed 2D vs 3D specific settings (Pixel Perfect, LODs, etc.).
- [ ] Advanced UI theming and skinning (using only Bevy UI).
- [ ] Error Handling strategy (Local vs Global errors).
- [ ] Controller/Gamepad remapping interface.
