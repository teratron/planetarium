use bevy::log::LogPlugin;
use bevy::prelude::*;
use planetarium::core::cli::CliArgs;
use planetarium::core::config::AppPaths;
use planetarium::core::config::metadata::APP_TITLE;
use planetarium::core::states::AppState;
use planetarium::game::GamePlugin;
use planetarium::launcher::LauncherPlugin;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

fn main() {
    // 1. Parsing CLI arguments before starting the engine.
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

    // 2. Setup Logging (Robustness L-603)
    let paths = AppPaths::from_env();
    let (non_blocking, _log_guard) =
        if let (Some(dir), Some(file)) = (paths.log_file.parent(), paths.log_file.file_name()) {
            let file_appender = tracing_appender::rolling::never(dir, file);
            let (nb, guard) = tracing_appender::non_blocking(file_appender);
            (Some(nb), Some(guard))
        } else {
            (None, None)
        };

    // Build the subscriber
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,wgpu=error,naga=error"));

    let stdout_layer = fmt::layer().with_ansi(true);

    if let Some(nb) = non_blocking {
        let file_layer = fmt::layer().with_ansi(false).with_writer(nb);
        let registry = tracing_subscriber::registry()
            .with(env_filter)
            .with(stdout_layer)
            .with(file_layer);
        if let Err(e) = tracing::subscriber::set_global_default(registry) {
            eprintln!("Failed to set tracing subscriber: {}", e);
        }
    } else {
        let registry = tracing_subscriber::registry()
            .with(env_filter)
            .with(stdout_layer);
        if let Err(e) = tracing::subscriber::set_global_default(registry) {
            eprintln!("Failed to set tracing subscriber: {}", e);
        }
    }

    info!("[Main] Initializing system with state: {:?}", initial_state);

    App::new()
        // Window setup - MUST be added before state insertion in Bevy 0.18
        // to ensure StatesPlugin (part of DefaultPlugins) is available.
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: APP_TITLE.into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    file_path: "assets".into(),
                    ..default()
                })
                .disable::<LogPlugin>(),
        )
        // Registering the high-level application state with our override.
        .insert_state(initial_state)
        // Global error state for reporting critical failures.
        .init_resource::<planetarium::core::states::ErrorState>()
        // Inserting CLI args as a Resource so they can be accessed anywhere.
        .insert_resource(args)
        // Global camera setup
        .add_systems(Startup, setup_camera)
        // Adding the aggregate Launcher and Game plugins
        .add_plugins((LauncherPlugin, GamePlugin))
        .run();
}

/// Global system to spawn the 2D camera required for UI rendering.
fn setup_camera(mut commands: Commands) {
    info!("[Main] Spawning 2D Camera...");
    commands.spawn(Camera2d);
}
