# Tasks: Configuration Management System Implementation

## Phase 1: Foundation & Specs Compliance

- [ ] Add dependencies to `Cargo.toml` (`serde`, `toml`, `ron`, `directories`, `notify`, `dotenvy`) <!-- id: 0 -->
- [ ] Create `src/config/mod.rs` with nested structs (`GraphicsConfig`, `AudioConfig`, `GameConfig`) <!-- id: 1 -->
- [ ] Implement `version: u32` in `GameConfig` for migration support <!-- id: 2 -->
- [ ] Implement `Default` traits and validation (scaling/clamping) logic <!-- id: 3 -->

## Phase 2: Robust IO & Persistence

- [ ] Implement cross-platform path resolution using `directories` (targeting `%APPDATA%` on Windows) <!-- id: 4 -->
- [ ] Implement atomic save logic (write to `.tmp` then rename) to prevent corruption <!-- id: 5 -->
- [ ] Implement `migrate_config` function (handles upgrade paths for `version`) <!-- id: 6 -->
- [ ] Implement error handling: backup corrupt files to `.bak` and reset to default <!-- id: 7 -->

## Phase 3: Reactive Bevy Integration

- [ ] Create `ConfigPlugin` to manage configuration as Bevy `Resource`s <!-- id: 8 -->
- [ ] Implement `apply_graphics_settings` (reacts to `Changed<GameConfig>` to update window) <!-- id: 9 -->
- [ ] Implement `apply_audio_settings` (reacts to `Changed<GameConfig>` to update volume) <!-- id: 10 -->
- [ ] Update `main.rs` to initialize window from `GameConfig` values instead of hardcoded strings <!-- id: 11 -->

## Phase 4: Sync & Developer Experience

- [ ] Implement `DevConfig` logic wrapped in `#[cfg(debug_assertions)]` <!-- id: 12 -->
- [ ] Integrate `DevConfig` with RON asset loader for auto-reloading from `assets/` <!-- id: 13 -->
- [ ] Implement "Memory Wins" synchronization system for `GameConfig` persistence <!-- id: 14 -->
- [ ] Add `config.toml` file watcher for hot-reload in debug mode <!-- id: 15 -->

## Phase 5: Verification & Security

- [ ] Create `assets/config/default_config.toml` with informative comments <!-- id: 16 -->
- [ ] Add unit tests for version migrations and boundary value validation <!-- id: 17 -->
- [ ] Verify `.env` secrets remain isolated and NEVER leak into `config.toml` <!-- id: 18 -->
- [ ] Verify that developer configs are stripped in release builds <!-- id: 19 -->
