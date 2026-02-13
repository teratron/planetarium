//! # Gameplay Systems
//!
//! Core gameplay logic systems that run during the InGame state.

use bevy::prelude::*;

use crate::game::components::Rotates;

/// Rotates entities marked with the `Rotates` component.
pub fn rotate_planet(mut query: Query<&mut Transform, With<Rotates>>, time: Res<Time>) {
    let delta = time.delta_secs() * 0.3;
    for mut transform in &mut query {
        transform.rotate_y(delta);
    }
}
