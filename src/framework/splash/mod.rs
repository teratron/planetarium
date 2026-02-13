//! # Splash Screen Module
//!
//! Displays branding and splash screens during application startup.
//! Supports auto-advance via timer and user-initiated skip.

pub mod components;
pub mod resources;
pub mod systems;

use crate::framework::states::AppState;
use bevy::prelude::*;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Splash), systems::setup_splash)
            .add_systems(
                Update,
                systems::countdown_splash.run_if(in_state(AppState::Splash)),
            )
            .add_systems(OnExit(AppState::Splash), systems::cleanup_splash);
    }
}
