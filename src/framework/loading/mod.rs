//! # Loading Screen Module
//!
//! Stage 4: Loading orchestration.
//! Handles asynchronous asset loading, progress tracking, and loading screen UI.

pub mod assets;
pub mod components;
pub mod resources;
pub mod systems;

use crate::framework::states::AppState;
use bevy::prelude::*;

use resources::LoadingTracker;

/// Plugin managing the loading phase between the menu and the gameplay.
///
/// Handles progress tracking, UI updates for the progress bar,
/// and rotating lore hints to engage the user during load times.
pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadingTracker>()
            .add_systems(
                OnEnter(AppState::Loading),
                (
                    systems::reset_loading_tracker,
                    systems::setup_loading_screen,
                ),
            )
            .add_systems(
                Update,
                (
                    systems::update_loading_progress,
                    systems::update_loading_ui,
                    systems::rotate_loading_hints,
                )
                    .run_if(in_state(AppState::Loading)),
            )
            .add_systems(OnExit(AppState::Loading), systems::cleanup_loading_screen);
    }
}
