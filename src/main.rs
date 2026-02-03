//! # Planetarium Application
//!
//! The main entry point for the Planetarium application.
//! This file is responsible for building the Bevy `App` and running it.

use bevy::prelude::*;

fn main() {
    // Initializing the Bevy App
    App::new()
        // DefaultPlugins provides the core engine functionality:
        // - Window management
        // - Rendering (WGPU)
        // - Input handling
        // - Asset server
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Planetarium".into(),
                ..default()
            }),
            ..default()
        }))
        // Our launcher and game logic will be added here as Plugins in the next tasks
        .run();
}
