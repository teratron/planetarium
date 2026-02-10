# Task List: In-Game Pause Menu (ESC)

## Phase 1: Setup

- [ ] T001 Create pause menu module skeleton in `src/game/pause_menu/mod.rs`
- [ ] T002 Wire `PauseMenuPlugin` into `src/game/mod.rs`

## Phase 2: Foundational

- [ ] T003 Add pause UI state/resource and events in `src/game/pause_menu/state.rs`
- [ ] T004 Add ESC input handling system in `src/game/pause_menu/input.rs`
- [ ] T005 Define pause menu UI components in `src/game/pause_menu/components.rs`

## Phase 3: User Stories

### US1 (Open/Close + Resume)

- [ ] T006 [US1] Spawn pause menu UI on open in `src/game/pause_menu/ui.rs`
- [ ] T007 [US1] Implement Resume action and UI teardown in `src/game/pause_menu/systems.rs`
- [ ] T008 [US1] Add run conditions to pause gameplay systems in `src/game/world.rs`

### US2 (Settings)

- [ ] T009 [US2] Reuse settings UI from launcher menu in `src/game/pause_menu/settings_bridge.rs`
- [ ] T010 [US2] Handle return flow from settings to pause menu in `src/game/pause_menu/systems.rs`

### US3 (Exit Actions)

- [ ] T011 [US3] Implement Exit to Main Menu action in `src/game/pause_menu/systems.rs`
- [ ] T012 [US3] Implement Exit Game action (AppExit) in `src/game/pause_menu/systems.rs`

## Final Phase: Polish & Cross-Cutting Concerns

- [ ] T013 [P] Add localization strings to `assets/locales/en-US/menu.ftl`
- [ ] T014 [P] Add localization strings to `assets/locales/ru-RU/menu.ftl`
- [ ] T015 Add audio feedback hooks for pause menu buttons in `src/game/pause_menu/systems.rs`
- [ ] T016 Add integration tests for pause menu navigation in `tests/pause_menu_navigation.rs`
