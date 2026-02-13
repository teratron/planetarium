//! # Game Layer
//!
//! Project-specific game logic following the ECS paradigm.
//! This module contains all gameplay-specific components, systems,
//! resources, entities, and constants.

pub mod components;
pub mod constants;
pub mod entities;
pub mod plugin;
pub mod resources;
pub mod systems;

pub use plugin::GamePlugin;
