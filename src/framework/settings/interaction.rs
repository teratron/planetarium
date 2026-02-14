//! # Settings Interaction and Reactive Systems
//!
//! This module handles the flow of data between the UI and the persistent `UserSettings`.
//! It manages interaction keys, reactive broadcasts (audio, display, theme), and auto-saving.
//!
//! ## Data Flow
//! 1. **UI Interaction**: User interacts with a widget (slider, dropdown).
//! 2. **Setting Update**: The widget system updates the `UserSettings` resource.
//! 3. **Reactive Broadcast**: `broadcast_settings_changes` detects the change and applies it to the engine (e.g., Bevy Window, Audio State).
//! 4. **Auto-Save**: `schedule_settings_save` triggers a debounced save to disk.

use crate::config::{UserSettings, save_settings};
use crate::framework::ui::theme::{Theme, ThemeColors, constants};
use bevy::prelude::*;

/// Type-safe keys for settings that can be modified via UI.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum SettingKey {
    MasterVolume,
    MusicVolume,
    SfxVolume,
    Fullscreen,
    Vsync,
    AllowMultipleInstances,
    Resolution,
    Quality,
    Language,
    Theme,
}

/// Fired when settings fail to save to disk.
#[derive(Message, Debug, Clone)]
pub struct SettingsSaveError {
    /// Human-readable error description.
    pub error: String,
}

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

/// Lightweight snapshot of settings for efficient change tracking.
#[derive(Default, PartialEq, Clone)]
pub struct SettingsSnapshot {
    pub display: crate::config::settings::DisplaySettings,
    pub audio: crate::config::settings::AudioSettings,
}

/// Tracks pending settings changes to debounce expensive operations (like resolution changes).
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
            debounce_duration: constants::timing::SETTINGS_DISPLAY_DEBOUNCE,
        }
    }
}

/// Watches the `UserSettings` resource and applies changes immediately to the engine.
pub fn broadcast_settings_changes(
    settings: Res<UserSettings>,
    mut prev: Local<Option<SettingsSnapshot>>,
    mut windows: Query<&mut Window>,
    mut runtime: ResMut<RuntimeAudioState>,
    mut tracker: ResMut<SettingsChangeTracker>,
    time: Res<Time>,
) {
    if !settings.is_changed() && prev.is_some() && !tracker.pending_display_changes {
        return;
    }

    // Initialize prev if None (first run)
    if prev.is_none() {
        if let Ok(mut window) = windows.single_mut() {
            apply_display_settings(&mut window, &settings.display);
        }

        runtime.master = settings.audio.master_volume;
        runtime.music = settings.audio.music_volume;
        runtime.sfx = settings.audio.sfx_volume;

        *prev = Some(SettingsSnapshot {
            display: settings.display.clone(),
            audio: settings.audio.clone(),
        });
        return;
    }

    // Audio - Apply immediately (low cost, needs responsiveness)
    if prev.as_ref().map(|p| &p.audio) != Some(&settings.audio) {
        runtime.master = settings.audio.master_volume;
        runtime.music = settings.audio.music_volume;
        runtime.sfx = settings.audio.sfx_volume;
        if let Some(p) = prev.as_mut() {
            p.audio = settings.audio.clone();
        }
    }

    // Display - Debounce expensive changes
    if prev.as_ref().map(|p| &p.display) != Some(&settings.display) {
        tracker.pending_display_changes = true;
        tracker.last_change_time = time.elapsed_secs();
    }

    // Apply pending display changes if debounce timer expired
    if tracker.pending_display_changes
        && (time.elapsed_secs() - tracker.last_change_time) > tracker.debounce_duration
    {
        if let Ok(mut window) = windows.single_mut() {
            apply_display_settings(&mut window, &settings.display);
            info!("[Settings] Applied display settings (debounced)");
        }

        if let Some(p) = prev.as_mut() {
            p.display = settings.display.clone();
        }
        tracker.pending_display_changes = false;
    }
}

fn apply_display_settings(window: &mut Window, display: &crate::config::settings::DisplaySettings) {
    window
        .resolution
        .set(display.width as f32, display.height as f32);
    window.mode = if display.fullscreen {
        bevy::window::WindowMode::Fullscreen(
            bevy::window::MonitorSelection::Current,
            bevy::window::VideoModeSelection::Current,
        )
    } else {
        bevy::window::WindowMode::Windowed
    };
    window.present_mode = if display.vsync {
        bevy::window::PresentMode::AutoVsync
    } else {
        bevy::window::PresentMode::AutoNoVsync
    };
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
        timer.pause();
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
    mut error_events: MessageWriter<SettingsSaveError>,
) {
    if timer.0.is_paused() {
        return;
    }

    timer.0.tick(time.delta());

    if timer.0.is_finished() {
        info!("[Settings] Auto-saving settings to disk...");
        if let Err(e) = save_settings(&paths, &settings) {
            error!("[Settings] Failed to auto-save settings: {}", e);
            error_events.write(SettingsSaveError {
                error: e.to_string(),
            });
        }
        timer.0.pause();
    }
}

pub fn settings_auto_save_active(timer: Res<SettingsAutoSaveTimer>) -> bool {
    !timer.0.is_paused()
}
