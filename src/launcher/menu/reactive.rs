use bevy::prelude::*;
use crate::core::config::UserSettings;

/// Runtime audio state resource (consumed by audio systems).
#[derive(Resource, Debug, Clone)]
pub struct RuntimeAudioState {
    pub master: f32,
    pub music: f32,
    pub sfx: f32,
}

impl Default for RuntimeAudioState {
    fn default() -> Self {
        Self { master: 1.0, music: 1.0, sfx: 1.0 }
    }
}

/// Watches `UserSettings` resource and applies changes immediately.
pub fn broadcast_settings_changes(
    settings: Res<UserSettings>,
    mut prev: Local<Option<UserSettings>>,
    mut windows: Query<&mut Window>,
    mut runtime: ResMut<RuntimeAudioState>,
) {
    if !settings.is_changed() {
        return;
    }

    // Display
    if prev.as_ref().map(|p| &p.display) != Some(&settings.display) {
        if let Ok(mut window) = windows.single_mut() {
            // Set resolution using provided API
            window.resolution.set(settings.display.width as f32, settings.display.height as f32);
            window.mode = if settings.display.fullscreen { bevy::window::WindowMode::Fullscreen(bevy::window::MonitorSelection::Current, bevy::window::VideoModeSelection::Current) } else { bevy::window::WindowMode::Windowed };
            info!("[Settings] Applied display settings: {}x{} fullscreen={}", settings.display.width, settings.display.height, settings.display.fullscreen);
        }
    }

    // Audio
    if prev.as_ref().map(|p| &p.audio) != Some(&settings.audio) {
        runtime.master = settings.audio.master_volume;
        runtime.music = settings.audio.music_volume;
        runtime.sfx = settings.audio.sfx_volume;
        info!("[Settings] Applied audio settings: master={} music={} sfx={}", runtime.master, runtime.music, runtime.sfx);
    }

    *prev = Some(settings.clone());
}
