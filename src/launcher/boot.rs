//! Stage 1: Booting logic.
//! Handles initialization, config loading, and environment checks.

use bevy::prelude::*;

pub struct BootPlugin;

impl Plugin for BootPlugin {
    fn build(&self, _app: &mut App) {
        info!("[BootPlugin] Initializing...");
    }
}
