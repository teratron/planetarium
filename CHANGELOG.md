# Changelog

All notable changes to this project will be documented in this file.

## Unreleased

### Added
- feat(config): Add `GraphicsSettings` and `Quality` enum to `UserSettings` (adds graphics presets support).
- feat(localization): Add `apply_language_change_system` to support on-the-fly language switching.

### Fixed
- fix(theme): Safe handling when creating embedded fallback font (no panics on failure).
- fix(game): Graceful handling when procedural sphere mesh generation fails.

### Chores
- chore(ci): Address `clippy` warnings and improve tests for widgets and diagnostics.

## Suggested Release
- Version: `v0.2.0` (MINOR) — backward-compatible API additions

---

(Generated automatically by the agent during MM-103/MM-201/MM-302 work.)