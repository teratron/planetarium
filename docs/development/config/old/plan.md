# Implementation Plan: Configuration Management System (Validated)

This plan outlines the implementation of a reactive, dual-layer configuration system for Planetarium, adhering to the validated design in `docs/plans/2026-01-30-config-system-design.md`.

## 1. Infrastructure & Dependencies

- **Core Dependencies**:
  - `serde = { version = "1.0", features = ["derive"] }`
  - `toml = "0.8"` (Player config)
  - `ron = "0.8"` (Dev config)
  - `directories = "5.0"` (Platform paths)
  - `notify = "6.1"` (File watching for debug sync)
  - `dotenvy = "0.15"` (Secrets)

## 2. Configuration Schema (`src/config/mod.rs`)

- Implement nested structures: `GraphicsConfig`, `AudioConfig`, `GameConfig`.
- Add `version: u32` to `GameConfig`.
- Implement `Default` and `PartialEq` for all structs.
- Implement `DevConfig` guarded by `#[cfg(debug_assertions)]`.

## 3. Storage & IO Logic

- **Path Resolution**: Use `directories::ProjectDirs` to target `AppData/Roaming` on Windows.
- **Robustness**:
  - Save to `.tmp` then rename to prevent corruption.
  - Backup corrupt files as `.bak`.
  - Implement `migrate_config` function to handle version bumps.
- **Reactive Sync**:
  - Load-on-startup logic.
  - Save-on-change logic (using Bevy `Changed<GameConfig>`).

## 4. Bevy Integration

- **ConfigPlugin**:
  - Register `GameConfig` and `DevConfig` as resources.
  - Setup `FixedUpdate` systems to apply settings (Window size, Audio volume).
- **Initialization**: Update `main.rs` to use configurations during `DefaultPlugins` setup.
- **Event Bus**: (Optional if `Changed<T>` is enough) for complex apply logic.

## 5. Developer Tools

- Implement `F5` hot-reload for `GameConfig`.
- Use Bevy's `AssetServer` for auto-reloading `assets/dev_config.ron`.

## 6. Verification

- Test migration from version 1 to version 2.
- Verify that manually editing `config.toml` updates the game (in debug).
- Verify that changing settings in-game updates `config.toml` instantly.
