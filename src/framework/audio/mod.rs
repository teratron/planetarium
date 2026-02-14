//! # Audio Framework
//!
//! Provides global audio settings and management.

pub mod systems;

use crate::framework::settings::AudioSettings;
use bevy::prelude::*;

/// Central audio plugin.
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioSettings>();
    }
}
