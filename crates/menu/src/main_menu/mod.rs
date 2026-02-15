//! # Main Menu Screen
//!
//! Main menu UI: layout, systems, and button interaction handlers.

use bevy::prelude::*;

pub mod layout;
pub mod systems;

pub use systems::{
    MainMenuRoot, MenuBackground, despawn_main_menu, handle_menu_button_clicks, spawn_main_menu,
};

/// Marker state for main menu transitions.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MainState {
    #[default]
    Active,
}

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, _app: &mut App) {
        // Main menu systems are registered in lib.rs for now
    }
}
