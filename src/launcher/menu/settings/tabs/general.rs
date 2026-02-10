//! General Settings Tab
//!
//! Provides UI controls for general settings (language, accessibility, etc.).

use crate::core::config::UserSettings;
use crate::core::config::settings::SettingKey;
use crate::core::localization::{Localization, LocalizedStrings};
use crate::ui::theme::Theme;
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

            // Language dropdown: internal options are locale IDs, display values are localized names
            let lang_options = vec!["en-US".to_string(), "ru-RU".to_string()];
            let lang_display = vec![strings.get("lang-en", loc), strings.get("lang-ru", loc)];
            // Determine selected index by matching settings language
            let selected_index = lang_options
                .iter()
                .position(|s| s == &settings.language)
                .unwrap_or(0);

            super::super::super::widgets::spawn_dropdown(
                commands,
                theme,
                super::super::super::widgets::DropdownSpec {
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

            super::super::super::widgets::spawn_dropdown(
                commands,
                theme,
                super::super::super::widgets::DropdownSpec {
                    label: strings.get("setting-theme", loc),
                    options: theme_options,
                    display_values: Some(theme_display),
                    selected_index: theme_index,
                    setting_key: SettingKey::Theme,
                },
                parent_entity,
            );
        });
}
