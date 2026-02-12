//! Input handlers for in-game pause interactions.

use super::state::{PauseMenuActionEvent, PauseMenuMode, PauseMenuState};
use bevy::prelude::*;

/// Handles ESC while in-game and emits pause actions.
pub fn handle_escape_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    pause_state: Res<PauseMenuState>,
    mut events: MessageWriter<PauseMenuActionEvent>,
) {
    if !keyboard.just_pressed(KeyCode::Escape) {
        return;
    }

    match pause_state.mode {
        PauseMenuMode::Closed => events.write(PauseMenuActionEvent::OpenMenu),
        PauseMenuMode::Menu => events.write(PauseMenuActionEvent::CloseAll),
        PauseMenuMode::Settings => events.write(PauseMenuActionEvent::OpenMenu),
    };
}
