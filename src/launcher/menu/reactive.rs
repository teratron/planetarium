use crate::core::config::UserSettings;
use bevy::prelude::*;

/// Runtime audio state resource (consumed by audio systems).
///
/// This is a lightweight version of AudioSettings designed for fast access
/// by the engine's audio playback systems.
#[derive(Resource, Debug, Clone)]
pub struct RuntimeAudioState {
    pub master: f32,
    pub music: f32,
    pub sfx: f32,
}

impl Default for RuntimeAudioState {
    fn default() -> Self {
        Self {
            master: 1.0,
            music: 1.0,
            sfx: 1.0,
        }
    }
}

/// Watches the `UserSettings` resource and applies changes immediately to the engine.
///
/// Specifically handles window resolution/mode changes and synchronizes
/// `RuntimeAudioState` with the user's volume preferences.
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
    if prev.as_ref().map(|p| &p.display) != Some(&settings.display)
        && let Ok(mut window) = windows.single_mut()
    {
        // Set resolution using provided API
        window.resolution.set(
            settings.display.width as f32,
            settings.display.height as f32,
        );
        window.mode = if settings.display.fullscreen {
            bevy::window::WindowMode::Fullscreen(
                bevy::window::MonitorSelection::Current,
                bevy::window::VideoModeSelection::Current,
            )
        } else {
            bevy::window::WindowMode::Windowed
        };
        info!(
            "[Settings] Applied display settings: {}x{} fullscreen={}",
            settings.display.width, settings.display.height, settings.display.fullscreen
        );
    }

    // Audio
    if prev.as_ref().map(|p| &p.audio) != Some(&settings.audio) {
        runtime.master = settings.audio.master_volume;
        runtime.music = settings.audio.music_volume;
        runtime.sfx = settings.audio.sfx_volume;
        info!(
            "[Settings] Applied audio settings: master={} music={} sfx={}",
            runtime.master, runtime.music, runtime.sfx
        );
    }

    *prev = Some(settings.clone());
}

/// Timer to debounce settings saving to disk.
#[derive(Resource)]
pub struct SettingsAutoSaveTimer(pub Timer);

impl Default for SettingsAutoSaveTimer {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(2.0, TimerMode::Once);
        timer.pause(); // Start paused
        Self(timer)
    }
}

/// Triggers save to disk when settings change, with a delay.
pub fn schedule_settings_save(
    settings: Res<UserSettings>,
    mut timer: ResMut<SettingsAutoSaveTimer>,
) {
    if settings.is_changed() {
        timer.0.reset();
        timer.0.unpause();
    }
}

/// Writes settings to disk when the timer expires.
pub fn auto_save_settings(
    time: Res<Time>,
    mut timer: ResMut<SettingsAutoSaveTimer>,
    settings: Res<UserSettings>,
    paths: Res<crate::core::config::AppPaths>,
) {
    if timer.0.is_paused() {
        return;
    }

    timer.0.tick(time.delta());

    if timer.0.is_finished() {
        info!("[Settings] Auto-saving settings to disk...");
        crate::core::config::save_settings(&paths, &settings);
        timer.0.pause();
    }
}
