# Architecture Migration Plan

## Overview

Migration of the current project structure (`core/` + `launcher/` + `ui/` + `game/`) to the new **Two-Layer Architecture** (`framework/` + `game/`) as defined in `docs/development/architecture/bevy_architecture_guide.md`.

## Scope Summary

| Metric | Value |
|--------|-------|
| Affected modules | `core/`, `launcher/`, `ui/`, `game/`, `assets/`, `main.rs`, `lib.rs` |
| Files to move/restructure | ~40+ |
| New modules to create | `framework/`, `config/`, `utils/` |
| Modules to remove | `core/`, `launcher/`, `src/assets/` |
| Estimated SemVer impact | **MINOR** (0.2.0 → 0.3.0, pre-1.0) |
| Risk level | **HIGH** — near-total structural overhaul |

## Current vs. Target Mapping

```
CURRENT                              TARGET
───────────────────────────────────  ───────────────────────────────────
src/                                 src/
├── main.rs                          ├── main.rs (updated imports)
├── lib.rs                           ├── lib.rs  (updated modules)
├── assets/mod.rs (empty)            │   (removed)
├── core/                            │   (removed — split below)
│   ├── states.rs        ──────────► ├── framework/states/app_state.rs
│   ├── cli.rs           ──────────► │   config/cli.rs
│   ├── config/          ──────────► │   config/
│   ├── single_instance  ──────────► │   utils/single_instance.rs
│   ├── localization/    ──────────► │   framework/localization/
│   └── assets/mod.rs    ──────────► │   framework/loading/assets.rs
├── launcher/                        │   (removed — merged into framework)
│   ├── boot.rs          ──────────► ├── framework/boot/
│   ├── splash.rs        ──────────► │   framework/splash/
│   ├── loading.rs       ──────────► │   framework/loading/
│   ├── error.rs         ──────────► │   framework/error/
│   ├── diagnostics.rs   ──────────► │   framework/diagnostics/
│   └── menu/            ──────────► │   framework/menu/
│       ├── settings/    ──────────► │   framework/settings/
│       └── widgets/     ──────────► │   framework/ui/widgets/
├── ui/                              │   (removed — merged into framework)
│   ├── fading.rs        ──────────► │   framework/ui/fading.rs
│   └── theme/           ──────────► │   framework/ui/theme/
├── game/                            ├── game/
│   ├── pause_menu/      ──────────► │   framework/menu/pause_menu/
│   └── world.rs         ──────────► │   game/{components,systems,entities}/
│                                    ├── config/   (from core/config + core/cli)
│                                    └── utils/    (from core/single_instance)
```

## Phase Breakdown

### Phase 1 — Foundation: `framework/` skeleton + state migration

**Goal:** Create the `framework/` module skeleton and migrate `AppState`.

- Create `src/framework/mod.rs` and `src/framework/plugin.rs`
- Create `src/framework/states/mod.rs` and `app_state.rs`
- Move `core/states.rs` → `framework/states/app_state.rs`
- Add missing states: `Paused`, `Settings`, `GameOver`
- Create `framework/states/transition.rs` (state transition helpers)
- Update all imports across codebase
- Verify: `cargo check`, `cargo test`

### Phase 2 — Migrate launcher infrastructure to `framework/`

**Goal:** Move all `launcher/` sub-modules into `framework/`.

- Move `launcher/boot.rs` → `framework/boot/` (systems.rs, resources.rs pattern)
- Move `launcher/splash.rs` → `framework/splash/` (split by ECS pattern)
- Move `launcher/loading.rs` → `framework/loading/` (split by ECS pattern)
- Move `launcher/error.rs` → `framework/error/`
- Move `launcher/diagnostics.rs` → `framework/diagnostics/`
- Update `FrameworkPlugin` to register all sub-plugins
- Remove `launcher/` module
- Verify: `cargo check`, `cargo test`

### Phase 3 — Migrate menu system to `framework/menu/`

**Goal:** Restructure menu, settings, and widgets under `framework/`.

