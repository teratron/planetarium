# Architecture Migration — Tasks

> **Feature:** `architecture-migration`
> **Plan:** [plan.md](plan.md)
> **Created:** 2026-02-12
> **Status:** 🔴 Not Started

---

## Phase 1 — Foundation: `framework/` skeleton + state migration

- [ ] **TASK-AM-001**: Create `src/framework/mod.rs` with sub-module declarations
  - Declare modules: `states`, `plugin`
  - Re-export `FrameworkPlugin` from `plugin.rs`
  - **Depends on:** none

- [ ] **TASK-AM-002**: Create `src/framework/plugin.rs` with `FrameworkPlugin` struct
  - Empty plugin struct implementing `Plugin` for `App`
  - Will be populated as sub-modules are migrated
  - **Depends on:** TASK-AM-001

- [ ] **TASK-AM-003**: Create `src/framework/states/mod.rs` and `app_state.rs`
  - Move `AppState` enum from `core/states.rs` to `framework/states/app_state.rs`
  - Add new variants: `Paused`, `Settings`, `GameOver`
  - Move `ErrorState` resource alongside
  - **Depends on:** TASK-AM-001

- [ ] **TASK-AM-004**: Create `src/framework/states/transition.rs`
  - Define reusable state transition helper systems
  - E.g. `check_splash_complete`, `check_loading_complete`
  - **Depends on:** TASK-AM-003

- [ ] **TASK-AM-005**: Update all imports across codebase to use `framework::states`
  - Replace `crate::core::states::AppState` → `crate::framework::states::AppState`
  - Files to update: `main.rs`, `launcher/*.rs`, `game/*.rs`
  - **Depends on:** TASK-AM-003

- [ ] **TASK-AM-006**: Remove legacy `core/states.rs` (or add re-export shim)
  - If desired, keep `core/states.rs` as a thin re-export for backward compat
  - **Depends on:** TASK-AM-005

- [ ] **TASK-AM-007**: ✅ Verification — Phase 1
  - Run `cargo check`, `cargo test`, `cargo clippy -- -D warnings`
  - **Depends on:** TASK-AM-006

---

## Phase 2 — Migrate launcher infrastructure to `framework/`

- [ ] **TASK-AM-008**: Move `launcher/boot.rs` → `framework/boot/`
  - Split into `mod.rs`, `systems.rs` following ECS pattern
  - Register `BootPlugin` in `FrameworkPlugin`
  - **Depends on:** TASK-AM-007

- [ ] **TASK-AM-009**: Move `launcher/splash.rs` → `framework/splash/`
  - Split into `mod.rs` (SplashPlugin), `systems.rs`, `components.rs`, `resources.rs`
  - **Depends on:** TASK-AM-007

- [ ] **TASK-AM-010**: Move `launcher/loading.rs` → `framework/loading/`
  - Split into `mod.rs` (LoadingPlugin), `systems.rs`, `resources.rs`, `assets.rs`
  - **Depends on:** TASK-AM-007

- [ ] **TASK-AM-011**: Move `launcher/error.rs` → `framework/error/`
  - Split into `mod.rs` (ErrorPlugin), `systems.rs`
  - **Depends on:** TASK-AM-007

- [ ] **TASK-AM-012**: Move `launcher/diagnostics.rs` → `framework/diagnostics/`
  - Split into `mod.rs` (DiagnosticsPlugin), `systems.rs`
  - **Depends on:** TASK-AM-007

- [ ] **TASK-AM-013**: Update `FrameworkPlugin` to aggregate all migrated sub-plugins
  - Register: BootPlugin, SplashPlugin, LoadingPlugin, ErrorPlugin, DiagnosticsPlugin
  - **Depends on:** TASK-AM-008..TASK-AM-012

- [ ] **TASK-AM-014**: ✅ Verification — Phase 2
  - Run `cargo check`, `cargo test`, `cargo clippy -- -D warnings`
  - **Depends on:** TASK-AM-013

---

## Phase 3 — Migrate menu system to `framework/menu/`

- [ ] **TASK-AM-015**: Move `launcher/menu/` → `framework/menu/`
  - Restructure: create `framework/menu/main_menu/` from `menu/screen.rs`, `menu/reactive.rs`, `menu/layout.rs`
  - Keep `framework/menu/mod.rs` as `MenuPlugin`
  - **Depends on:** TASK-AM-014

- [ ] **TASK-AM-016**: Move `launcher/menu/settings/` → `framework/settings/`
  - Create standalone `SettingsPlugin`
  - Restructure into: `mod.rs`, `systems.rs`, `components.rs`, `layout.rs`, `ui.rs`, `tabs/`
  - **Depends on:** TASK-AM-015

- [ ] **TASK-AM-017**: Move `game/pause_menu/` → `framework/menu/pause_menu/`
  - Pause menu is UI infrastructure, belongs in framework
  - Migrate: `components.rs`, `input.rs`, `state.rs`, `systems.rs`, `settings_bridge.rs`, `ui.rs`
  - **Depends on:** TASK-AM-015

