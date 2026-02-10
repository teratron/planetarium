# Implementation Plan: In-Game Pause Menu (ESC)

## Purpose

Add an in-game pause menu that opens with ESC and offers: Resume, Settings, Exit to Main Menu, Exit Game.

## Scope

- Input handling for ESC while in `AppState::InGame`.
- Pause menu UI overlay with four actions.
- State management for paused gameplay.
- Settings access from pause menu (reuse existing settings UI).
- Localization for new menu strings.
- Tests for pause menu navigation.

## Out of Scope

- Confirm dialog for exiting the game (separate TODO).
- Main menu redesign.
- Gameplay mechanics beyond pausing.

## User Stories

- **US1**: As a player, I can press ESC during gameplay to open a pause menu and resume the game.
- **US2**: As a player, I can open Settings from the pause menu and return to gameplay.
- **US3**: As a player, I can exit to the main menu or exit the game from the pause menu.

## Architecture & ECS Alignment

- Introduce a dedicated pause menu plugin under `src/game/pause_menu/`.
- Use a lightweight UI state/resource for pause overlay (e.g., `PauseMenuState`), running only in `AppState::InGame`.
- Pause game systems via run conditions keyed off the pause state (avoid hard pausing the entire app unless needed).
- Reuse `Theme` and localization resources from existing UI infrastructure.
- Integrate settings UI as an overlay and route back to pause menu on close.

## UI & Input

- Overlay UI spawned/despawned with `OnEnter/OnExit` for a pause UI state.
- ESC toggles pause menu (open/close); when settings is open, ESC returns to pause menu.
- Buttons:
  - Resume: closes pause menu and resumes gameplay.
  - Settings: opens settings overlay.
  - Exit to Main Menu: transitions to `AppState::MainMenu`.
  - Exit Game: triggers app exit event.

## Localization

- Add pause menu strings to `assets/locales/*/menu.ftl`.

## Acceptance Criteria

- Pressing ESC in `AppState::InGame` opens pause menu.
- Resume closes menu and gameplay continues.
- Settings opens and returns to pause menu on close.
- Exit to Main Menu transitions cleanly with no leftover UI.
- Exit Game terminates the app.
- All new strings are localized.
- Tests cover basic navigation and state transitions.
