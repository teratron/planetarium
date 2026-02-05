# Implementation Plan: Main Menu & Settings

## 1. Core Structural Fixes

Before implementing new features, we must stabilize the asset loading and path resolution.

### 1.1 Robust Path Resolution

- Modify `setup_asset_manifest` and other loading systems to use a base path derived from `std::env::current_exe()`.
- Ensure `assets/` is found even when running the build directly from `target/debug/`.

### 1.2 Localization integration

- Create `assets/locales/en-US/text/menu.ftl` and `assets/locales/ru-RU/text/menu.ftl`.
- Replace hardcoded strings in `src/launcher/menu/screen.rs` and `settings.rs` with Fluent keys.

---

## 2. Main Menu Refactoring

### 2.1 UI Layout

- Update `spawn_main_menu` to use advanced `bevy_ui` nodes.
- Implement a responsive centered panel with the game title and primary buttons.
- Add `ButtonHoverState` components to all buttons for visual reactivity.

### 2.2 Audio Integration

- Register `hover.ogg` and `click.ogg` in the asset manifest.
- Add systems to play these sounds on relevant `Interaction` changes.

---

## 3. Settings Sub-system (AAA Upgrade)

### 3.1 Categorized Navigation (Tabs)

- Create a reusable `TabButton` widget.
- Implement a state-based system to switch between content panels:
  - **Graphics**: Resolution, VSync, Fullscreen, Quality Presets.
  - **Audio**: Separate sliders for Master, Music, SFX, UI, Ambience.
  - **General**: Language selection, Theme selection.
  - **Controls**: Display current keybindings (read-only for now).

### 3.2 Reactive Configuration Bridge

- Define `SettingChangedEvent { group: String, key: String, value: ConfigValue }`.
- System: `apply_settings_changes` to update engine state.
- System: `persist_settings_system` to save to disk with a throttle (to avoid excessive IO during slider movement).

---

## 4. Visual Polish & Transitions

### 4.1 Fading Transitions

- Use the existing `ScreenFade` system or extend it for modular panel transitions.
- Implement an alpha-layer that covers the UI during sub-menu transitions.

---

## 5. Roadmap

1. **Cleanup & Stability**: Fix path issues and missing `assets.toml` reading.
2. **Localization Layer**: Migrate all UI text to Fluent.
3. **Widget Upgrades**: Create professional Slider, Toggle, and Tab widgets.
4. **Settings Rewrite**: Implement the tabbed settings interface synced with `UserSettings`.
5. **Audio & SFX**: Wire up interaction sounds.
6. **Final Polish**: Fades and hover animations.
