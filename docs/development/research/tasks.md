# Research Tasks

## Phase 2: Foundational

- [x] T001 Implement safe fallback font load handling in `src/ui/theme/mod.rs`
- [x] T002 Add debounced settings change handling in `src/launcher/menu/reactive.rs`
- [x] T003 Add run conditions for settings-related systems in `src/launcher/menu/mod.rs`
- [x] T004 Ensure typed settings keys are used end-to-end in `src/core/config/settings.rs`
- [x] T005 Split settings module into focused submodules under `src/launcher/menu/settings/`
- [x] T006 Add a base widget trait and implementations in `src/launcher/menu/widgets/`
- [x] T007 Add localization cache in `src/core/localization/mod.rs`
- [x] T008 Centralize UI constants in `src/ui/theme/constants.rs`
- [x] T009 Implement lazy asset cache in `src/core/assets/mod.rs`

## Final Phase: Polish & Cross-Cutting Concerns

- [x] T010 Add menu-related rustdoc examples in `src/launcher/menu/screen.rs` and `src/launcher/menu/settings/ui.rs`
- [x] T011 Add integration tests for menu navigation in `tests/menu_navigation.rs`
- [x] T012 Add property-based tests for settings ranges in `src/core/config/settings.rs`
- [x] T013 Batch UI updates for settings controls in `src/launcher/menu/settings/systems.rs`
