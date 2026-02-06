//! Controls Settings Tab
//!
//! Provides UI controls for input-related settings (keybindings, controller config, etc.).

use crate::core::config::UserSettings;
use crate::core::localization::Localization;
use crate::ui::theme::Theme;
use bevy::prelude::*;

use super::super::components::ControlsSettingsPanel;

/// Spawns the Controls settings tab content.
pub fn spawn_controls_tab(
    parent: &mut bevy::ecs::hierarchy::ChildSpawnerCommands,
    theme: &Theme,
    _loc: &Localization,
    _settings: &UserSettings,
) {
    parent
        .spawn((ControlsSettingsPanel, Node { ..default() }))
        .with_children(|p| {
            p.spawn((
                Text::new("Controls tab - coming soon"),
                TextFont {
                    font: theme.fonts.main.clone(),
                    font_size: theme.sizes.font_body,
                    ..default()
                },
                TextColor(theme.colors.text_secondary),
            ));
        });
}
