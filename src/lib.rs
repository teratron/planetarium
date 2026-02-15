//! # Planetarium Core Library
//!
//! Architecture:
//! - `plugin`: Core gameplay logic entry point.
//! - `components`: ECS components.
//! - `config`: Game configuration.
//! - `constants`: Game constants.
//! - `entities`: Entity spawners.
//! - `resources`: Global resources.
//! - `systems`: Game systems.

pub mod components;
pub mod config;
pub mod constants;
pub mod entities;
pub mod plugin;
pub mod resources;
pub mod systems;

pub use plugin::GamePlugin;
