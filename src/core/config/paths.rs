//! # Application Paths
//!
//! Handles resolution of platform-specific directories for data and configuration.

use crate::core::config::metadata::APP_NAME;
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
    /// Full path to the settings.toml file.
    pub settings_file: PathBuf,
    /// Path to the session log file.
    pub log_file: PathBuf,
}

impl AppPaths {
    /// Resolves paths based on the operating system and environment.
    /// This is designed as a reusable template for different Bevy games.
    pub fn from_env() -> Self {
        // 1. Resolve Data Directory (OS-specific)
        let data_dir = if let Some(proj_dirs) = dirs::data_dir() {
            proj_dirs.join(APP_NAME)
        } else {
            PathBuf::from(".").join("data")
        };

        // 2. Resolve Assets Directory
        // First check CWD, then check relative to executable (helpful for direct runs from target/debug)
        let mut assets_dir = PathBuf::from("assets");
        if !assets_dir.exists()
            && let Ok(exe_path) = std::env::current_exe()
            && let Some(exe_dir) = exe_path.parent()
        {
            // Try next to exe
            let exe_assets = exe_dir.join("assets");
            if exe_assets.exists() {
                assets_dir = exe_assets;
            } else if let Some(parent) = exe_dir.parent() {
                // Try parent of exe (e.g. from target/debug/ up to project root)
                let parent_assets = parent.join("assets");
                if parent_assets.exists() {
                    assets_dir = parent_assets;
                } else if let Some(grandparent) = parent.parent() {
                    // Try grandparent (target/debug/ -> target/ -> root)
                    let gp_assets = grandparent.join("assets");
                    if gp_assets.exists() {
                        assets_dir = gp_assets;
                    }
                }
            }
        }

        let settings_file = data_dir.join("settings.toml");
        let log_file = data_dir.join("session.log");

        Self {
            data_dir,
            assets_dir,
            settings_file,
            log_file,
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
