//! # Audio Framework
//!
//! Provides global audio settings and management.

pub mod resources;
pub mod systems;

use bevy::prelude::*;

/// Central audio plugin.
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::AudioSettings>();
    }
}
