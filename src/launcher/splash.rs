//! Stage 2: Splash screen sequence.
//! Handles displaying brand logos and license requirements.

use bevy::prelude::*;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, _app: &mut App) {
        info!("[SplashPlugin] Initializing...");
    }
}
