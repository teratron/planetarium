//! # Planetarium Application
//!
//! The main entry point for the Planetarium application.

use bevy::prelude::*;
use planetarium::core::states::AppState;
use planetarium::launcher::LauncherPlugin;

fn main() {
    App::new()
        // Registering the high-level application state
        .init_state::<AppState>()
        // Window setup
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Planetarium".into(),
                ..default()
            }),
            ..default()
        }))
        // Adding the aggregate Launcher plugin
        .add_plugins(LauncherPlugin)
        .run();
}
