# Architecture Conformance Migration — Manual Cleanup Steps

## Overview

This document lists the files that **must be manually deleted** to complete
the architecture migration. The refactoring created new directory-based modules
per the `bevy_architecture_guide.md` standard, but Rust does not allow both
`module.rs` and `module/mod.rs` to coexist.

## Files to DELETE

Run these commands from the project root:

```powershell
# Phase 1: States — old app_state.rs (replaced by state.rs)
Remove-Item -Path "D:\Projects\src\github.com\teratron\planetarium\src\framework\states\app_state.rs"

# Phase 2: Splash — old monolithic file (replaced by splash/ directory)
Remove-Item -Path "D:\Projects\src\github.com\teratron\planetarium\src\framework\splash.rs"

# Phase 3: Loading — old monolithic file (replaced by loading/ directory)
Remove-Item -Path "D:\Projects\src\github.com\teratron\planetarium\src\framework\loading.rs"

# Phase 4: Game — old monolithic world.rs (replaced by components/, systems/, plugin.rs)
Remove-Item -Path "D:\Projects\src\github.com\teratron\planetarium\src\game\world.rs"

# Phase 4: Game — deprecated pause_menu shim (canonical code lives in framework/menu/pause/)
Remove-Item -Recurse -Path "D:\Projects\src\github.com\teratron\planetarium\src\game\pause_menu"
```

## Verification

After deleting those files, run:

```powershell
cargo check
cargo test
cargo clippy -- -D warnings
```

## What Changed (Summary)

| Old Location | New Location | Status |
|---|---|---|
| `framework/states/app_state.rs` | `framework/states/state.rs` | Created, old needs DELETE |
| `framework/splash.rs` | `framework/splash/mod.rs` + `systems.rs` + `components.rs` + `resources.rs` | Created, old needs DELETE |
| `framework/loading.rs` | `framework/loading/mod.rs` + `systems.rs` + `components.rs` + `resources.rs` | Created, old needs DELETE |
| `game/mod.rs` | Updated (ECS structure) | Done |
| `game/world.rs` (monolithic) | `game/components/` + `game/systems/` + `game/plugin.rs` | Created, old needs DELETE |
| `game/pause_menu/` (deprecated copy) | Use `framework/menu/pause/` directly | Old needs DELETE |
| — | `game/resources/mod.rs` (placeholder) | Created |
| — | `game/entities/mod.rs` (placeholder) | Created |
| — | `game/constants.rs` | Created |
