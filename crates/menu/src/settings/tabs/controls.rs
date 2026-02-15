//! Controls Settings Tab
//!
//! Provides UI controls for input-related settings (keybindings, controller config, etc.).

use crate::settings::ControlsSettingsPanel;
use bevy::prelude::*;
use launcher::config::UserSettings;
use localization::{Localization, LocalizedStrings};
use theme::Theme;

/// Spawns the Controls settings tab content.
pub fn spawn_controls_tab(
    commands: &mut Commands,
    parent: Entity,
    theme: &Theme,
    loc: &Localization,
    strings: &mut LocalizedStrings,
    _settings: &UserSettings,
) {
    let panel = commands
        .spawn((
            ControlsSettingsPanel,
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                ..default()
            },
        ))
        .id();

    commands.entity(parent).add_child(panel);

    // Populate panel
    spawn_controls_settings(commands, panel, theme, loc, strings, _settings);
}

pub fn spawn_controls_settings(
    commands: &mut Commands,
    panel: Entity,
    theme: &Theme,
    loc: &Localization,
    strings: &mut LocalizedStrings,
    _settings: &UserSettings,
) {
    let controls = [
        ("label-move-forward", "W".to_string()),
        ("label-move-backward", "S".to_string()),
        ("label-move-left", "A".to_string()),
        ("label-move-right", "D".to_string()),
        ("label-interact", "E".to_string()),
    ];

    for (label_key, key_value) in controls {
        spawn_control_row(commands, panel, theme, loc, strings, label_key, key_value);
    }
}

fn spawn_control_row(
    commands: &mut Commands,
    parent: Entity,
    theme: &Theme,
    loc: &Localization,
    strings: &mut LocalizedStrings,
    label_key: &str,
    key_value: String,
) {
    let row_id = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Px(40.0),
            padding: UiRect::horizontal(Val::Px(10.0)),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        })
        .id();

    commands.entity(parent).add_child(row_id);

    let label = strings.get(label_key, loc);
    let label_id = commands
        .spawn((
            Text::new(label),
            TextFont {
                font: theme.fonts.main.clone(),
                font_size: theme.sizes.font_body,
                ..default()
            },
            TextColor(theme.colors.text_primary),
        ))
        .id();

    commands.entity(row_id).add_child(label_id);

    let value_id = commands
        .spawn((
            Text::new(key_value),
            TextFont {
                font: theme.fonts.bold.clone(),
                font_size: theme.sizes.font_body,
                ..default()
            },
            TextColor(theme.colors.accent),
        ))
        .id();

    commands.entity(row_id).add_child(value_id);
}
