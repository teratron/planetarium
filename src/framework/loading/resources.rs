//! # Loading Screen Resources
//!
//! Resources used by the loading screen system for tracking progress.

use bevy::prelude::*;

/// Resource to track the current loading progress and display hints.
#[derive(Resource)]
pub struct LoadingTracker {
    /// 0.0 to 1.0 progress.
    pub progress: f32,
    /// Index of the currently displayed hint.
    pub current_hint_index: usize,
    /// Timer to rotate hints.
    pub hint_timer: Timer,
    /// Flag to prevent repeated logging when loading completes.
    pub completed_logged: bool,
}

impl Default for LoadingTracker {
    fn default() -> Self {
        Self {
            progress: 0.0,
            current_hint_index: 0,
            hint_timer: Timer::from_seconds(4.0, TimerMode::Repeating),
            completed_logged: false,
        }
    }
}

/// Resource that tracks which assets need to be loaded.
///
/// Populated during setup; `update_loading_progress` polls these handles
/// via `AssetServer::get_load_state` to compute real progress.
#[derive(Resource, Default)]
pub struct AssetLoadingState {
    /// Handles to all assets that must be loaded before transitioning.
    pub required_assets: Vec<UntypedHandle>,
    /// Number of assets that have finished loading.
    pub loaded_count: usize,
    /// Total number of assets tracked (cached from `required_assets.len()`).
    pub total_count: usize,
}
