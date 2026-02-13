//! Controls Settings Tab
//!
//! Provides UI controls for input-related settings (keybindings, controller config, etc.).

use crate::config::UserSettings;
use crate::framework::localization::{Localization, LocalizedStrings};
use crate::framework::ui::theme::Theme;
use bevy::prelude::*;

use super::super::components::ControlsSettingsPanel;

/// Spawns the Controls settings tab content.
pub fn spawn_controls_tab(
    parent: &mut bevy::ecs::hierarchy::ChildSpawnerCommands,
    theme: &Theme,
    loc: &Localization,
    strings: &mut LocalizedStrings,
    _settings: &UserSettings,
) {
    parent
        .spawn((
            ControlsSettingsPanel,
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                ..default()
            },
        ))
        .with_children(|p| {
            let controls = [
                ("control-forward", "W"),
                ("control-backward", "S"),
                ("control-left", "A"),
                ("control-right", "D"),
                ("control-jump", "SPACE"),
                ("control-sprint", "LSHIFT"),
                ("control-interact", "E"),
            ];

            for (label_key, key_value) in controls {
                spawn_control_row(p, theme, loc, strings, label_key, key_value);
            }
        });
}

fn spawn_control_row(
    parent: &mut bevy::ecs::hierarchy::ChildSpawnerCommands,
    theme: &Theme,
    loc: &Localization,
    strings: &mut LocalizedStrings,
    label_key: &str,
    key_value: &str,
) {
    parent
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Px(40.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::horizontal(Val::Px(10.0)),
            ..default()
        })
        .with_children(|row| {
            // Action Label
            row.spawn((
                Text::new(strings.get(label_key, loc)),
                TextFont {
                    font: theme.fonts.main.clone(),
                    font_size: theme.sizes.font_body,
                    ..default()
                },
                TextColor(theme.colors.text_primary),
            ));

            // Key Value
            row.spawn((
                Text::new(key_value),
                TextFont {
                    font: theme.fonts.bold.clone(),
                    font_size: theme.sizes.font_body,
                    ..default()
                },
                TextColor(theme.colors.accent),
            ));
        });
}
