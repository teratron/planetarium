# Launch Sequence Implementation Tasks

Based on [Spec](../specs/launching-game.md) and [Design Plan](../plans/2026-01-29-launch-sequence-design.md).

## 1. Project Structure & Core Setup

- [ ] Create directory structure:
  - `src/states/`
  - `src/ui/`
  - `src/resources/`
- [ ] Create `src/states/mod.rs`:
  - [ ] Define `GameState` enum (Boot, Splash, MainMenu, Settings, Loading, Gameplay, Paused, Credits).
  - [ ] Re-export separate state modules.
  - [ ] Implement `StatesPlugin` to register all state plugins.
- [ ] Create `src/ui/mod.rs` and `src/resources/mod.rs` for module exports.
- [ ] Update `src/main.rs`:
  - [ ] Add `mod` declarations for `states`, `ui`, `resources`.
  - [ ] Initialize `App` with `DefaultPlugins` and `StatesPlugin`.
  - [ ] Initialize `GameState`.

## 2. Boot State

- [ ] Create `src/states/boot.rs`:
  - [ ] Implement `BootPlugin`.
  - [ ] System `on_enter(Boot)`: Setup 2D Camera, Initialize `GameSettings` and `SaveData` resources.
  - [ ] System `update(Boot)`: Check initialization complete -> Transition to `Splash`.

## 3. Splash Screen State

- [ ] Create `assets/textures/logo.png` (placeholder).
- [ ] Create `src/states/splash.rs`:
  - [ ] Implement `SplashPlugin`.
  - [ ] Component `SplashScreen` (Marker).
  - [ ] System `on_enter(Splash)`: Spawn UI with Logo and "Loading..." text. Start `SplashTimer` (3s).
  - [ ] System `update(Splash)`: Tick timer, handle skip input (Space/Enter/Click) after 1s. Transition to `MainMenu`.
  - [ ] System `on_exit(Splash)`: Despawn `SplashScreen` entities.

## 4. Main Menu State

- [ ] Create `src/ui/buttons.rs` and `src/ui/theme.rs`:
  - [ ] Implement shared button spawning helpers and style constants.
- [ ] Create `src/states/main_menu.rs`:
  - [ ] Implement `MainMenuPlugin`.
  - [ ] Component `MainMenuRoot` (Marker).
  - [ ] System `on_enter(MainMenu)`: Spawn Title Text and Buttons (New Game, Settings, Credits, Exit).
  - [ ] System `update(MainMenu)`: Handle button interactions (Color change on hover, Action on click).
  - [ ] System `on_exit(MainMenu)`: Despawn `MainMenuRoot` entities.

## 5. Settings State

- [ ] Create `src/states/settings.rs`:
  - [ ] Implement `SettingsPlugin`.
  - [ ] Component `SettingsScreen` (Marker).
  - [ ] System `on_enter(Settings)`: Spawn Placeholder Text and "Back" instruction.
  - [ ] System `update(Settings)`: Handle `Esc` key -> Transition to `MainMenu`.
  - [ ] System `on_exit(Settings)`: Despawn `SettingsScreen` entities.

## 6. Loading State

- [ ] Create `src/states/loading.rs`:
  - [ ] Implement `LoadingPlugin`.
  - [ ] Component `LoadingScreen` (Marker).
  - [ ] Resource `LoadingAssets` to track handles.
  - [ ] System `on_enter(Loading)`: Start loading assets (example textures), spawn Progress UI.
  - [ ] System `update(Loading)`: Update progress bar, check `AssetServer::get_load_state`. Transition to `Gameplay` when done.
  - [ ] System `on_exit(Loading)`: Despawn `LoadingScreen` entities.

## 7. Gameplay & Paused States

- [ ] Create `src/states/gameplay.rs`:
  - [ ] Implement `GameplayPlugin`.
  - [ ] Component `GameplayCamera` (Marker).
  - [ ] System `on_enter(Gameplay)`: Spawn 3D Camera.
  - [ ] System `update(Gameplay)`: Handle `Esc` -> Transition to `Paused`.
  - [ ] System `on_exit(Gameplay)`: Despawn `GameplayCamera` (optional, depending on persistence).
- [ ] Create `src/states/paused.rs` (Optional/MVP):
  - [ ] Simple overlay or state handling to return to gameplay or menu.

## 8. Final Verification

- [ ] Run `cargo run`.
- [ ] Verify flow: Boot -> Splash -> MainMenu -> Settings -> Back -> New Game -> Loading -> Gameplay -> Pause/Exit.
