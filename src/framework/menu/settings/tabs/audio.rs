//! Audio Settings Tab
//!
//! Provides UI controls for audio-related settings (volume levels, etc.).

use crate::core::config::UserSettings;
use crate::core::config::settings::SettingKey;
use crate::core::localization::{Localization, LocalizedStrings};
use crate::ui::theme::Theme;
use bevy::prelude::*;

use super::super::components::AudioSettingsPanel;

/// Spawns the Audio settings tab content.
pub fn spawn_audio_tab(
    parent: &mut bevy::ecs::hierarchy::ChildSpawnerCommands,
    theme: &Theme,
    loc: &Localization,
    strings: &mut LocalizedStrings,
    settings: &UserSettings,
) {
    parent
        .spawn((
            AudioSettingsPanel,
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(15.0),
                ..default()
            },
        ))
        .with_children(|p| {
            spawn_volume_slider(
                p,
                theme,
                loc,
                strings,
                "setting-master-volume",
                settings.audio.master_volume,
                SettingKey::MasterVolume,
            );
            spawn_volume_slider(
                p,
                theme,
                loc,
                strings,
                "setting-music-volume",
                settings.audio.music_volume,
                SettingKey::MusicVolume,
            );
            spawn_volume_slider(
                p,
                theme,
                loc,
                strings,
                "setting-sfx-volume",
                settings.audio.sfx_volume,
                SettingKey::SfxVolume,
            );
        });
}

/// Helper to spawn a volume slider control.
fn spawn_volume_slider(
    parent: &mut bevy::ecs::hierarchy::ChildSpawnerCommands,
    theme: &Theme,
    loc: &Localization,
    strings: &mut LocalizedStrings,
    key: &str,
    value: f32,
    setting_key: SettingKey,
) {
    let parent_entity = parent.target_entity();
    let commands = parent.commands_mut();
    super::super::super::widgets::spawn_slider(
        commands,
        theme,
        &strings.get(key, loc),
        super::super::super::widgets::SliderSpec {
            min: 0.0,
            max: 1.0,
            value,
        },
        setting_key,
        parent_entity,
    );
}
