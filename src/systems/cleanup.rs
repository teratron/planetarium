//! # Cleanup Systems
//!
//! Systems for cleaning up game entities on state exit.

use bevy::prelude::*;

use crate::components::GameWorldRoot;

/// Despawns all entities under the game world root.
pub fn cleanup_game_world(mut commands: Commands, query: Query<Entity, With<GameWorldRoot>>) {
    info!("[Game] Cleaning up world...");
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
