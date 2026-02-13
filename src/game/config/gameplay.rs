//! # Gameplay Configuration
//!
//! Defines the structure for game-specific balance and physics parameters.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Physics parameters for the planetary simulation.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct PhysicsConfig {
    /// Gravitational constant (G).
    pub gravitational_constant: f64,
    /// Time step for fixed update physics.
    pub time_step: f32,
    /// Maximum allowed velocity for any object.
    pub max_velocity: f32,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            gravitational_constant: 6.67430e-11,
            time_step: 0.016,
            max_velocity: 1000.0,
        }
    }
}

/// Parameters for planet generation and behavior.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct PlanetConfig {
    /// Default mass for a newly spawned planet (Earth mass).
    pub default_mass: f64,
    /// Minimum allowed radius for a planet.
    pub min_radius: f32,
    /// Maximum allowed radius for a planet.
    pub max_radius: f32,
    /// Default rotation speed (radians per second).
    pub rotation_speed: f32,
}

impl Default for PlanetConfig {
    fn default() -> Self {
        Self {
            default_mass: 5.972e24,
            min_radius: 100.0,
            max_radius: 10000.0,
            rotation_speed: 0.3,
        }
    }
}

/// Global resource for gameplay-related configuration.
#[derive(Resource, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[serde(default)]
pub struct GameplayConfig {
    pub physics: PhysicsConfig,
    pub planets: PlanetConfig,
}

// Implementations of Validate moved to src/config/validation.rs for centralization.
