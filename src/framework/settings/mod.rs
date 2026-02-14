//! Settings Screen UI
//!
//! Implements a professional modal settings panel with categorized tabs:
//! Graphics, Audio, Controls, and General.
//! The panel is fully localized and synced with `UserSettings`.

use bevy::prelude::*;

pub mod components;
pub mod interaction;
pub mod layout;
pub mod model;
pub mod pending;
pub mod pending_systems;
pub mod systems;
pub mod tabs;
pub mod ui;
pub mod validation;

pub use components::*;
pub use interaction::*;
pub use layout::panel as panel_layout;
pub use model::*;
pub use pending::{ApplyChangesButton, PendingSettings, ResetChangesButton};
pub use systems::{
    animate_settings_fade, handle_settings_tab_clicks, spawn_settings_if_needed,
    update_settings_tab_content, update_settings_ui,
};
pub use ui::spawn_settings_menu;
pub use validation::*;

/// Resource tracking visibility.
#[derive(Resource, Default, Debug, Clone)]
pub struct SettingsOpen(pub bool);

/// Categories available in the settings menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
pub enum SettingsTab {
    #[default]
    Graphics,
    Audio,
    Controls,
    General,
}

/// Resource tracking the currently active tab in the settings menu.
#[derive(Resource, Default, Debug, Clone)]
pub struct ActiveSettingsTab(pub SettingsTab);

/// Marker for the back button in settings.
#[derive(Component)]
pub struct SettingsBackButton;
