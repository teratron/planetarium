# Architecture Migration — Tasks

> **Feature:** `architecture-migration`
> **Plan:** [plan.md](plan.md)
> **Created:** 2026-02-12
> **Status:** 🔄 In Progress (Phase 3 ✅)

---

## Phase 1 — Foundation: `framework/` skeleton + state migration ✅

- [x] **TASK-AM-001**: Create `src/framework/mod.rs` — module root with sub-module declarations
  - **Action:** CREATE (no existing equivalent)

- [x] **TASK-AM-002**: Create `src/framework/plugin.rs` — `FrameworkPlugin` shell
  - **Action:** CREATE (no existing equivalent)

- [x] **TASK-AM-003**: Create `src/framework/states/{mod.rs, app_state.rs}`
  - **Action:** MOVE content from `core/states.rs`, add new variants: `Paused`, `Settings`, `GameOver`

- [x] **TASK-AM-004**: Create `src/framework/states/transition.rs`
  - **Action:** CREATE (no existing transition file — transitions are inline in systems)

- [x] **TASK-AM-005**: Register `framework` module in `lib.rs`
  - **Action:** ADAPT — added `pub mod framework;`

- [x] **TASK-AM-006**: Convert `core/states.rs` to re-export shim
  - **Action:** RE-EXPORT — `pub use crate::framework::states::{AppState, ErrorState};`
  - All existing imports continue working without changes

- [x] **TASK-AM-007**: ✅ Verification — `cargo check` passed

---

## Phase 2 — Move `launcher/` → `framework/`

> **Strategy:** Move files as-is (no splitting). Leave `launcher/mod.rs` as re-export shim.

- [x] **TASK-AM-008**: Move `launcher/boot.rs` → `framework/boot.rs`
  - **Action:** MOVE (53 lines, single file, well-structured — no split needed)
  - Internal imports left as `crate::core::*` (works through shims)
  - **Depends on:** Phase 1

- [x] **TASK-AM-009**: Move `launcher/splash.rs` → `framework/splash.rs`
  - **Action:** MOVE (100 lines, self-contained — no split needed)
  - Internal imports left as `crate::core::*` / `crate::ui::*` (works through shims)
  - **Depends on:** Phase 1

- [x] **TASK-AM-010**: Move `launcher/loading.rs` → `framework/loading.rs`
  - **Action:** MOVE (~9KB, could split later but not now)
  - Internal imports left as-is (through shims)
  - **Depends on:** Phase 1

- [x] **TASK-AM-011**: Move `launcher/error.rs` → `framework/error.rs`
  - **Action:** MOVE (~3.4KB, single file)
  - Updated `crate::launcher::menu::widgets` → `crate::framework::menu::widgets`
  - **Depends on:** Phase 1

- [x] **TASK-AM-012**: Move `launcher/diagnostics.rs` → `framework/diagnostics.rs`
  - **Action:** MOVE (~5.5KB, single file)
  - Internal imports left as-is (through shims)
  - **Depends on:** Phase 1

- [x] **TASK-AM-013**: Move `launcher/menu/` → `framework/menu/`
  - **Action:** MOVE entire folder including `settings/` and `widgets/` sub-folders
  - Kept `settings/` and `widgets/` inside `menu/` (not separated)
  - Updated `crate::launcher::menu::reactive` → `crate::framework::menu::reactive` in widgets
  - **Depends on:** TASK-AM-008..012

- [x] **TASK-AM-014**: Move `launcher/menu/settings/` → `framework/menu/settings/`
  - **Action:** MOVE entire folder (5 files + `tabs/` subfolder with 5 files = 10 files total)
  - Kept inside `framework/menu/` (not promoted to top-level `framework/settings/`)
  - **Depends on:** TASK-AM-013

- [x] **TASK-AM-015**: Move `launcher/menu/widgets/` → `framework/menu/widgets/`
  - **Action:** MOVE entire folder (7 files: `mod.rs`, `base.rs`, `buttons.rs`, `components.rs`, `constants.rs`, `dropdowns.rs`, `sliders.rs`)
  - Kept inside `framework/menu/` (not promoted to `framework/ui/`)
  - Updated 4 `crate::launcher::menu::reactive` references → `crate::framework::menu::reactive`
  - **Depends on:** TASK-AM-013

- [x] **TASK-AM-016**: Update `framework/mod.rs` — declare all moved sub-modules
  - **Action:** ADAPT — added `boot`, `splash`, `loading`, `error`, `diagnostics`, `menu`
  - **Depends on:** TASK-AM-008..015

- [x] **TASK-AM-017**: Update `FrameworkPlugin` — register all sub-plugins
  - **Action:** ADAPT — registered Boot, Splash, Menu, Loading, Error, Diagnostics, Fading, Theme
  - **Depends on:** TASK-AM-016

- [x] **TASK-AM-018**: Convert `launcher/mod.rs` to re-export shim
  - **Action:** RE-EXPORT — individual `pub use crate::framework::{boot,splash,menu,...}` + `LauncherPlugin` alias
  - All external imports (`planetarium::launcher::*`) continue working
  - **Depends on:** TASK-AM-017

