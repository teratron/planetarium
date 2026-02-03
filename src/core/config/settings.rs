//! # User Settings
//!
//! Defines the structure of the application settings and handles loading/saving.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use crate::core::config::AppPaths;

/// Current version of the settings schema, used for migrations.
pub const SETTINGS_VERSION: u32 = 1;

/// Global resource holding all user settings.
#[derive(Resource, Serialize, Deserialize, Debug, Clone)]
pub struct UserSettings {
    /// Schema version for this config file.
    pub version: u32,
    pub display: DisplaySettings,
    pub audio: AudioSettings,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisplaySettings {
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            version: SETTINGS_VERSION,
            display: DisplaySettings {
                width: 1280,
                height: 720,
                fullscreen: false,
            },
            audio: AudioSettings {
                master_volume: 0.8,
                music_volume: 0.7,
                sfx_volume: 1.0,
            },
        }
    }
}

/// Loads settings from disk or returns defaults if not found or invalid.
pub fn load_settings(paths: &AppPaths) -> UserSettings {
    if paths.settings_file.exists() {
        match fs::read_to_string(&paths.settings_file) {
            Ok(content) => match toml::from_str::<UserSettings>(&content) {
                Ok(s) => {
                    info!("[Config] Settings loaded from {:?}", paths.settings_file);
                    s
                }
                Err(e) => {
                    warn!("[Config] Failed to parse settings.toml: {}. Using defaults.", e);
                    UserSettings::default()
                }
            },
            Err(e) => {
                warn!("[Config] Failed to read settings file: {}. Using defaults.", e);
                UserSettings::default()
            }
        }
    } else {
        info!("[Config] settings.toml not found. Creating default.");
        let default_settings = UserSettings::default();
        save_settings(paths, &default_settings);
        default_settings
    }
}

/// Saves settings to disk in TOML format.
pub fn save_settings(paths: &AppPaths, settings: &UserSettings) {
    if let Ok(toml_string) = toml::to_string_pretty(settings) {
        if let Err(e) = fs::write(&paths.settings_file, toml_string) {
            error!("[Config] Failed to save settings: {}", e);
        }
    }
}
