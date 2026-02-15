//! Audio Settings Tab
//!
//! Provides UI controls for audio-related settings (volume levels, etc.).

use crate::settings::{AudioSettingsPanel, SettingKey};
use crate::widgets::{SliderSpec, spawn_slider};
use bevy::prelude::*;
use launcher::config::UserSettings;
use localization::{Localization, LocalizedStrings};
use theme::Theme;

/// Spawns the Audio settings tab content.
pub fn spawn_audio_tab(
    commands: &mut Commands,
    parent: Entity,
    theme: &Theme,
    loc: &Localization,
    strings: &mut LocalizedStrings,
    settings: &UserSettings,
) {
    let panel = commands
        .spawn((
            AudioSettingsPanel,
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(15.0),
                ..default()
            },
        ))
        .id();

    commands.entity(parent).add_child(panel);

    spawn_volume_slider(
        commands,
        panel,
        theme,
        loc,
        strings,
        "setting-master-volume",
        settings.audio.master_volume,
        SettingKey::MasterVolume,
    );
    spawn_volume_slider(
        commands,
        panel,
        theme,
        loc,
        strings,
        "setting-music-volume",
        settings.audio.music_volume,
        SettingKey::MusicVolume,
    );
    spawn_volume_slider(
        commands,
        panel,
        theme,
        loc,
        strings,
        "setting-sfx-volume",
        settings.audio.sfx_volume,
        SettingKey::SfxVolume,
    );
}

/// Helper to spawn a volume slider control.
#[allow(clippy::too_many_arguments)]
fn spawn_volume_slider(
    commands: &mut Commands,
    parent: Entity,
    theme: &Theme,
    loc: &Localization,
    strings: &mut LocalizedStrings,
    key: &str,
    value: f32,
    setting_key: SettingKey,
) {
    spawn_slider(
        commands,
        theme,
        &strings.get(key, loc),
        SliderSpec {
            min: 0.0,
            max: 1.0,
            value,
        },
        setting_key,
        parent,
    );
}
