# Architecture Migration Plan

## Overview

Migration of the current project structure (`core/` + `launcher/` + `ui/` + `game/`) to the new **Two-Layer Architecture** (`framework/` + `game/`) as defined in `docs/architecture/bevy_architecture_guide.md`.

## Scope Summary

| Metric | Value |
|--------|-------|
| Affected modules | `core/`, `launcher/`, `ui/`, `game/`, `assets/`, `main.rs`, `lib.rs` |
| Files to move | ~50 |
| New modules to create | `framework/`, `config/`, `utils/` |
| Modules to remove | `core/`, `launcher/`, `src/assets/` |
| Estimated SemVer impact | **MINOR** (0.2.0 → 0.3.0, pre-1.0) |
| Risk level | **HIGH** — near-total structural overhaul |

## Migration Strategy: Move-First, Re-Export Shim

> **Key principle:** Prefer **moving existing files as-is** over splitting/rewriting them.
> For each moved module, leave a **re-export shim** at the old path so
> all existing imports continue to compile. This allows a safe,
> incremental migration.

### File-level action classification

| Action | Meaning |
|--------|---------|
| **MOVE** | Copy file to new path, update internal `use`/`mod` declarations in the file |
| **RE-EXPORT** | Replace old file content with `pub use crate::new::path::*;` |
| **CREATE** | New file that has no existing equivalent |
| **DELETE** | Remove file (only after all references updated) |
| **ADAPT** | Modify existing file in-place (update imports, add module declarations) |

### Do NOT split unless necessary

The ECS split pattern (`components/`, `systems/`, `resources/`) is the **target for new code**.
Existing well-structured single-file modules (like `boot.rs` — 53 lines,
`splash.rs` — 100 lines) should be **moved as-is** and split only when
they grow large enough to warrant it.

---

## Current → Target Mapping

### `core/` decomposition

| Current | Target | Action |
|---------|--------|--------|
| `core/states.rs` | `framework/states/app_state.rs` | ✅ DONE (Phase 1) |
| `core/config/` (4 files) | `config/` (top-level) | MOVE entire folder |
| `core/cli.rs` | `config/cli.rs` | MOVE |
| `core/localization/` (3 files) | `framework/localization/` | MOVE entire folder |
| `core/assets/mod.rs` | `framework/loading/assets.rs` | MOVE + ADAPT |
| `core/single_instance.rs` | `utils/single_instance.rs` | MOVE |
| `core/mod.rs` | — | RE-EXPORT shim → DELETE later |

### `launcher/` → `framework/`

| Current | Target | Action |
|---------|--------|--------|
| `launcher/boot.rs` | `framework/boot.rs` | MOVE (single file, no split) |
| `launcher/splash.rs` | `framework/splash.rs` | MOVE (single file) |
| `launcher/loading.rs` | `framework/loading.rs` | MOVE (single file) |
| `launcher/error.rs` | `framework/error.rs` | MOVE (single file) |
| `launcher/diagnostics.rs` | `framework/diagnostics.rs` | MOVE (single file) |
| `launcher/menu/` (4 files) | `framework/menu/` | MOVE entire folder |
| `launcher/menu/settings/` (6 files + tabs/) | `framework/settings/` | MOVE entire folder |
| `launcher/menu/widgets/` (7 files) | `framework/ui/widgets/` | MOVE entire folder |
| `launcher/mod.rs` | — | RE-EXPORT shim → DELETE later |

### `ui/` → `framework/ui/`

| Current | Target | Action |
|---------|--------|--------|
| `ui/fading.rs` | `framework/ui/fading.rs` | MOVE |
| `ui/theme/` (4 files) | `framework/ui/theme/` | MOVE entire folder |
| `ui/mod.rs` | — | RE-EXPORT shim → DELETE later |

### `game/` restructure

| Current | Target | Action |
|---------|--------|--------|
| `game/pause_menu/` (7 files) | `framework/menu/pause_menu/` | MOVE entire folder |
| `game/world.rs` | `game/world.rs` (keep) | ADAPT imports only |
| `game/mod.rs` | `game/mod.rs` | ADAPT imports |

### `assets/`

| Current | Target | Action |
|---------|--------|--------|
| `assets/mod.rs` (empty) | — | DELETE |

### New files (no existing equivalent)

| File | Purpose |
|------|---------|
| `framework/mod.rs` | ✅ DONE — module root |
| `framework/plugin.rs` | ✅ DONE — FrameworkPlugin |
| `framework/states/mod.rs` | ✅ DONE — state re-exports |
| `framework/states/app_state.rs` | ✅ DONE — AppState + ErrorState |
| `framework/states/transition.rs` | ✅ DONE — transition helpers stub |
| `framework/ui/mod.rs` | CREATE — UI module root |
| `framework/ui/styles.rs` | CREATE — shared style constants (stub) |
| `framework/ui/layout.rs` | CREATE — layout helpers (stub) |
| `framework/camera/mod.rs` | CREATE — camera plugin |
| `framework/camera/orbit.rs` | CREATE — orbit camera (stub) |
| `framework/audio/mod.rs` | CREATE — audio plugin (stub) |
| `framework/audio/systems.rs` | CREATE — audio systems (stub) |
| `framework/audio/resources.rs` | CREATE — AudioSettings resource (stub) |
| `config/mod.rs` | CREATE — top-level config module root |
| `utils/mod.rs` | CREATE — utilities module root |

