# Launch Sequence Implementation Tasks

Based on [Spec](../specs/launching-game.md) and [Design Plan](../plans/2026-01-29-launch-sequence-design.md).

## 1. Project Structure & Core Setup

- [x] Create directory structure:
  - `src/states/`
  - `src/ui/`
  - `src/resources/`
- [x] Create `src/states/mod.rs`:
  - [x] Define `GameState` enum (Boot, Splash, MainMenu, Settings, Loading, Gameplay, Paused, Credits).
  - [x] Re-export separate state modules.
  - [x] Implement `StatesPlugin` to register all state plugins.
- [x] Create `src/ui/mod.rs` and `src/resources/mod.rs` for module exports.
- [x] Update `src/main.rs`:
  - [x] Add `mod` declarations for `states`, `ui`, `resources`.
  - [x] Initialize `App` with `DefaultPlugins` and `StatesPlugin`.
  - [x] Initialize `GameState`.

## 2. Boot State

- [x] Create `src/states/boot.rs`:
  - [x] Implement `BootPlugin`.
  - [x] System `on_enter(Boot)`: Setup 2D Camera, Initialize `GameSettings` and `SaveData` resources.
  - [x] System `update(Boot)`: Check initialization complete -> Transition to `Splash`.

## 3. Splash Screen State

- [x] Create `assets/textures/` directory (logo uses text placeholder).
- [x] Create `src/states/splash.rs`:
  - [x] Implement `SplashPlugin`.
  - [x] Component `SplashScreen` (Marker).
  - [x] System `on_enter(Splash)`: Spawn UI with Logo and "Loading..." text. Start `SplashTimer` (3s).
  - [x] System `update(Splash)`: Tick timer, handle skip input (Space/Enter/Click) after 1s. Transition to `MainMenu`.
  - [x] System `on_exit(Splash)`: Despawn `SplashScreen` entities.

## 4. Main Menu State

- [x] Create `src/ui/buttons.rs` and `src/ui/theme.rs`:
  - [x] Implement shared button spawning helpers and style constants.
- [x] Create `src/states/main_menu.rs`:
  - [x] Implement `MainMenuPlugin`.
  - [x] Component `MainMenuRoot` (Marker).
  - [x] System `on_enter(MainMenu)`: Spawn Title Text and Buttons (New Game, Settings, Credits, Exit).
  - [x] System `update(MainMenu)`: Handle button interactions (Color change on hover, Action on click).
  - [x] System `on_exit(MainMenu)`: Despawn `MainMenuRoot` entities.

## 5. Settings State

- [x] Create `src/states/settings.rs`:
  - [x] Implement `SettingsPlugin`.
  - [x] Component `SettingsScreen` (Marker).
  - [x] System `on_enter(Settings)`: Spawn Placeholder Text and "Back" instruction.
  - [x] System `update(Settings)`: Handle `Esc` key -> Transition to `MainMenu`.
  - [x] System `on_exit(Settings)`: Despawn `SettingsScreen` entities.

## 6. Loading State

- [x] Create `src/states/loading.rs`:
  - [x] Implement `LoadingPlugin`.
  - [x] Component `LoadingScreen` (Marker).
  - [x] Resource `LoadingAssets` to track handles.
  - [x] System `on_enter(Loading)`: Start loading assets (example textures), spawn Progress UI.
  - [x] System `update(Loading)`: Update progress bar, check `AssetServer::get_load_state`. Transition to `Gameplay` when done.
  - [x] System `on_exit(Loading)`: Despawn `LoadingScreen` entities.

## 7. Gameplay & Paused States

- [x] Create `src/states/gameplay.rs`:
  - [x] Implement `GameplayPlugin`.
  - [x] Component `GameplayCamera` (Marker).
  - [x] System `on_enter(Gameplay)`: Spawn 3D Camera.
  - [x] System `update(Gameplay)`: Handle `Esc` -> Transition to `Paused`.
  - [x] System `on_exit(Gameplay)`: Despawn `GameplayCamera` (optional, depending on persistence).
- [x] Create `src/states/paused.rs` (Optional/MVP):
  - [x] Simple overlay or state handling to return to gameplay or menu.

## 8. Final Verification

- [x] Run `cargo run`.
- [x] Verify flow: Boot -> Splash -> MainMenu -> Settings -> Back -> New Game -> Loading -> Gameplay -> Pause/Exit.

## Implementation Complete! ✅

All tasks have been successfully completed. The launch sequence is fully functional with:
- Complete state machine (Boot → Splash → MainMenu → Settings/Loading → Gameplay ↔ Paused)
- Proper resource management (GameSettings, SaveData)
- UI system with theme and button components
- Asset loading with progress tracking
- Input handling and state transitions
- Clean state cleanup on exit
