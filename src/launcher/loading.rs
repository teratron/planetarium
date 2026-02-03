//! Stage 4: Loading orchestration.
//! Handles asynchronous asset loading and progress tracking.

use bevy::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, _app: &mut App) {
        info!("[LoadingPlugin] Initializing...");
    }
}