---

## Phase Breakdown (Revised)

### Phase 1 — Foundation: `framework/` skeleton + state migration ✅ DONE

- Created `framework/{mod.rs, plugin.rs, states/}`
- Migrated `AppState` + `ErrorState` to `framework/states/app_state.rs`
- Added new state variants: `Paused`, `Settings`, `GameOver`
- Left `core/states.rs` as re-export shim
- Verified: `cargo check` ✅

### Phase 2 — Move `launcher/` → `framework/`

**Goal:** Move all launcher sub-modules into framework. No code splitting.

1. Move `launcher/boot.rs` → `framework/boot.rs`
2. Move `launcher/splash.rs` → `framework/splash.rs`
3. Move `launcher/loading.rs` → `framework/loading.rs`
4. Move `launcher/error.rs` → `framework/error.rs`
5. Move `launcher/diagnostics.rs` → `framework/diagnostics.rs`
6. Move `launcher/menu/` → `framework/menu/` (entire folder)
7. Move `launcher/menu/settings/` → `framework/settings/` (entire folder with tabs/)
8. Move `launcher/menu/widgets/` → `framework/ui/widgets/` (entire folder)
9. Update `framework/mod.rs` to declare all moved sub-modules
10. Update `FrameworkPlugin` to register all sub-plugins
11. Create `launcher/mod.rs` as re-export shim
12. Update internal imports within moved files
13. Verify: `cargo check`, `cargo test`

### Phase 3 — Move `ui/` → `framework/ui/`

**Goal:** Consolidate UI infrastructure under framework.

1. Create `framework/ui/mod.rs`
2. Move `ui/fading.rs` → `framework/ui/fading.rs`
3. Move `ui/theme/` → `framework/ui/theme/` (entire folder)
4. Create stubs: `framework/ui/styles.rs`, `framework/ui/layout.rs`
5. Create `ui/mod.rs` as re-export shim
6. Update internal imports within moved files
7. Verify: `cargo check`, `cargo test`

### Phase 4 — Move `game/pause_menu/` → `framework/menu/pause_menu/`

**Goal:** Pause menu is UI infrastructure — belongs in framework.

1. Move `game/pause_menu/` → `framework/menu/pause_menu/` (entire folder, 7 files)
2. Update `game/mod.rs` to remove pause_menu, update GamePlugin
3. Update `framework/menu/mod.rs` to register PauseMenuPlugin
4. Update internal imports within moved files
5. Verify: `cargo check`, `cargo test`

### Phase 5 — Decompose `core/` → `config/` + `utils/` + `framework/`

**Goal:** Eliminate the `core/` module entirely.

1. Create `src/config/mod.rs` (top-level module)
2. Move `core/config/{metadata.rs, paths.rs, settings.rs}` → `config/`
3. Move `core/cli.rs` → `config/cli.rs`
4. Move `core/localization/` → `framework/localization/` (entire folder)
5. Move `core/assets/mod.rs` → `framework/loading/assets.rs`
6. Create `src/utils/mod.rs`
7. Move `core/single_instance.rs` → `utils/single_instance.rs`
8. Create `core/mod.rs` as re-export shim
9. Delete `src/assets/mod.rs` (empty)
10. Update `lib.rs` module declarations
11. Update all remaining `crate::core::*` imports
12. Verify: `cargo check`, `cargo test`

### Phase 6 — New framework modules: camera, audio

**Goal:** Create standard framework modules.

1. Create `framework/camera/mod.rs` — `CameraPlugin`
2. Move `setup_camera` + `diagnose_cameras` from `main.rs`
3. Create `framework/audio/{mod.rs, systems.rs, resources.rs}` — stub `AudioPlugin`
4. Register in `FrameworkPlugin`
5. Verify: `cargo check`, `cargo test`

### Phase 7 — Cleanup, entry point update, versioning

**Goal:** Final integration, cleanup, and release.

1. Update `main.rs` — simplify `build_app()`, use `FrameworkPlugin` + `GamePlugin`
2. Remove re-export shims (convert to errors/deprecation warnings if desired)
3. Delete old empty modules
4. Run full: `cargo check`, `cargo clippy -- -D warnings`, `cargo test`, `cargo fmt`
5. SemVer bump: 0.2.0 → 0.3.0
6. Update `CHANGELOG.md`
7. Update architecture diagrams in `docs/architecture/`
8. Update integration tests
9. Propose git tag `v0.3.0`

---

## Verification Strategy

After each phase:

1. `cargo check` — compilation
2. `cargo clippy -- -D warnings` — lints
3. `cargo test` — all tests pass
4. `cargo fmt --check` — formatting

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| Breaking all imports at once | Re-export shims at old paths keep everything compiling |
| Test failures | Run tests after every phase |
| Merge conflicts | Dedicated branch `refactor/architecture-migration` |
| Forgotten references | `grep_search` for all old import paths |
| Feature regressions | Manual smoke test: splash → menu → game → pause flow |

## Branch Strategy

- Branch: `refactor/architecture-migration`
- Base: `main`
- Merge: squash merge after all phases complete