- [ ] **TASK-AM-018**: Update `FrameworkPlugin` to register `MenuPlugin`, `SettingsPlugin`
  - **Depends on:** TASK-AM-015..TASK-AM-017

- [ ] **TASK-AM-019**: ✅ Verification — Phase 3
  - Run `cargo check`, `cargo test`, `cargo clippy -- -D warnings`
  - **Depends on:** TASK-AM-018

---

## Phase 4 — Migrate UI infrastructure to `framework/ui/`

- [ ] **TASK-AM-020**: Move `ui/fading.rs` → `framework/ui/fading.rs`
  - **Depends on:** TASK-AM-019

- [ ] **TASK-AM-021**: Move `ui/theme/` → `framework/ui/theme/`
  - Includes: `mod.rs`, `colors.rs`, `constants.rs`, `metrics.rs`
  - **Depends on:** TASK-AM-019

- [ ] **TASK-AM-022**: Move `launcher/menu/widgets/` → `framework/ui/widgets/`
  - Includes: `mod.rs`, `base.rs`, `buttons.rs`, `components.rs`, `constants.rs`, `dropdowns.rs`, `sliders.rs`
  - **Depends on:** TASK-AM-019

- [ ] **TASK-AM-023**: Create `framework/ui/styles.rs` and `framework/ui/layout.rs`
  - Define shared UI style constants and layout helpers
  - **Depends on:** TASK-AM-020..TASK-AM-022

- [ ] **TASK-AM-024**: Remove old `src/ui/` module
  - Remove from `lib.rs`
  - **Depends on:** TASK-AM-023

- [ ] **TASK-AM-025**: Update all UI imports across codebase
  - Replace `crate::ui::fading` → `crate::framework::ui::fading`
  - Replace `crate::ui::theme` → `crate::framework::ui::theme`
  - Replace `crate::launcher::menu::widgets` → `crate::framework::ui::widgets`
  - **Depends on:** TASK-AM-024

- [ ] **TASK-AM-026**: ✅ Verification — Phase 4
  - Run `cargo check`, `cargo test`, `cargo clippy -- -D warnings`
  - **Depends on:** TASK-AM-025

---

## Phase 5 — Restructure `game/` following ECS patterns

- [ ] **TASK-AM-027**: Create `game/plugin.rs` (extract plugin logic from `game/mod.rs`)
  - `GamePlugin` struct with proper system registration
  - **Depends on:** TASK-AM-026

- [ ] **TASK-AM-028**: Create `game/components/mod.rs`
  - Extract `GameWorldRoot`, `Rotates` from `world.rs`
  - Add placeholder component files for future game components
  - **Depends on:** TASK-AM-027

- [ ] **TASK-AM-029**: Create `game/systems/mod.rs`, `setup.rs`, `gameplay.rs`
  - Move `setup_game_world` → `game/systems/setup.rs`
  - Move `rotate_planet` → `game/systems/gameplay.rs`
  - Move `cleanup_game_world` → `game/systems/setup.rs` (or cleanup.rs)
  - **Depends on:** TASK-AM-028

- [ ] **TASK-AM-030**: Create `game/entities/mod.rs`
  - Extract entity spawning logic from `setup_game_world` into spawner functions
  - E.g. `spawn_planet()`, `spawn_light()`, `spawn_game_camera()`
  - **Depends on:** TASK-AM-029

- [ ] **TASK-AM-031**: Create `game/resources/mod.rs` and `game/constants.rs`
  - Stub resource files for future game state
  - Define game constants
  - **Depends on:** TASK-AM-027

- [ ] **TASK-AM-032**: Remove legacy `game/world.rs`, update `game/mod.rs`
  - **Depends on:** TASK-AM-028..TASK-AM-031

- [ ] **TASK-AM-033**: ✅ Verification — Phase 5
  - Run `cargo check`, `cargo test`, `cargo clippy -- -D warnings`
  - **Depends on:** TASK-AM-032

---

## Phase 6 — Restructure `core/` → `config/` + `utils/`

- [ ] **TASK-AM-034**: Create `src/config/mod.rs` — top-level config module
  - Move `core/config/` contents: `metadata.rs`, `paths.rs`, `settings.rs`
  - Move `core/cli.rs` → `config/cli.rs`
  - **Depends on:** TASK-AM-033

- [ ] **TASK-AM-035**: Create `src/utils/mod.rs` — shared utilities
  - Move `core/single_instance.rs` → `utils/single_instance.rs`
  - Create stub `utils/math.rs` and `utils/debug.rs`
  - **Depends on:** TASK-AM-033

- [ ] **TASK-AM-036**: Move `core/localization/` → `framework/localization/`
  - Localization is framework-level infrastructure
  - **Depends on:** TASK-AM-033

- [ ] **TASK-AM-037**: Consolidate `core/assets/` into `framework/loading/`
  - Merge asset manifest logic into loading module
  - **Depends on:** TASK-AM-033

