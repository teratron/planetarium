//! # Game Module
//!
//! Entry point for the core gameplay logic and world orchestration.

use bevy::prelude::*;

/// Re-export shim for backward compatibility.
/// **DEPRECATED**: Use `crate::framework::menu::pause` directly.
pub mod pause_menu {
    pub use crate::framework::menu::pause::*;
}

pub mod world;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(world::WorldPlugin);
    }
}
