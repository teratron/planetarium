//! Graphics Settings Tab
//!
//! Provides UI controls for graphics-related settings (quality, resolution, etc.).

use crate::config::UserSettings;
use crate::config::settings::SettingKey;
use crate::framework::localization::{Localization, LocalizedStrings};
use crate::framework::ui::theme::Theme;
use crate::framework::ui::widgets::{DropdownSpec, spawn_dropdown};
use bevy::prelude::*;

use super::super::components::GraphicsSettingsPanel;

/// Spawns the Graphics settings tab content.
pub fn spawn_graphics_tab(
    parent: &mut bevy::ecs::hierarchy::ChildSpawnerCommands,
    theme: &Theme,
    loc: &Localization,
    strings: &mut LocalizedStrings,
    settings: &UserSettings,
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
            let parent_entity = p.target_entity();
            let commands = p.commands_mut();

            // Quality Dropdown
            spawn_dropdown(
                commands,
                theme,
                DropdownSpec {
                    label: strings.get("setting-quality", loc),
                    options: vec![
                        strings.get("val-low", loc),
                        strings.get("val-medium", loc),
                        strings.get("val-high", loc),
                        strings.get("val-ultra", loc),
                    ],
                    display_values: Some(vec![
                        strings.get("val-low", loc),
                        strings.get("val-medium", loc),
                        strings.get("val-high", loc),
                        strings.get("val-ultra", loc),
                    ]),
                    selected_index: 2,
                    setting_key: SettingKey::Quality,
                },
                parent_entity,
            );

            // Resolution Dropdown
            spawn_dropdown(
                commands,
                theme,
                DropdownSpec {
                    label: strings.get("setting-resolution", loc),
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
                    setting_key: SettingKey::Resolution,
                },
                parent_entity,
            );

            // Fullscreen Dropdown
            let fullscreen_options = vec!["false".to_string(), "true".to_string()];
            let fullscreen_display = vec![strings.get("val-off", loc), strings.get("val-on", loc)];
            let fullscreen_index = if settings.display.fullscreen { 1 } else { 0 };

            spawn_dropdown(
                commands,
                theme,
                DropdownSpec {
                    label: strings.get("setting-fullscreen", loc),
                    options: fullscreen_options,
                    display_values: Some(fullscreen_display),
                    selected_index: fullscreen_index,
                    setting_key: SettingKey::Fullscreen,
                },
                parent_entity,
            );

            // VSync Dropdown
            let vsync_options = vec!["false".to_string(), "true".to_string()];
            let vsync_display = vec![strings.get("val-off", loc), strings.get("val-on", loc)];
            let vsync_index = if settings.display.vsync { 1 } else { 0 };

            spawn_dropdown(
                commands,
                theme,
                DropdownSpec {
                    label: strings.get("setting-vsync", loc),
                    options: vsync_options,
                    display_values: Some(vsync_display),
                    selected_index: vsync_index,
                    setting_key: SettingKey::Vsync,
                },
                parent_entity,
            );
        });
}
