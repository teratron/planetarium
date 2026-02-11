//! Components for the in-game pause menu UI.

use bevy::prelude::*;

/// Root marker for the pause menu overlay.
#[derive(Component)]
pub struct PauseMenuRoot;

/// Marker for interactive pause menu buttons.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PauseMenuButton {
    pub action: PauseMenuButtonAction,
}

/// Supported actions from the pause menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PauseMenuButtonAction {
    Resume,
    OpenSettings,
    ExitToMainMenu,
    ExitGame,
}
