//! # Configuration Module
//!
//! Top-level configuration: user settings, path resolution, CLI args, and app metadata.

pub mod build_mode;
pub mod cli;
pub mod metadata;
pub mod settings;

pub use crate::boot::AppPaths;
use bevy::prelude::*;
pub use metadata::AppMetadata;
pub use settings::UserSettings;

/// System to initialize paths and load settings.
pub fn setup_config(
    mut commands: Commands,
    mut next_state: ResMut<NextState<crate::states::AppState>>,
    mut error_state: ResMut<crate::states::ErrorState>,
) {
    let metadata = AppMetadata::default();
    info!(
        "[Config] Initializing {} v{}",
        metadata.title, metadata.version
    );

    let paths = AppPaths::from_env();

    if let Err(e) = paths.ensure_dirs() {
        let err_msg = format!("Failed to create data directory: {}", e);
        error!("[Config] {}", err_msg);
        error_state.message = err_msg;
        next_state.set(crate::states::AppState::Error);
    }

    let settings = settings::load_settings(&paths);

    commands.insert_resource(metadata);
    commands.insert_resource(paths);
    commands.insert_resource(settings);
}
