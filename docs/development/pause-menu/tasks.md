# Task List: In-Game Pause Menu (ESC)

## Phase 1: Setup

- [x] T001 Create pause menu module skeleton in `src/game/pause_menu/mod.rs`
- [x] T002 Wire `PauseMenuPlugin` into `src/game/mod.rs`

## Phase 2: Foundational

- [x] T003 Add pause UI state/resource and events in `src/game/pause_menu/state.rs`
- [x] T004 Add ESC input handling system in `src/game/pause_menu/input.rs`
- [x] T005 Define pause menu UI components in `src/game/pause_menu/components.rs`

## Phase 3: User Stories

### US1 (Open/Close + Resume)

- [x] T006 [US1] Spawn pause menu UI on open in `src/game/pause_menu/ui.rs`
- [x] T007 [US1] Implement Resume action and UI teardown in `src/game/pause_menu/systems.rs`
- [x] T008 [US1] Add run conditions to pause gameplay systems in `src/game/world.rs`

### US2 (Settings)

- [x] T009 [US2] Reuse settings UI from launcher menu in `src/game/pause_menu/settings_bridge.rs`
- [x] T010 [US2] Handle return flow from settings to pause menu in `src/game/pause_menu/systems.rs`

### US3 (Exit Actions)

- [x] T011 [US3] Implement Exit to Main Menu action in `src/game/pause_menu/systems.rs`
- [x] T012 [US3] Implement Exit Game action (AppExit) in `src/game/pause_menu/systems.rs`

## Final Phase: Polish & Cross-Cutting Concerns

- [x] T013 [P] Add localization strings to `assets/locales/en-US/menu.ftl`
- [x] T014 [P] Add localization strings to `assets/locales/ru-RU/menu.ftl`
- [x] T015 Add audio feedback hooks for pause menu buttons in `src/game/pause_menu/systems.rs`
- [x] T016 Add integration tests for pause menu navigation in `tests/pause_menu_navigation.rs`
