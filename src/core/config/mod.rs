//! # Configuration Module
//!
//! This module handles the application's configuration settings, including
//! user-defined settings and platform-specific path resolution.

pub mod paths;
pub mod settings;

pub use paths::AppPaths;
pub use settings::UserSettings;

use bevy::prelude::*;

/// System to initialize paths and load settings.
pub fn setup_config(mut commands: Commands) {
    let paths = AppPaths::from_env();

    if let Err(e) = paths.ensure_dirs() {
        error!("[Config] Failed to create data directory: {}", e);
    }

    let settings = settings::load_settings(&paths);

    commands.insert_resource(paths);
    commands.insert_resource(settings);
}