- [x] **TASK-AM-019**: ✅ Verification — Phase 2
  - `cargo check` ✅, `cargo test` ✅ (14/15 passed, 1 pre-existing flaky test), `cargo clippy -- -D warnings` ✅
  - **Depends on:** TASK-AM-018

---

## Phase 3 — Move `ui/` → `framework/ui/`

> **Strategy:** Move files as-is into `framework/ui/`. Leave `ui/mod.rs` as re-export shim.

- [x] **TASK-AM-020**: Create `framework/ui/mod.rs`
  - **Action:** CREATE — declared `fading`, `theme` sub-modules
  - **Depends on:** Phase 2

- [x] **TASK-AM-021**: Move `ui/fading.rs` → `framework/ui/fading.rs`
  - **Action:** MOVE (~4.4KB, single file, no import changes needed)
  - **Depends on:** TASK-AM-020

- [x] **TASK-AM-022**: Move `ui/theme/` → `framework/ui/theme/`
  - **Action:** MOVE entire folder (4 files: `mod.rs`, `colors.rs`, `constants.rs`, `metrics.rs`)
  - Fixed `include_bytes!` relative path (`../../../` → `../../../../`)
  - **Depends on:** TASK-AM-020

- [x] **TASK-AM-023**: Convert `ui/mod.rs` to re-export shim
  - **Action:** RE-EXPORT — `pub use crate::framework::ui::{fading, theme}`
  - Updated `FrameworkPlugin` to use `super::ui::` instead of `crate::ui::`
  - **Depends on:** TASK-AM-021..022

- [x] **TASK-AM-024**: ✅ Verification — Phase 3
  - `cargo check` ✅, `cargo clippy -- -D warnings` ✅, `cargo test` ✅ (14/14 + 1 pre-existing flaky)
  - **Depends on:** TASK-AM-023

---

## Phase 4 — Move `game/pause_menu/` → `framework/menu/pause/`

> **Strategy:** Pause menu is UI infrastructure. Move entire folder to framework. Rename `pause_menu` → `pause` to avoid tautology inside `menu/`.

- [ ] **TASK-AM-025**: Move `game/pause_menu/` → `framework/menu/pause/`
  - **Action:** MOVE entire folder (7 files: `mod.rs`, `components.rs`, `input.rs`, `state.rs`, `systems.rs`, `settings_bridge.rs`, `ui.rs`)
  - Rename module from `pause_menu` to `pause`
  - Update internal imports (`crate::game::pause_menu` → `crate::framework::menu::pause`)
  - **Depends on:** Phase 3

- [ ] **TASK-AM-026**: Update `game/mod.rs` — remove `pause_menu` module, update `GamePlugin`
  - **Action:** ADAPT — remove PauseMenuPlugin from GamePlugin
  - **Depends on:** TASK-AM-025

- [ ] **TASK-AM-027**: Update `framework/menu/mod.rs` — register PauseMenuPlugin
  - **Action:** ADAPT — add `pub mod pause;` and register in MenuPlugin
  - **Depends on:** TASK-AM-025

- [ ] **TASK-AM-028**: ✅ Verification — Phase 4
  - Run `cargo check`, `cargo test`, `cargo clippy -- -D warnings`
  - **Depends on:** TASK-AM-026..027

---

## Phase 5 — Decompose `core/` → `config/` + `utils/` + `framework/`

> **Strategy:** Move folders as-is. Leave `core/mod.rs` as re-export shim.

- [ ] **TASK-AM-029**: Create `src/config/mod.rs` — top-level config module
  - **Action:** CREATE — new module root re-exporting moved config content
  - **Depends on:** Phase 4

- [ ] **TASK-AM-030**: Move `core/config/{metadata.rs, paths.rs, settings.rs}` → `config/`
  - **Action:** MOVE (3 files + adapt `core/config/mod.rs` logic into `config/mod.rs`)
  - Update internal imports
  - **Depends on:** TASK-AM-029

- [ ] **TASK-AM-031**: Move `core/cli.rs` → `config/cli.rs`
  - **Action:** MOVE (single file)
  - Update internal imports
  - **Depends on:** TASK-AM-029

- [ ] **TASK-AM-032**: Move `core/localization/` → `framework/localization/`
  - **Action:** MOVE entire folder (3 files: `mod.rs`, `systems.rs`, `utils.rs`)
  - Update internal imports, add to `framework/mod.rs`
  - **Depends on:** Phase 4

- [ ] **TASK-AM-033**: Move `core/assets/mod.rs` → `framework/loading/assets.rs`
  - **Action:** MOVE + ADAPT (merge with loading module)
  - Update internal imports
  - **Depends on:** Phase 4

- [ ] **TASK-AM-034**: Create `src/utils/mod.rs` — utilities module
  - **Action:** CREATE — new module root
  - **Depends on:** Phase 4

- [ ] **TASK-AM-035**: Move `core/single_instance.rs` → `utils/single_instance.rs`
  - **Action:** MOVE (single file, ~7.5KB)
  - Update internal imports
  - **Depends on:** TASK-AM-034

