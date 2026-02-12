//! Pause menu state, events, and run conditions.

use bevy::prelude::*;

/// UI mode of the in-game pause flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PauseMenuMode {
    /// Pause UI is fully hidden and gameplay is active.
    #[default]
    Closed,
    /// Pause menu with primary actions is visible.
    Menu,
    /// Settings overlay is visible from the pause flow.
    Settings,
}

/// Resource holding the current pause menu mode.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PauseMenuState {
    pub mode: PauseMenuMode,
}

/// Event describing requested pause-menu actions.
#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PauseMenuActionEvent {
    Toggle,
    OpenMenu,
    OpenSettings,
    Resume,
    CloseAll,
    ExitToMainMenu,
    ExitGame,
}

/// Run condition for gameplay systems. Gameplay runs only when pause flow is closed.
pub fn gameplay_active(pause_state: Option<Res<PauseMenuState>>) -> bool {
    pause_state.is_none_or(|state| state.mode == PauseMenuMode::Closed)
}

/// Run condition for systems that should work while pause UI is visible.
pub fn pause_overlay_active(pause_state: Res<PauseMenuState>) -> bool {
    pause_state.mode != PauseMenuMode::Closed
}

/// Run condition for systems that should work only in pause settings mode.
pub fn pause_settings_active(pause_state: Res<PauseMenuState>) -> bool {
    pause_state.mode == PauseMenuMode::Settings
}
