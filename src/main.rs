//! # Planetarium Application
//!
//! The main entry point for the Planetarium application.

use bevy::prelude::*;
use planetarium::core::cli::CliArgs;
use planetarium::core::config::metadata::APP_TITLE;
use planetarium::core::states::AppState;
use planetarium::launcher::LauncherPlugin;

fn main() {
    // 1. Parsing CLI arguments before starting the engine.
    // This allows us to use these flags to configure the app itself.
    let args = CliArgs::parse_args();

    // Determine the initial state.
    let initial_state = if let Some(state_str) = &args.state {
        match state_str.to_lowercase().as_str() {
            "splash" => AppState::Splash,
            "mainmenu" | "menu" => AppState::MainMenu,
            "loading" => AppState::Loading,
            "ingame" | "game" => AppState::InGame,
            _ => AppState::Booting,
        }
    } else {
        AppState::Booting
    };

    info!("[Main] Initializing system with state: {:?}", initial_state);
    if args.skip_splash {
        info!("[Main] CLI: Splash screens will be skipped.");
    }

    App::new()
        // Registering the high-level application state with our override.
        .insert_state(initial_state)
        // 2. Inserting CLI args as a Resource so they can be accessed anywhere.
        .insert_resource(args)
        // Window setup
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: APP_TITLE.into(),
                ..default()
            }),
            ..default()
        }))
        // Adding the aggregate Launcher plugin
        .add_plugins(LauncherPlugin)
        .run();
}
