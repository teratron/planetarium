use crate::config::UserSettings;
use crate::framework::ui::theme::constants;
use crate::framework::ui::theme::{Theme, ThemeColors};
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

/// Tracks pending settings changes to debounce expensive operations.
#[derive(Resource)]
pub struct SettingsChangeTracker {
    pub pending_display_changes: bool,
    pub last_change_time: f32,
    pub debounce_duration: f32,
}

impl Default for SettingsChangeTracker {
    fn default() -> Self {
        Self {
            pending_display_changes: false,
            last_change_time: 0.0,
            debounce_duration: constants::timing::SETTINGS_DISPLAY_DEBOUNCE, // Debounce for expensive display changes
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
    mut tracker: ResMut<SettingsChangeTracker>,
    time: Res<Time>,
) {
    // Initialize prev if None (first run)
    if prev.is_none() {
        // Apply everything immediately on startup
        if let Ok(mut window) = windows.single_mut() {
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
            window.present_mode = if settings.display.vsync {
                bevy::window::PresentMode::AutoVsync
            } else {
                bevy::window::PresentMode::AutoNoVsync
            };
        }

        runtime.master = settings.audio.master_volume;
        runtime.music = settings.audio.music_volume;
        runtime.sfx = settings.audio.sfx_volume;

        *prev = Some(settings.clone());
        return;
    }

    // Audio - Apply immediately (low cost, needs responsiveness)
    if prev.as_ref().map(|p| &p.audio) != Some(&settings.audio) {
        runtime.master = settings.audio.master_volume;
        runtime.music = settings.audio.music_volume;
        runtime.sfx = settings.audio.sfx_volume;
        info!(
            "[Settings] Applied audio settings: master={} music={} sfx={}",
            runtime.master, runtime.music, runtime.sfx
        );
        // Update cached audio settings
        if let Some(p) = prev.as_mut() {
            p.audio = settings.audio.clone();
        }
    }

    // Display - Debounce expensive changes
    if prev.as_ref().map(|p| &p.display) != Some(&settings.display) {
        tracker.pending_display_changes = true;
        tracker.last_change_time = time.elapsed_secs();
        // Do NOT update prev.display yet
    }

    // Apply pending display changes if debounce timer expired
    if tracker.pending_display_changes
        && (time.elapsed_secs() - tracker.last_change_time) > tracker.debounce_duration
    {
        if let Ok(mut window) = windows.single_mut() {
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

            window.present_mode = if settings.display.vsync {
                bevy::window::PresentMode::AutoVsync
            } else {
                bevy::window::PresentMode::AutoNoVsync
            };

            info!(
                "[Settings] Applied display settings (debounced): {}x{} fullscreen={} vsync={}",
                settings.display.width,
                settings.display.height,
                settings.display.fullscreen,
                settings.display.vsync
            );
        }

        // Update cached display settings
        if let Some(p) = prev.as_mut() {
            p.display = settings.display.clone();
        }
        tracker.pending_display_changes = false;
    }
}

/// Watches for theme changes and updates the global Theme resource.
pub fn broadcast_theme_changes(
    settings: Res<UserSettings>,
    mut prev: Local<Option<String>>,
    mut theme: ResMut<Theme>,
) {
    if !settings.is_changed() {
        return;
    }

    if prev.as_ref() != Some(&settings.theme) {
        info!("[Settings] Applying theme change: {}", settings.theme);

        match settings.theme.as_str() {
            "light" => theme.colors = ThemeColors::light(),
            _ => theme.colors = ThemeColors::default(), // dark
        }
    }

    *prev = Some(settings.theme.clone());
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
    paths: Res<crate::config::AppPaths>,
) {
    if timer.0.is_paused() {
        return;
    }

    timer.0.tick(time.delta());

    if timer.0.is_finished() {
        info!("[Settings] Auto-saving settings to disk...");
        crate::config::save_settings(&paths, &settings);
        timer.0.pause();
    }
}

/// Run condition: only tick auto-save when the timer is active.
pub fn settings_auto_save_active(timer: Res<SettingsAutoSaveTimer>) -> bool {
    !timer.0.is_paused()
}
