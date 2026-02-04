//! # Application Paths
//!
//! Handles resolution of platform-specific directories for data and configuration.

use crate::core::config::metadata::APP_NAME;
use bevy::prelude::*;
use std::fs;
use std::path::PathBuf;

/// Resource that stores resolved platform-specific paths.
#[derive(Resource, Debug, Clone)]
pub struct AppPaths {
    /// Directory for configuration and persistent data.
    pub data_dir: PathBuf,
    /// Full path to the settings.toml file.
    pub settings_file: PathBuf,
    /// Path to the session log file.
    pub log_file: PathBuf,
}

impl AppPaths {
    /// Resolves paths based on the operating system.
    pub fn from_env() -> Self {
        let data_dir = if let Some(proj_dirs) = dirs::data_dir() {
            proj_dirs.join(APP_NAME)
        } else {
            // Fallback to local directory if we can't find home
            PathBuf::from(".").join("data")
        };

        let settings_file = data_dir.join("settings.toml");
        let log_file = data_dir.join("session.log");

        Self {
            data_dir,
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
