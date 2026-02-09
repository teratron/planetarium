# Task List: Main Menu & Settings Implementation

## Phase 1: Infrastructure & Stability (Priority: Critical)

- [x] **[MM-101]** Fix Path Resolution
  - Implement a helper to find the `assets/` directory relative to the executable.
  - Update `AssetManifest` and `Localization` loaders.
- [x] **[MM-102]** Setup Menu Localization
  - Create `.ftl` files for EN and RU.
  - Integrate `bevy_fluent` or existing localization resource into UI spawning.
- [x] **[MM-103]** Embedded Asset Fallbacks ✅
  - Use `include_bytes!` for emergency fonts to prevent "Error State" on startup if assets are missing.
  - Implemented safe creation of embedded fallback font with error logging instead of panicking.

## Phase 2: Refined Main Menu (Priority: High)

- [x] **[MM-201]** Main Menu UI Overhaul
  - Update layout in `screen.rs` for AAA aesthetics (typography, margins).
  - Add smooth hover animations (scaling/color shifts).
- [x] **[MM-202]** Audio Feedback
  - Implement systems to play `hover.ogg` and `click.ogg`.
  - Add audio files to `assets/audio/ui/`.

## Phase 3: Advanced Settings (Priority: High)

- [x] **[MM-301]** Tabbed Interface Implementation
  - Create the tab switching logic.
  - Implement **Graphics** tab (Resolutions, VSync, Fullscreen).
  - Implement **Audio** tab (All volume sliders from `settings.toml`).
  - Implement **General** tab (Language/Theme toggle).
- [x] **[MM-302]** Configuration Bridge
  - Link UI widgets to `UserSettings` resource.
  - Implement `SettingChangedEvent` and immediate engine updates.
  - Implement throttled saving to `settings.toml`.

## Phase 4: Polish & UX (Priority: Medium)

- [ ] **[MM-401]** Transition Fading
  - Add professional fades when opening/closing the settings overlay.
- [ ] **[MM-402]** Keybinding Display
  - Create a "Controls" tab that shows currently bound keys from `settings.toml`.
- [ ] **[MM-403]** Final Design Review
  - Polish colors, fonts, and alignment to match AAA standards.
  