//! # Game World Components
//!
//! ECS components specific to the game world.

use bevy::prelude::*;

/// Marker component for the game world entities.
#[derive(Component)]
pub struct GameWorldRoot;

/// Marker for entities with idle rotation while gameplay is active.
#[derive(Component)]
pub struct Rotates;
