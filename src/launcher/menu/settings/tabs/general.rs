//! General Settings Tab
//!
//! Provides UI controls for general settings (language, accessibility, etc.).

use crate::core::config::UserSettings;
use crate::core::localization::Localization;
use crate::ui::theme::Theme;
use bevy::prelude::*;

use super::super::components::GeneralSettingsPanel;

/// Spawns the General settings tab content.
pub fn spawn_general_tab(
    parent: &mut bevy::ecs::hierarchy::ChildSpawnerCommands,
    theme: &Theme,
    loc: &Localization,
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
            let lang_display = vec![loc.t("lang-en"), loc.t("lang-ru")];
            // Determine selected index by matching settings language
            let selected_index = lang_options
                .iter()
                .position(|s| s == &settings.language)
                .unwrap_or(0);

            super::super::super::widgets::spawn_dropdown(
                commands,
                theme,
                super::super::super::widgets::DropdownSpec {
                    label: loc.t("setting-language"),
                    options: lang_options,
                    display_values: Some(lang_display),
                    selected_index,
                    setting_key: "language".to_string(),
                },
                parent_entity,
            );
        });
}
