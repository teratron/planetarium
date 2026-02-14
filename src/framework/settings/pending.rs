//! # Pending Settings
//!
//! Tracks unsaved changes in the settings UI before they are applied to the global UserSettings.

use crate::framework::settings::UserSettings;
use bevy::prelude::*;

/// Tracks modified settings that haven't been applied yet.
#[derive(Resource, Default, Debug, Clone)]
pub struct PendingSettings {
    pub original: Option<UserSettings>,
    pub modified: Option<UserSettings>,
    pub has_changes: bool,
}

impl PendingSettings {
    /// Starts an editing session with the current settings.
    pub fn start_editing(&mut self, settings: &UserSettings) {
        self.original = Some(settings.clone());
        self.modified = Some(settings.clone());
        self.has_changes = false;
        info!("[Settings] Started editing session");
    }

    /// Updates the modified settings and checks if they differ from the original.
    pub fn update(&mut self, new_settings: UserSettings) {
        self.modified = Some(new_settings.clone());
        if let Some(original) = &self.original {
            self.has_changes = original != &new_settings;
        }
    }

    /// Applies the modified settings back to the global resource.
    pub fn apply(&mut self, target: &mut UserSettings) {
        if let Some(modified) = &self.modified {
            *target = modified.clone();
            self.original = Some(modified.clone());
            self.has_changes = false;
            info!("[Settings] Changes applied successfully");
        }
    }

    /// Resets the modified settings back to the original values.
    pub fn reset(&mut self, target: &mut UserSettings) {
        if let Some(original) = &self.original {
            *target = original.clone();
            self.modified = Some(original.clone());
            self.has_changes = false;
            info!("[Settings] Changes reset to original");
        }
    }
}

/// Marker for the Apply button.
#[derive(Component)]
pub struct ApplyChangesButton;

/// Marker for the Reset button.
#[derive(Component)]
pub struct ResetChangesButton;
