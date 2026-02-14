# Changelog

All notable changes to this project will be documented in this file.

## [0.3.2] - 2026-02-14

### Added

- feat(localization): Localize all hardcoded strings in logging macros (`info!`, `warn!`, `error!`).
- feat(localization): Localize loading screen UI, including rotating lore hints.
- feat(localization): Added `main_bundle_mut()` and `fallback_bundle_mut()` to `Localization` resource for dynamic asset loading.
- feat(menu): Complete Main Menu overhaul with transition fading and polished UI (MM-401, MM-403).
- feat(menu): "Controls" tab displaying keybindings (MM-402).
- feat(menu): "Graphics" tab with VSync, Fullscreen, and Resolution controls (MM-301).
- feat(config): Add `GraphicsSettings` and `Quality` enum to `UserSettings` (adds graphics presets support).
- feat(localization): Add `apply_language_change_system` to support on-the-fly language switching.

### Fixed

- fix(localization): Corrected bridge system synchronization for settings UI in pause menu.
- fix(menu): Smooth transitions for settings overlay (no immediate despawn).
- fix(theme): Safe handling when creating embedded fallback font (no panics on failure).
- fix(game): Graceful handling when procedural sphere mesh generation fails.

### Chores

- chore(ci): Address `clippy` warnings and improve tests for widgets and diagnostics.

## [0.3.1] - 2026-02-13

Internal patch and preparation for localization system updates.

## [0.3.0] - 2026-02-12

## Suggested Release

- Version: `v0.2.0` (MINOR) — backward-compatible API additions

---

(Generated automatically by the agent during MM-103/MM-201/MM-302 work.)