- Move `launcher/menu/` → `framework/menu/`
- Move `launcher/menu/settings/` → `framework/settings/`
- Move `game/pause_menu/` → `framework/menu/pause_menu/`
- Move `launcher/menu/widgets/` → `framework/ui/widgets/`
- Create `framework/menu/main_menu/` and `framework/menu/pause_menu/`
- Verify: `cargo check`, `cargo test`

### Phase 4 — Migrate UI infrastructure to `framework/ui/`

**Goal:** Consolidate all UI utilities under `framework/ui/`.

- Move `ui/fading.rs` → `framework/ui/fading.rs`
- Move `ui/theme/` → `framework/ui/theme/`
- Create `framework/ui/styles.rs` (UI style constants)
- Create `framework/ui/layout.rs` (layout helpers)
- Remove `src/ui/` module
- Verify: `cargo check`, `cargo test`

### Phase 5 — Restructure `game/` following ECS patterns

**Goal:** Decompose game logic into proper ECS directories.

- Create `game/plugin.rs` (extract from mod.rs)
- Create `game/components/mod.rs` with `GameWorldRoot`, `Rotates`, etc.
- Create `game/systems/mod.rs`, `setup.rs`, `gameplay.rs`
- Create `game/entities/mod.rs` (entity spawner functions)
- Create `game/resources/mod.rs`
- Create `game/constants.rs`
- Split `world.rs` across components/systems/entities
- Verify: `cargo check`, `cargo test`

### Phase 6 — Restructure `core/` → `config/` + `utils/`

**Goal:** Eliminate the `core/` module, distributing its contents.

- Move `core/config/` → `config/` (top-level src module)
- Move `core/cli.rs` → `config/cli.rs`
- Move `core/single_instance.rs` → `utils/single_instance.rs`
- Move `core/localization/` → `framework/localization/`
- Move `core/assets/` → consolidate into `framework/loading/`
- Remove `core/` module
- Remove `src/assets/` (empty module)
- Create `utils/mod.rs` with math.rs, debug.rs stubs
- Verify: `cargo check`, `cargo test`

### Phase 7 — New framework modules: camera, audio

**Goal:** Create standard framework modules for camera and audio.

- Create `framework/camera/mod.rs`, `orbit.rs`
- Move camera setup from `main.rs` → `framework/camera/`
- Create `framework/audio/mod.rs`, `systems.rs`, `resources.rs`
- Register in `FrameworkPlugin`
- Verify: `cargo check`, `cargo test`

### Phase 8 — Entry point update + final integration

**Goal:** Update `main.rs` and `lib.rs` for the new structure.

- Update `lib.rs` module declarations: `framework`, `game`, `config`, `utils`
- Update `main.rs` to use `FrameworkPlugin` + `GamePlugin`
- Full cleanup of dead imports and unused modules
- Verify: `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `cargo fmt`

### Phase 9 — Verification, versioning, docs

**Goal:** Final quality assurance, versioning, and documentation.

- Run full test suite
- SemVer bump: 0.2.0 → 0.3.0
- Update `CHANGELOG.md`
- Update `README.md` if needed
- Update architecture diagrams to match actual structure
- Propose git tag `v0.3.0`

## Verification Strategy

After each phase:

1. `cargo check` — compilation
2. `cargo clippy -- -D warnings` — lints
3. `cargo test` — all tests pass
4. `cargo fmt --check` — formatting

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| Breaking all imports at once | Phase sequentially; keep old modules with re-exports during transition |
| Test failures | Run tests after every phase; keep backward-compatible re-exports |
| Merge conflicts | Do this on a dedicated branch |
| Forgotten references | Use `grep_search` to find all `use crate::core::`, `use crate::launcher::`, `use crate::ui::` |
| Feature regressions | Manual smoke test: launch app, check splash → menu → game → pause flow |

## Branch Strategy

- Branch: `refactor/architecture-migration`
- Base: `main`
- Merge: squash merge after all phases complete
