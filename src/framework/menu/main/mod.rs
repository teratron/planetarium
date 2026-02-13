//! # Main Menu Screen
//!
//! Main menu UI: layout, systems, and button interaction handlers.

pub mod layout;
pub mod systems;

pub use systems::{
    MainMenuRoot, MenuBackground, despawn_main_menu, handle_menu_button_clicks, spawn_main_menu,
};
