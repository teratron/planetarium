//! Shared Framework Utilities
//!
//! Common helper functions and utilities used across the framework.

use bevy::prelude::*;

/// Recursively despawn an entity and all its descendants.
///
/// This is a manual implementation of `despawn_recursive` because the `bevy_hierarchy`
/// crate's extension trait is not readily available or behaves differently in this version.
/// This function ensures that all children are also despawned to prevent memory leaks.
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
    // despawn the entity itself
    commands.entity(entity).despawn();
}
