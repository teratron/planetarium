//! # Game Plugin
//!
//! The top-level plugin that bundles all game-specific logic.

use bevy::prelude::*;

use super::systems;
use crate::framework::states::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), systems::setup::setup_game_world)
            .add_systems(
                Update,
                systems::gameplay::rotate_planet
                    .run_if(in_state(AppState::InGame))
                    .run_if(crate::framework::menu::pause::state::gameplay_active),
            )
            .add_systems(
                OnExit(AppState::InGame),
                systems::cleanup::cleanup_game_world,
            );
    }
}
