# Task List: Launching Module Implementation

## Phase 1: Foundation & Booting (Priority: Critical)

- [x] **[L-101]** Setup Project Structure
  - Create `src/launcher/`, `src/game/`, `src/core/`, `src/ui/`.
  - Setup `main.rs` and `lib.rs`.
- [x] **[L-102]** AppState & Core Plugins
  - Define `AppState` enum.
  - Register empty `LauncherPlugin` and its sub-plugins.
- [x] **[L-103]** CLI Arguments Support
  - Implement `--skip-splash` and `--state` flags.
- [x] **[L-104]** Configuration System
  - Implement `settings.toml` loading with `serde`.
  - Implement cross-platform path resolution for `%APPDATA%`.
- [x] **[L-105]** Config Migration (Version Guard)
  - Add version check and auto-merge logic for settings.
- [x] **[L-106]** App Metadata Centralization
  - Move hardcoded values (App Title, Version) from `main.rs` to a config file.

## Phase 2: Localization & Assets (Priority: High)

- [x] **[L-201]** Fluent Localization Engine
  - Setup `bevy_fluent` or manual Fluent integration.
  - Create `en-US` and `ru-RU` directory structure.
- [x] **[L-202]** Localized Asset Loader
  - Implement the "Fallback" logic for missing strings/audio.
- [x] **[L-203]** Asset Manifest System
  - Implement `assets.toml` parsing.
  - Setup bundled asset loading handles.

## Phase 3: Brand & Feedback (Splash) (Priority: Medium)

- [x] **[L-301]** Splash Screen Sprite/Video Player
  - Implement sequence logic with timers.
- [x] **[L-302]** Skip Logic & Input Interleaving
  - Implement the "1s minimum" skip rule.
- [x] **[L-303]** UI Fading System
  - Component-based alpha blending for smooth transitions.

## Phase 4: Main Menu & UI Framework (Priority: High)

- [x] **[L-401]** Theme & Design Tokens
  - Define colors, fonts, and sizes in `theme.rs`.
- [x] **[L-402]** Generic Widget Library
  - Create `PrimaryButton`, `Slider`, and `Dropdown` widgets.
- [x] **[L-403]** Main Menu Layout
  - Implement the landing screen (Play, Settings, Exit).
- [x] **[L-404]** Settings Screen GUI
  - Implement tabs for Graphics, Audio, Controls.
- [x] **[L-405]** Reactive Audio/Graphics Settings
  - Implement events that update the engine state immediately.

## Phase 5: Loading & Orchestration (Priority: High)

- [x] **[L-501]** Loading Progress UI
  - Implement the progress bar and lore hints.
- [x] **[L-502]** Transition to In-Game
  - Logic for handing over control from Launcher to Game plugins.

## Phase 6: Robustness & Diagnostics (Priority: Low)

- [ ] **[L-601]** Debug Overlay
  - Toggleable FPS/Memory/Logs overlay.
- [ ] **[L-602]** Embedded Fallbacks
  - Use `include_bytes!` for emergency fonts and textures.
- [ ] **[L-603]** Logging & Analytics
  - Setup file-based logging in the user data folder.
