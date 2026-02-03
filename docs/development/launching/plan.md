# Implementation Plan: Launching Module

## 1. Core Architecture

The module is built as an aggregate Bevy `Plugin` that encapsulates the entire application lifecycle from boot to game entry.

### 1.1 State Machine

We use a centralized `AppState` enum registered in `core/states.rs`:

```rust
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Booting,
    Splash,
    MainMenu,
    Loading,
    InGame,
    Error,
}
```

### 1.2 Plugin Hierarchy

- `LauncherPlugin` (Main Entry)
  - `BootPlugin` (Init, Config, FS)
  - `SplashPlugin` (Video/Image sequence)
  - `MenuPlugin` (UI, Settings, Saves)
  - `LoadingPlugin` (Asset Orchestration)
  - `DiagnosticPlugin` (Debug Overlay, Logs)

---

## 2. Phase 1: Foundation (Booting)

**Goal**: Setup the environment and load configurations.

### 2.1 Configuration Management

- **App Metadata**: Dedicated `metadata.toml` (or a section in `settings.toml`) for non-player-facing properties like `app_title` and `app_id`.
- Use `serde` and `toml` for configuration management.
- **System**: `load_config_system` (Startup).
- **Migration**: Check `version` field. If mismatched, merge with `default.toml` using `serde_json::Value` or similar manual merge logic.
- **FS**: Determine `DataDir` using `dirs` crate (Cross-platform).

### 2.2 Localization Bridge

- Initialize `FluentBundle` for the language specified in `settings.toml`.
- Load `.ftl` files from `assets/locales/{lang}/text/`.
- Provide a `LocalizedString` resource for easy text lookup.

---

## 3. Phase 2: Brand Experience (Splash)

**Goal**: Professional visual feedback and branding.

### 3.1 Splash Sequence

- Resource: `struct SplashSequence(Vec<Handle<Image>>, usize)`.
- System: `animate_splash_system`.
- **Polish**:
  - `FadeComponent` (0.0 to 1.0 alpha).
  - `SplashTimer` (supports minimum 1s duration before skip).

---

## 4. Phase 3: Interaction (Main Menu)

**Goal**: Interactive hub and settings.

### 4.1 UI Framework

- Implementation of `widgets.rs`: Generic `PrimaryButton`, `Slider`, `Toggle`.
- **Theme**: `Theme` resource containing `Color` and `Handle<Font>`.
- **Transitions**: Every menu change triggers a `TransitionEvent` to handle cross-fades.

### 4.2 Settings Sub-system

- Reactive settings: Changing a slider sends a `SettingChangedEvent`.
- Listeners update `Audio` volume or `Window` resolution immediately.

---

## 5. Phase 4: Data Orchestration (Loading)

**Goal**: Fast and informative asset loading.

### 5.1 Asset Manifest

- `assets.toml` defines groups: `[bundle.world]`, `[bundle.ui_heavy]`.
- Plugin: `LoadingPlugin` watches a `LoadingTracker` resource.
- **Progress Calculation**: `(LoadedCount / TotalCount) * 100.0`.

---

## 6. Phase 5: Robustness & Developer Tools

### 6.1 CLI & Overrides

- Use `argh` or `clap` to parse arguments.
- If `--skip-splash` is present, `BootPlugin` transitions directly to `AppState::MainMenu`.

### 6.2 Debug Overlay

- Independent `Window` or a high-order `ZIndex` UI node.
- Real-time stats from `DiagnosticsStore`.

---

## 7. Implementation Roadmap

1. **Stage 1: Core Lifecycle** (States, Plugins, CLI parsing).
2. **Stage 2: Storage & Config** (FS paths, Settings loading, Version Guard).
3. **Stage 3: Localization Engine** (Fluent setup, .ftl loader).
4. **Stage 4: Splash & Transitions** (Alpha blending, timer logic).
5. **Stage 5: UI Widgets & Theme** (Buttons, Hover SFX, Icons).
6. **Stage 6: Main Menu & Settings** (Full GUI, Event Bus for settings).
7. **Stage 7: Loading & Manifest** (Asset bundles, progress bar).
8. **Stage 8: Final Polish** (Debug overlay, error fallbacks).
