//! General Settings Tab
//!
//! Provides UI controls for general settings (language, accessibility, etc.).

use super::super::UserSettings;
use crate::framework::localization::{Localization, LocalizedStrings};
use crate::framework::settings::SettingKey;
use crate::framework::ui::theme::Theme;
use crate::framework::ui::widgets::{DropdownSpec, spawn_dropdown};
use bevy::prelude::*;

use super::super::components::GeneralSettingsPanel;

/// Spawns the General settings tab content.
pub fn spawn_general_tab(
    parent: &mut bevy::ecs::hierarchy::ChildSpawnerCommands,
    theme: &Theme,
    loc: &Localization,
    strings: &mut LocalizedStrings,
    settings: &UserSettings,
) {
    parent
        .spawn((
            GeneralSettingsPanel,
            Node {
                width: Val::Percent(100.0),
                ..default()
            },
        ))
        .with_children(|p| {
            let parent_entity = p.target_entity();
            let commands = p.commands_mut();

            // Language dropdown
            let lang_options = vec!["en-US".to_string(), "ru-RU".to_string()];
            let lang_display = vec![strings.get("lang-en", loc), strings.get("lang-ru", loc)];
            let selected_index = lang_options
                .iter()
                .position(|s| s == &settings.language)
                .unwrap_or(0);

            spawn_dropdown(
                commands,
                theme,
                DropdownSpec {
                    label: strings.get("setting-language", loc),
                    options: lang_options,
                    display_values: Some(lang_display),
                    selected_index,
                    setting_key: SettingKey::Language,
                },
                parent_entity,
            );

            // Theme dropdown
            let theme_options = vec!["dark".to_string(), "light".to_string()];
            let theme_display = vec![
                strings.get("theme-dark", loc),
                strings.get("theme-light", loc),
            ];
            let theme_index = theme_options
                .iter()
                .position(|t| t == &settings.theme)
                .unwrap_or(0);

            spawn_dropdown(
                commands,
                theme,
                DropdownSpec {
                    label: strings.get("setting-theme", loc),
                    options: theme_options,
                    display_values: Some(theme_display),
                    selected_index: theme_index,
                    setting_key: SettingKey::Theme,
                },
                parent_entity,
            );

            // Multiple instances toggle
            let multi_instance_options = vec!["false".to_string(), "true".to_string()];
            let multi_instance_display =
                vec![strings.get("val-off", loc), strings.get("val-on", loc)];
            let multi_instance_index = if settings.allow_multiple_instances {
                1
            } else {
                0
            };

            spawn_dropdown(
                commands,
                theme,
                DropdownSpec {
                    label: strings.get("setting-allow-multiple-instances", loc),
                    options: multi_instance_options,
                    display_values: Some(multi_instance_display),
                    selected_index: multi_instance_index,
                    setting_key: SettingKey::AllowMultipleInstances,
                },
                parent_entity,
            );
        });
}
