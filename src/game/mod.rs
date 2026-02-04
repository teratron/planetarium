//! # Game Module
//!
//! Entry point for the core gameplay logic and world orchestration.

use bevy::prelude::*;

pub mod world;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(world::WorldPlugin);
    }
}
