//! # Application Paths
//!
//! Handles resolution of platform-specific directories for data and configuration.

use crate::boot::platform_paths::get_data_dir;
use crate::config::metadata::{
    APP_NAME, ASSET_MANIFEST_FILENAME, ASSETS_DIRNAME, LOCK_FILENAME, LOG_FILENAME,
    SETTINGS_FILENAME,
};
use bevy::prelude::*;
use std::fs;
use std::path::PathBuf;

/// Resource that stores resolved platform-specific paths.
#[derive(Resource, Debug, Clone)]
#[non_exhaustive]
pub struct AppPaths {
    /// Directory for configuration and persistent data.
    pub data_dir: PathBuf,
    /// Directory for game assets.
    pub assets_dir: PathBuf,
    /// Full path to the settings.ron file.
    pub settings_file: PathBuf,
    /// Path to the session log file.
    pub log_file: PathBuf,
    /// Path to the file used for single-instance locking.
    pub instance_lock_file: PathBuf,
}

impl AppPaths {
    /// Resolves paths based on the operating system and environment.
    /// This is designed as a reusable template for different Bevy games.
    pub fn from_env() -> Self {
        // 1. Resolve Data Directory (OS-specific custom implementation)
        let data_dir = get_data_dir(APP_NAME);

        // 2. Resolve Assets Directory
        // First check CWD, then check relative to executable (helpful for direct runs from target/debug)
        let mut assets_dir = PathBuf::from(ASSETS_DIRNAME);
        if !is_valid_assets_dir(&assets_dir)
            && let Ok(exe_path) = std::env::current_exe()
            && let Some(exe_dir) = exe_path.parent()
        {
            // Try next to exe
            let exe_assets = exe_dir.join("assets");
            if is_valid_assets_dir(&exe_assets) {
                assets_dir = exe_assets;
            } else if let Some(parent) = exe_dir.parent() {
                // Try parent of exe (e.g. from target/debug/ up to project root)
                let parent_assets = parent.join("assets");
                if is_valid_assets_dir(&parent_assets) {
                    assets_dir = parent_assets;
                } else if let Some(grandparent) = parent.parent() {
                    // Try grandparent (target/debug/ -> target/ -> root)
                    let gp_assets = grandparent.join("assets");
                    if is_valid_assets_dir(&gp_assets) {
                        assets_dir = gp_assets;
                    }
                }
            }
        }
        if assets_dir.exists() {
            assets_dir = assets_dir
                .canonicalize()
                .unwrap_or_else(|_| assets_dir.clone());
        }

        let settings_file = data_dir.join(SETTINGS_FILENAME);
        let log_file = data_dir.join(LOG_FILENAME);
        let instance_lock_file = data_dir.join(LOCK_FILENAME);

        Self {
            data_dir,
            assets_dir,
            settings_file,
            log_file,
            instance_lock_file,
        }
    }

    /// Ensures that the data directory exists on disk.
    pub fn ensure_dirs(&self) -> std::io::Result<()> {
        if !self.data_dir.exists() {
            info!("[Config] Creating data directory at {:?}", self.data_dir);
            fs::create_dir_all(&self.data_dir)?;
        }
        Ok(())
    }
}

fn is_valid_assets_dir(path: &std::path::Path) -> bool {
    path.exists() && path.join(ASSET_MANIFEST_FILENAME).exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_env_uses_expected_lock_file_name() {
        let paths = AppPaths::from_env();
        assert_eq!(
            paths.instance_lock_file.file_name(),
            Some(std::ffi::OsStr::new(LOCK_FILENAME))
        );
    }
}
