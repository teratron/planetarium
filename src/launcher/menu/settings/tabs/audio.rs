//! Audio Settings Tab
//!
//! Provides UI controls for audio-related settings (volume levels, etc.).

use crate::core::config::UserSettings;
use crate::core::localization::Localization;
use crate::ui::theme::Theme;
use bevy::prelude::*;

use super::super::components::AudioSettingsPanel;

/// Spawns the Audio settings tab content.
pub fn spawn_audio_tab(
    parent: &mut bevy::ecs::hierarchy::ChildSpawnerCommands,
    theme: &Theme,
    loc: &Localization,
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
                "setting-master-volume",
                settings.audio.master_volume,
                "master_volume",
            );
            spawn_volume_slider(
                p,
                theme,
                loc,
                "setting-music-volume",
                settings.audio.music_volume,
                "music_volume",
            );
            spawn_volume_slider(
                p,
                theme,
                loc,
                "setting-sfx-volume",
                settings.audio.sfx_volume,
                "sfx_volume",
            );
        });
}

/// Helper to spawn a volume slider control.
fn spawn_volume_slider(
    parent: &mut bevy::ecs::hierarchy::ChildSpawnerCommands,
    theme: &Theme,
    loc: &Localization,
    key: &str,
    value: f32,
    setting_key: &str,
) {
    let parent_entity = parent.target_entity();
    let commands = parent.commands_mut();
    super::super::super::widgets::spawn_slider(
        commands,
        theme,
        &loc.t(key),
        super::super::super::widgets::SliderSpec {
            min: 0.0,
            max: 1.0,
            value,
        },
        setting_key,
        parent_entity,
    );
}