- [ ] **TASK-AM-036**: Convert `core/mod.rs` to re-export shim
  - **Action:** RE-EXPORT — point sub-modules to new locations
  - **Depends on:** TASK-AM-030..035

- [ ] **TASK-AM-037**: Delete `src/assets/mod.rs` (empty module)
  - **Action:** DELETE — remove from `lib.rs`
  - **Depends on:** TASK-AM-033

- [ ] **TASK-AM-038**: Update `lib.rs` module declarations
  - **Action:** ADAPT — add `config`, `utils`; keep `core` shim for now
  - **Depends on:** TASK-AM-036..037

- [ ] **TASK-AM-039**: ✅ Verification — Phase 5
  - Run `cargo check`, `cargo test`, `cargo clippy -- -D warnings`
  - **Depends on:** TASK-AM-038

---

## Phase 6 — New framework modules: camera, audio

> **Strategy:** Extract camera from `main.rs`, create audio stub.

- [ ] **TASK-AM-040**: Create `framework/camera/mod.rs` — `CameraPlugin`
  - **Action:** CREATE + MOVE `setup_camera` and `diagnose_cameras` from `main.rs`
  - **Depends on:** Phase 5

- [ ] **TASK-AM-041**: Create `framework/audio/{mod.rs, systems.rs, resources.rs}` — stub
  - **Action:** CREATE — `AudioPlugin` with `AudioSettings` resource stub
  - **Depends on:** Phase 5

- [ ] **TASK-AM-042**: Register `CameraPlugin` and `AudioPlugin` in `FrameworkPlugin`
  - **Action:** ADAPT
  - **Depends on:** TASK-AM-040..041

- [ ] **TASK-AM-043**: ✅ Verification — Phase 6
  - Run `cargo check`, `cargo test`, `cargo clippy -- -D warnings`
  - **Depends on:** TASK-AM-042

---

## Phase 7 — Cleanup, versioning, documentation

> **Strategy:** Remove all shims, finalize entry point, update docs.

- [ ] **TASK-AM-044**: Update `main.rs` — use `FrameworkPlugin` + `GamePlugin` only
  - **Action:** ADAPT — simplify `build_app()`, remove inline camera systems
  - **Depends on:** Phase 6

- [ ] **TASK-AM-045**: Remove re-export shims — replace with deprecation or delete
  - **Action:** DELETE old shim files: `core/states.rs`, `launcher/mod.rs`, `ui/mod.rs`, `core/mod.rs`
  - Update `lib.rs` to remove `core`, `launcher`, `ui` modules
  - **Depends on:** TASK-AM-044

- [ ] **TASK-AM-046**: Dead code cleanup
  - **Action:** `cargo clippy -- -D warnings`, remove unused imports
  - **Depends on:** TASK-AM-045

- [ ] **TASK-AM-047**: Update integration tests (`tests/`)
  - **Action:** ADAPT — update `use planetarium::core::*` → `use planetarium::framework::*` etc.
  - Files: `diagnostics_integration.rs`, `menu_navigation.rs`, `pause_menu_navigation.rs`, `widgets_integration.rs`
  - **Depends on:** TASK-AM-045

- [ ] **TASK-AM-048**: SemVer bump: `0.2.0` → `0.3.0` in `Cargo.toml`
  - **Action:** ADAPT — apply `rust-semver` skill for validation
  - **Depends on:** TASK-AM-046..047

- [ ] **TASK-AM-049**: Update `CHANGELOG.md`
  - **Action:** ADAPT — document the architectural migration
  - **Depends on:** TASK-AM-048

- [ ] **TASK-AM-050**: Update architecture diagrams in `docs/architecture/`
  - **Action:** ADAPT — ensure `.mermaid` files and guides match actual final structure
  - **Depends on:** TASK-AM-048

- [ ] **TASK-AM-051**: ✅ Final Verification
  - Full `cargo check`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt`
  - Manual smoke test: launch → splash → menu → game → pause → menu
  - **Depends on:** TASK-AM-048..050

- [ ] **TASK-AM-052**: Propose git tag `v0.3.0`
  - Commit message: `refactor!: migrate to two-layer framework/game architecture`
  - **Depends on:** TASK-AM-051

---

## Summary

| Phase | Tasks | Status | Description |
|-------|-------|--------|-------------|
| 1 | AM-001 — AM-007 | ✅ Done | Framework skeleton + state migration |
| 2 | AM-008 — AM-019 | ✅ Done | Move `launcher/` → `framework/` |
| 3 | AM-020 — AM-024 | ✅ Done | Move `ui/` → `framework/ui/` |
| 4 | AM-025 — AM-028 | ⬜ | Move `game/pause_menu/` → `framework/menu/pause/` |
| 5 | AM-029 — AM-039 | ⬜ | Decompose `core/` → `config/` + `utils/` |
| 6 | AM-040 — AM-043 | ⬜ | New camera + audio modules |
| 7 | AM-044 — AM-052 | ⬜ | Cleanup, versioning, docs |
| **Total** | **52 tasks** | | **7 phases** |
