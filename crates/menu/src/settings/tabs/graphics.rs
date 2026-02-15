//! Graphics Settings Tab
//!
//! Provides UI controls for graphics-related settings (quality, resolution, etc.).

use crate::settings::{GraphicsSettingsPanel, SettingKey};
use crate::widgets::{DropdownSpec, spawn_dropdown};
use bevy::prelude::*;
use launcher::config::UserSettings;
use launcher::config::settings::Quality;
use localization::{Localization, LocalizedStrings};
use theme::Theme;

/// Spawns the Graphics settings tab content.
pub fn spawn_graphics_tab(
    commands: &mut Commands,
    parent: Entity,
    theme: &Theme,
    loc: &Localization,
    strings: &mut LocalizedStrings,
    settings: &UserSettings,
) {
    let panel = commands
        .spawn((
            GraphicsSettingsPanel,
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(15.0),
                ..default()
            },
        ))
        .id();

    commands.entity(parent).add_child(panel);

    // Quality Dropdown
    spawn_dropdown(
        commands,
        theme,
        DropdownSpec {
            label: strings.get("setting-quality", loc),
            options: vec![
                "Low".to_string(), // Keep consistent with enum casing? original code used localized strings here as values?
                // Original: strings.get("val-low", loc) for values.
                // Wait, DropdownSpec `options` are the internal values. `display_values` are for display.
                // In interaction.rs, how are these processed?
                // SettingKey::Quality -> UserSettings just stores Quality enum.
                // If I store "val-low" localized string, deserialization might fail if it expects "Low".
                // I should check `interaction.rs` logic for applying settings.
                // Actually `spawn_dropdown` stores `Dropdown` component with selected index.
                // Interaction logic likely reads index -> option[index].
                // So options should probably be stable strings like "Low", "Medium".
                // Original used localized strings for both options and display?
                /*
                    options: vec![
                        strings.get("val-low", loc),
                        ...
                    ]
                */
                // If the interaction logic parses this string, it depends on localization? That's bad.
                // But I should follow original behavior for now, or improve it.
                // Launcher config has `Quality` enum with `#[serde(rename_all = "lowercase")]`.
                // So expected values are "low", "medium", "high", "ultra".
                "low".to_string(),
                "medium".to_string(),
                "high".to_string(),
                "ultra".to_string(),
            ],
            display_values: Some(vec![
                strings.get("val-low", loc),
                strings.get("val-medium", loc),
                strings.get("val-high", loc),
                strings.get("val-ultra", loc),
            ]),
            selected_index: match settings.graphics.quality {
                Quality::Low => 0,
                Quality::Medium => 1,
                Quality::High => 2,
                Quality::Ultra => 3,
            },
            setting_key: SettingKey::Quality,
        },
        panel,
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
            selected_index: 1, // Need logic to match current settings.display.width/height
            // For now hardcoding 1 as in original, but todo: improved matching
            setting_key: SettingKey::Resolution,
        },
        panel,
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
        panel,
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
        panel,
    );
}
