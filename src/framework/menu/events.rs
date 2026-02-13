//! # Menu Events
//!
//! Event types for UI interactions and audio feedback.

use bevy::prelude::*;

/// Categories of UI audio feedback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiAudioEvent {
    Click,
    Hover,
    Back,
    Error,
    Scroll,
    Open,
    Close,
    Select,
}

impl UiAudioEvent {
    /// Returns the asset manifest key for this audio event.
    pub fn manifest_key(&self) -> &'static str {
        match self {
            Self::Click => "click",
            Self::Hover => "hover",
            Self::Back => "back",
            Self::Error => "error",
            Self::Scroll => "scroll",
            Self::Open => "open",
            Self::Close => "close",
            Self::Select => "select",
        }
    }
}

/// Helper to play a UI sound effect.
///
/// Looks up the audio handle via the asset manifest and spawns a one-shot player.
pub fn play_ui_audio(
    event: UiAudioEvent,
    volume: f32,
    commands: &mut Commands,
    cache: &mut crate::framework::loading::assets::AssetCache,
    asset_server: &AssetServer,
    manifest: &crate::framework::loading::assets::AssetManifest,
) {
    let key = event.manifest_key();
    if let Some(handle) = cache.get_or_load_audio(key, asset_server, manifest) {
        commands.spawn((
            AudioPlayer::new(handle),
            PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: bevy::audio::Volume::Linear(volume),
                ..default()
            },
        ));
    }
}