- [ ] **TASK-AM-038**: Remove `core/` module and `src/assets/` module
  - Remove from `lib.rs`
  - **Depends on:** TASK-AM-034..TASK-AM-037

- [ ] **TASK-AM-039**: Update all imports from `crate::core::*`
  - `crate::core::config` → `crate::config`
  - `crate::core::cli` → `crate::config::cli`
  - `crate::core::single_instance` → `crate::utils::single_instance`
  - `crate::core::localization` → `crate::framework::localization`
  - **Depends on:** TASK-AM-038

- [ ] **TASK-AM-040**: ✅ Verification — Phase 6
  - Run `cargo check`, `cargo test`, `cargo clippy -- -D warnings`
  - **Depends on:** TASK-AM-039

---

## Phase 7 — New framework modules: camera, audio

- [ ] **TASK-AM-041**: Create `framework/camera/mod.rs` and `orbit.rs`
  - Move `setup_camera` and `diagnose_cameras` from `main.rs`
  - Create `CameraPlugin` with proper state-dependent scheduling
  - **Depends on:** TASK-AM-040

- [ ] **TASK-AM-042**: Create `framework/audio/mod.rs`, `systems.rs`, `resources.rs`
  - Stub `AudioPlugin` with `AudioSettings` resource
  - Prepare for future audio integration
  - **Depends on:** TASK-AM-040

- [ ] **TASK-AM-043**: Register `CameraPlugin` and `AudioPlugin` in `FrameworkPlugin`
  - **Depends on:** TASK-AM-041, TASK-AM-042

- [ ] **TASK-AM-044**: ✅ Verification — Phase 7
  - Run `cargo check`, `cargo test`, `cargo clippy -- -D warnings`
  - **Depends on:** TASK-AM-043

---

## Phase 8 — Entry point update + final integration

- [ ] **TASK-AM-045**: Update `lib.rs` — new module declarations
  - Modules: `framework`, `game`, `config`, `utils`
  - Remove: `core`, `launcher`, `ui`, `assets`
  - **Depends on:** TASK-AM-044

- [ ] **TASK-AM-046**: Update `main.rs` — use `FrameworkPlugin` + `GamePlugin`
  - Clean up: remove inline camera systems (moved to framework/camera)
  - Simplify `build_app()` function
  - **Depends on:** TASK-AM-045

- [ ] **TASK-AM-047**: Remove old `launcher/` module entirely
  - **Depends on:** TASK-AM-046

- [ ] **TASK-AM-048**: Dead code cleanup
  - Find and remove all unused imports, dead re-exports
  - Run `cargo clippy -- -D warnings`
  - **Depends on:** TASK-AM-047

- [ ] **TASK-AM-049**: ✅ Verification — Phase 8
  - Run `cargo check`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt`
  - **Depends on:** TASK-AM-048

---

## Phase 9 — Versioning, docs, completion

- [ ] **TASK-AM-050**: SemVer bump: `0.2.0` → `0.3.0` in `Cargo.toml`
  - Apply `rust-semver` skill for validation
  - **Depends on:** TASK-AM-049

- [ ] **TASK-AM-051**: Update `CHANGELOG.md`
  - Document the architectural migration
  - **Depends on:** TASK-AM-050

- [ ] **TASK-AM-052**: Update `README.md` project structure section (if applicable)
  - **Depends on:** TASK-AM-050

- [ ] **TASK-AM-053**: Verify architecture diagrams match actual structure
  - Update `.mermaid` files in `docs/development/architecture/` if needed
  - **Depends on:** TASK-AM-050

- [ ] **TASK-AM-054**: Update integration tests to reflect new module paths
  - `tests/diagnostics_integration.rs`
  - `tests/menu_navigation.rs`
  - `tests/pause_menu_navigation.rs`
  - `tests/widgets_integration.rs`
  - **Depends on:** TASK-AM-049

- [ ] **TASK-AM-055**: ✅ Final Verification
  - Full `cargo check`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt`
  - Manual smoke test: launch app → splash → menu → game → pause → menu
  - **Depends on:** TASK-AM-050..TASK-AM-054

- [ ] **TASK-AM-056**: Propose git tag `v0.3.0`
  - Commit message: `refactor!: migrate to two-layer framework/game architecture`
  - **Depends on:** TASK-AM-055

---

## Summary

| Phase | Tasks | Description |
|-------|-------|-------------|
| 1 | AM-001 — AM-007 | Framework skeleton + state migration |
| 2 | AM-008 — AM-014 | Launcher → framework migration |
| 3 | AM-015 — AM-019 | Menu system restructuring |
| 4 | AM-020 — AM-026 | UI infrastructure consolidation |
| 5 | AM-027 — AM-033 | Game module ECS restructuring |
| 6 | AM-034 — AM-040 | Core → config/utils decomposition |
| 7 | AM-041 — AM-044 | New camera + audio modules |
| 8 | AM-045 — AM-049 | Entry point + final integration |
| 9 | AM-050 — AM-056 | Versioning, docs, completion |
| **Total** | **56 tasks** | **9 phases** |
