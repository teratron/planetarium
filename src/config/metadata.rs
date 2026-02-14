//! # Application Metadata
//!
//! Centralized information about the application, such as its name,
//! title, and current version.

use bevy::prelude::*;

/// Internal name of the application.
pub const APP_NAME: &str = "Planetarium";
/// Public-facing title of the application.
pub const APP_TITLE: &str = "Planetarium";

/// Default logging configuration level.
pub const DEFAULT_LOG_FILTER: &str = "info,wgpu=error,naga=error";
/// Debug logging configuration level used when `--debug` flag is passed.
pub const DEBUG_LOG_FILTER: &str = "debug,wgpu=error,naga=error";

/// Name of the user settings file.
pub const SETTINGS_FILENAME: &str = "settings.ron";
/// Name of the application session log file.
pub const LOG_FILENAME: &str = "session.log";
/// Name of the single-instance lock file.
pub const LOCK_FILENAME: &str = "instance.lock";
/// Name of the default assets directory.
pub const ASSETS_DIRNAME: &str = "assets";
/// Name of the asset manifest file.
pub const ASSET_MANIFEST_FILENAME: &str = "assets.ron";

/// Centralized application metadata resource.
#[derive(Resource, Debug, Clone)]
#[non_exhaustive]
pub struct AppMetadata {
    /// The internal name of the application (used for file system paths).
    pub name: &'static str,
    /// The public-facing title of the game (used for window title).
    pub title: &'static str,
    /// The current version of the application.
    pub version: &'static str,
    /// A short description of the application.
    pub description: &'static str,
    /// URL to the application's source code repository.
    pub repository: &'static str,
    /// List of application authors.
    pub authors: &'static str,
}

impl Default for AppMetadata {
    fn default() -> Self {
        Self {
            name: APP_NAME,
            title: APP_TITLE,
            version: env!("CARGO_PKG_VERSION"),
            description: env!("CARGO_PKG_DESCRIPTION"),
            repository: env!("CARGO_PKG_REPOSITORY"),
            authors: env!("CARGO_PKG_AUTHORS"),
        }
    }
}
