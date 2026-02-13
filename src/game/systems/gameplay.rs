//! # Gameplay Systems
//!
//! Core gameplay logic systems that run during the InGame state.

use bevy::prelude::*;

use crate::game::components::Rotates;

use crate::game::config::GameplayConfig;

/// Rotates entities marked with the `Rotates` component.
pub fn rotate_planet(
    mut query: Query<&mut Transform, With<Rotates>>,
    time: Res<Time>,
    config: Res<GameplayConfig>,
) {
    let delta = time.delta_secs() * config.planets.rotation_speed;
    for mut transform in &mut query {
        transform.rotate_y(delta);
    }
}
