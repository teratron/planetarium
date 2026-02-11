# Tasks: Configuration Management System Implementation

## Phase 1: Foundation

- [ ] Add dependencies to `Cargo.toml` (`serde`, `toml`, `ron`, `directories`, `notify`, `dotenvy`) <!-- id: 0 -->
- [ ] Create `src/config/mod.rs` with versioned `GameConfig` and nested structs <!-- id: 1 -->
- [ ] Implement `Default` and `Validation` logic (clamping values) <!-- id: 2 -->

## Phase 2: Input/Output & Robustness

- [ ] Implement `migrate_config` function to handle version upgrades <!-- id: 18 -->
- [ ] Implement safe atomic saving (write to `.tmp` then rename) <!-- id: 19 -->
- [ ] Implement error handling (backup corrupt files to `.bak`) <!-- id: 17 -->
- [ ] Implement cross-platform path resolution using `directories` <!-- id: 3 -->

## Phase 3: Bevy Integration (Reactive)

- [ ] Create `ConfigPlugin` and register resources <!-- id: 7 -->
- [ ] Implement `apply_graphics_settings` system (responds to `Changed<GameConfig>`) <!-- id: 16 -->
- [ ] Implement `apply_audio_settings` system (responds to `Changed<GameConfig>`) <!-- id: 9 -->
- [ ] Update `main.rs` to initialize the window using `GameConfig` defaults <!-- id: 8 -->

## Phase 4: Sync & Dev Tools

- [ ] Implement `DevConfig` with RON asset loader integration <!-- id: 10 -->
- [ ] Implement "Memory Wins" sync system for `GameConfig` persistence <!-- id: 20 -->
- [ ] Add file watcher for `config.toml` to support hot-reload (debug only) <!-- id: 11 -->

## Phase 5: Assets & Testing

- [ ] Create `assets/config/default_config.toml` template <!-- id: 13 -->
- [ ] Write unit tests for version migrations and value clamping <!-- id: 14 -->
- [ ] Verify `.env` secrets remain isolated from `GameConfig` <!-- id: 21 -->
