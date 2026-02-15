//! # Settings UI Interaction Systems (Pending Changes)
//!
//! Systems to handle Apply and Reset buttons in the settings UI.

use super::{ApplyChangesButton, PendingSettings, ResetChangesButton, UserSettings};
use bevy::prelude::*;

/// System to handle Apply and Reset button clicks in the settings UI.
pub fn handle_settings_action_buttons(
    apply_query: Query<&Interaction, (Changed<Interaction>, With<ApplyChangesButton>)>,
    reset_query: Query<&Interaction, (Changed<Interaction>, With<ResetChangesButton>)>,
    mut pending: ResMut<PendingSettings>,
    mut settings: ResMut<UserSettings>,
) {
    // 1. Handle Apply
    for interaction in &apply_query {
        if *interaction == Interaction::Pressed && pending.has_changes {
            pending.apply(&mut settings);
        }
    }

    // 2. Handle Reset
    for interaction in &reset_query {
        if *interaction == Interaction::Pressed && pending.has_changes {
            pending.reset(&mut settings);
        }
    }
}

/// System to start a pending edit session when the settings menu is opened.
pub fn initialize_pending_settings(
    mut pending: ResMut<PendingSettings>,
    settings: Res<UserSettings>,
    settings_open: Res<super::SettingsOpen>,
) {
    if settings_open.is_changed() && settings_open.0 && pending.original.is_none() {
        pending.start_editing(&settings);
    } else if settings_open.is_changed() && !settings_open.0 {
        // Clear pending when closed
        pending.original = None;
        pending.modified = None;
        pending.has_changes = false;
    }
}
