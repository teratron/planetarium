//! # Splash Screen Resources
//!
//! Resources used by the splash screen system.

use bevy::prelude::*;

/// Resource to track the splash screen timer.
#[derive(Resource)]
pub struct SplashTimer(pub Timer);
