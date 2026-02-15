//! Settings Screen UI
//!
//! Implements a professional modal settings panel with categorized tabs:
//! Graphics, Audio, Controls, and General.
//! The panel is fully localized and synced with `UserSettings`.

use bevy::prelude::*;

pub mod components;
pub mod interaction;
pub mod layout;
pub mod pending;
pub mod pending_systems;
pub mod systems;
pub mod tabs;
pub mod ui;
pub mod validation;

pub use components::*;
pub use interaction::*;
pub use layout::panel as panel_layout;
pub use pending::{ApplyChangesButton, PendingSettings, ResetChangesButton};
pub use systems::{
    animate_settings_fade, handle_settings_tab_clicks, spawn_settings_if_needed,
    update_settings_tab_content, update_settings_ui,
};
pub use ui::spawn_settings_menu;
pub use validation::*;

// Re-export UserSettings and related types from launcher configuration
pub use launcher::config::AppPaths;
pub use launcher::config::settings::save_settings;
pub use launcher::config::settings::{
    AudioSettings, DisplaySettings, GraphicsSettings, Quality, UserSettings,
};

/// Resource tracking visibility.
#[derive(Resource, Default, Debug, Clone)]
pub struct SettingsOpen(pub bool);

/// Categories available in the settings menu.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Reflect)]
pub enum SettingsTab {
    #[default]
    Graphics,
    Audio,
    Controls,
    General,
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, _app: &mut App) {
        // Systems are currently registered in lib.rs
    }
}

/// Resource tracking the currently active tab in the settings menu.
#[derive(Resource, Default, Debug, Clone)]
pub struct ActiveSettingsTab(pub SettingsTab);

/// Marker for the back button in settings.
#[derive(Component)]
pub struct SettingsBackButton;
