//! Shared Framework Utilities
//!
//! Common helper functions and utilities used across the framework.

use bevy::prelude::*;

pub mod single_instance;

/// Recursively despawn an entity and all its descendants.
pub fn despawn_recursive(
    commands: &mut Commands,
    entity: Entity,
    children_query: &Query<&Children>,
) {
    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            despawn_recursive(commands, child, children_query);
        }
    }
    commands.entity(entity).despawn();
}
