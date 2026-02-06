//! Graphics Settings Tab
//!
//! Provides UI controls for graphics-related settings (quality, resolution, etc.).

use crate::core::config::UserSettings;
use crate::core::localization::Localization;
use crate::ui::theme::Theme;
use bevy::prelude::*;

use super::super::components::GraphicsSettingsPanel;

/// Spawns the Graphics settings tab content.
pub fn spawn_graphics_tab(
    parent: &mut bevy::ecs::hierarchy::ChildSpawnerCommands,
    theme: &Theme,
    loc: &Localization,
    _settings: &UserSettings,
) {
    parent
        .spawn((
            GraphicsSettingsPanel,
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(15.0),
                ..default()
            },
        ))
        .with_children(|p| {
            // Quality Dropdown
            let parent_entity = p.target_entity();
            let commands = p.commands_mut();

            super::super::super::widgets::spawn_dropdown(
                commands,
                theme,
                super::super::super::widgets::DropdownSpec {
                    label: loc.t("setting-quality"),
                    options: vec![
                        loc.t("val-low"),
                        loc.t("val-medium"),
                        loc.t("val-high"),
                        loc.t("val-ultra"),
                    ],
                    display_values: Some(vec![
                        loc.t("val-low"),
                        loc.t("val-medium"),
                        loc.t("val-high"),
                        loc.t("val-ultra"),
                    ]),
                    selected_index: 2,
                    setting_key: "quality".to_string(),
                },
                parent_entity,
            );

            // Resolution Dropdown
            super::super::super::widgets::spawn_dropdown(
                commands,
                theme,
                super::super::super::widgets::DropdownSpec {
                    label: loc.t("setting-resolution"),
                    options: vec![
                        "1280x720".to_string(),
                        "1920x1080".to_string(),
                        "2560x1440".to_string(),
                    ],
                    display_values: Some(vec![
                        "1280x720".to_string(),
                        "1920x1080".to_string(),
                        "2560x1440".to_string(),
                    ]),
                    selected_index: 1,
                    setting_key: "resolution".to_string(),
                },
                parent_entity,
            );
        });
}
