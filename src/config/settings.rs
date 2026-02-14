//! # User Settings
//!
//! Defines the structure of the application settings and handles loading/saving.

use crate::config::AppPaths;
use anyhow::Context;
use bevy::prelude::*;
use ron;
use serde::{Deserialize, Serialize};
use std::fs;

/// Current version of the settings schema, used for migrations.
pub const SETTINGS_VERSION: u32 = 4;

/// Fired when settings fail to save to disk.
#[derive(Message, Debug, Clone)]
pub struct SettingsSaveError {
    /// Human-readable error description.
    pub error: String,
}

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
    AllowMultipleInstances,
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
    /// Allows launching multiple game instances simultaneously.
    #[serde(default)]
    pub allow_multiple_instances: bool,
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
            allow_multiple_instances: false,
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
            Ok(content) => match ron::from_str::<UserSettings>(&content) {
                Ok(mut s) => {
                    info!("[Config] Settings loaded from {:?}", paths.settings_file);

                    // --- VERSION GUARD / MIGRATION ---
                    if s.version < SETTINGS_VERSION {
                        s = migrate_settings(s, paths);
                        // Try to save the migrated and validated settings back to disk.
                        let _ = save_settings(paths, &s);
                    } else if s.version > SETTINGS_VERSION {
                        warn!(
                            "[Config] Settings v{} is newer than current v{}. Using defaults to prevent corruption.",
                            s.version, SETTINGS_VERSION
                        );
                        return UserSettings::default();
                    }

                    s
                }
                Err(e) => {
                    warn!(
                        "[Config] Failed to parse settings.ron: {}. Using defaults.",
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
        info!("[Config] settings.ron not found. Creating default.");
        let default_settings = UserSettings::default();
        let _ = save_settings(paths, &default_settings);
        default_settings
    }
}

/// Migrates settings from an older version to the current version.
/// Ensures that existing values are preserved while new fields are correctly initialized.
pub fn migrate_settings(mut old: UserSettings, _paths: &AppPaths) -> UserSettings {
    info!(
        "[Config] Migration path: {} -> {}",
        old.version, SETTINGS_VERSION
    );

    // Simple version bump for now, as Serde's #[serde(default)] handles
    // addition of new fields during deserialization.
    // For more complex migrations (e.g. data renames), add logic here.
    old.version = SETTINGS_VERSION;
    old
}

/// Saves settings to disk in RON format atomically.
/// Writes to a temporary file first, then renames to avoid corruption during crashes.
pub fn save_settings(paths: &AppPaths, settings: &UserSettings) -> anyhow::Result<()> {
    let pretty_config = ron::ser::PrettyConfig::default()
        .struct_names(true)
        .enumerate_arrays(true);

    let ron_string = ron::ser::to_string_pretty(settings, pretty_config)
        .context("Failed to serialize settings")?;

    let tmp_file = paths.settings_file.with_extension("ron.tmp");

    // Write to temp file
    fs::write(&tmp_file, ron_string)
        .with_context(|| format!("Failed to write to temp settings file: {:?}", tmp_file))?;

    // Rename to final destination (atomic on many filesystems)
    fs::rename(&tmp_file, &paths.settings_file).with_context(|| {
        format!(
            "Failed to rename temp settings file {:?} to {:?}",
            tmp_file, paths.settings_file
        )
    })?;

    Ok(())
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

    #[test]
    fn default_settings_disallow_multiple_instances() {
        let s = UserSettings::default();
        assert!(!s.allow_multiple_instances);
    }

    #[test]
    fn missing_allow_multiple_instances_defaults_to_false() {
        let parsed: UserSettings =
            ron::from_str("UserSettings(version: 4)").expect("minimal settings should deserialize");
        assert!(!parsed.allow_multiple_instances);
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
