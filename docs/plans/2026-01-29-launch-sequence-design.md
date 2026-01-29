# Launch Sequence Design

**Date:** 2026-01-29  
**Status:** Draft  
**Spec Reference:** [launching-game.md](../specs/launching-game.md)

## Overview

Implementation of the game launch sequence for Planetarium — a state machine managing application flow from boot to gameplay.

## Design Decisions (from Brainstorming)

| Decision | Choice | Rationale |
| -------- | ------ | --------- |
| Code Organization | Modular + Plugin | Each state in separate file with Plugin trait |
| Scope | Base → Full | Start with core states, extensible for animations |
| Asset Handling | External (AssetServer) | Bevy standard, hot-reload support |
| Error Handling | Simple fallback | Skip failed assets, proceed to next state |

## Architecture

### State Flow Diagram

```plaintext
Boot → Splash → MainMenu → Loading → Gameplay ↔ Paused
                   ↓           ↑
               Settings     Credits
```

### File Structure

```plaintext
src/
├── main.rs                 # App entry, plugin registration
├── lib.rs                  # Library root (optional)
├── states/
│   ├── mod.rs              # GameState enum, re-exports
│   ├── boot.rs             # BootPlugin
│   ├── splash.rs           # SplashPlugin  
│   ├── main_menu.rs        # MainMenuPlugin
│   ├── settings.rs         # SettingsPlugin
│   ├── loading.rs          # LoadingPlugin
│   ├── gameplay.rs         # GameplayPlugin
│   └── paused.rs           # PausedPlugin
├── ui/
│   ├── mod.rs              # UI exports
│   ├── buttons.rs          # Reusable button spawning
│   └── theme.rs            # Colors, fonts, spacing
└── resources/
    ├── mod.rs              # Resource exports
    └── settings.rs         # GameSettings resource
```

### GameState Enum

```rust
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Boot,
    Splash,
    MainMenu,
    Settings,
    Loading,
    Gameplay,
    Paused,
    Credits,
}
```

### Plugin Pattern

Each state follows the pattern:

```rust
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Splash), setup_splash)
           .add_systems(Update, splash_logic.run_if(in_state(GameState::Splash)))
           .add_systems(OnExit(GameState::Splash), cleanup_splash);
    }
}
```

### Bevy Best Practices Applied

1. **Modular plugins** — logic grouped by feature
2. **StateScoped cleanup** — entities despawned on state exit
3. **AssetServer** — async loading with `Handle<T>`
4. **Component markers** — tag entities for queries (`SplashScreen`, `MainMenuRoot`)
5. **Event system** — loose coupling between systems

## Components

### Core Resources

| Resource | Purpose |
| -------- | ------- |
| `GameSettings` | Graphics, audio, controls config |
| `SplashTimer` | Auto-transition timer |
| `LoadingAssets` | Tracks asset handles for progress |

### Marker Components (per state)

| State | Marker | Cleanup |
| ----- | ------ | ------- |
| Splash | `SplashScreen` | OnExit despawn |
| MainMenu | `MainMenuRoot` | OnExit despawn |
| Settings | `SettingsScreen` | OnExit despawn |
| Loading | `LoadingScreen` | OnExit despawn |
| Gameplay | `GameplayCamera` | OnExit despawn |

## Error Handling Strategy

**Fallback approach:**

```rust
fn setup_splash(commands: Commands, asset_server: Res<AssetServer>) {
    // If logo fails to load, splash will still work
    // Timer will transition to MainMenu regardless
    let logo = asset_server.load("textures/logo.png");
    // ... spawn with handle
}
```

- Missing assets → use default/placeholder or skip
- State transition errors → log warning, proceed anyway
- No blocking error dialogs in MVP

## Assets Required

```plaintext
assets/
├── textures/
│   └── logo.png            # Splash screen logo
└── fonts/
    └── FiraSans-Regular.ttf  # Optional: custom font
```

## Future Extensions (v2)

- [ ] Fade-in/fade-out transitions between states
- [ ] Splash skip after 1 second
- [ ] Loading progress bar with percentage
- [ ] Button hover animations
- [ ] Sound effects on UI interaction
- [ ] Save/Load game functionality
