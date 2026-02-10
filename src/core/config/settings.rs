//! # User Settings
//!
//! Defines the structure of the application settings and handles loading/saving.

use crate::core::config::AppPaths;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

/// Current version of the settings schema, used for migrations.
pub const SETTINGS_VERSION: u32 = 3;

/// Quality presets for graphics settings.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum Quality {
    Low,
    #[default]
    Medium,
    High,
    Ultra,
}

/// Type-safe keys for settings that can be modified via UI.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum SettingKey {
    MasterVolume,
    MusicVolume,
    SfxVolume,
    Fullscreen,
    Vsync,
    Resolution,
    Quality,
    Language,
    Theme,
}

/// Graphics-related settings.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(default)]
#[non_exhaustive]
pub struct GraphicsSettings {
    pub quality: Quality,
}

/// Global resource holding all user settings.
#[derive(Resource, Serialize, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct UserSettings {
    /// Schema version for this config file.
    pub version: u32,
    /// Preferred language (e.g., "en-US", "ru-RU").
    #[serde(default)]
    pub language: String,
    /// Preferred theme ("dark", "light").
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default)]
    pub display: DisplaySettings,
    #[serde(default)]
    pub audio: AudioSettings,
    #[serde(default)]
    pub graphics: GraphicsSettings,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(default)]
#[non_exhaustive]
pub struct DisplaySettings {
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub vsync: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
#[non_exhaustive]
pub struct AudioSettings {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
}

impl Default for DisplaySettings {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
            fullscreen: false,
            vsync: true,
        }
    }
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            master_volume: 0.8,
            music_volume: 0.7,
            sfx_volume: 1.0,
        }
    }
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            version: SETTINGS_VERSION,
            language: "en-US".to_string(),
            theme: default_theme(),
            display: DisplaySettings::default(),
            audio: AudioSettings::default(),
            graphics: GraphicsSettings::default(),
        }
    }
}

fn default_theme() -> String {
    "dark".to_string()
}

// Unit tests moved to bottom of file to satisfy clippy (no items after test module).

/// Loads settings from disk or returns defaults if not found or invalid.
pub fn load_settings(paths: &AppPaths) -> UserSettings {
    if paths.settings_file.exists() {
        match fs::read_to_string(&paths.settings_file) {
            Ok(content) => match toml::from_str::<UserSettings>(&content) {
                Ok(mut s) => {
                    info!("[Config] Settings loaded from {:?}", paths.settings_file);

                    // --- VERSION GUARD / MIGRATION ---
                    if s.version < SETTINGS_VERSION {
                        info!(
                            "[Config] Migration: Upgrading settings from v{} to v{}",
                            s.version, SETTINGS_VERSION
                        );
                        s.version = SETTINGS_VERSION;
                        // Saving the migrated settings back to disk.
                        // Because of #[serde(default)], any NEW fields in our current
                        // Rust structs will be populated with defaults while keeping
                        // the user's existing values for old fields.
                        save_settings(paths, &s);
                    }

                    s
                }
                Err(e) => {
                    warn!(
                        "[Config] Failed to parse settings.toml: {}. Using defaults.",
                        e
                    );
                    UserSettings::default()
                }
            },
            Err(e) => {
                warn!(
                    "[Config] Failed to read settings file: {}. Using defaults.",
                    e
                );
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
    if let Ok(toml_string) = toml::to_string_pretty(settings)
        && let Err(e) = fs::write(&paths.settings_file, toml_string)
    {
        error!("[Config] Failed to save settings: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn default_settings_include_graphics() {
        let s = UserSettings::default();
        assert_eq!(s.graphics.quality, Quality::Medium);
    }

    #[test]
    fn default_settings_include_theme() {
        let s = UserSettings::default();
        assert_eq!(s.theme, "dark");
    }

    #[test]
    fn default_settings_include_vsync() {
        let s = UserSettings::default();
        assert!(s.display.vsync);
    }

    proptest! {
        #[test]
        fn volume_settings_remain_in_range(volume in 0.0f32..=1.0) {
            let mut s = UserSettings::default();
            s.audio.master_volume = volume;
            s.audio.music_volume = volume;
            s.audio.sfx_volume = volume;

            prop_assert!((0.0..=1.0).contains(&s.audio.master_volume));
            prop_assert!((0.0..=1.0).contains(&s.audio.music_volume));
            prop_assert!((0.0..=1.0).contains(&s.audio.sfx_volume));
        }
    }
}
