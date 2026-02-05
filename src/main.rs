use bevy::log::LogPlugin;
use bevy::prelude::*;
use planetarium::core::cli::CliArgs;
use planetarium::core::config::AppPaths;
use planetarium::core::config::metadata::APP_TITLE;
use planetarium::core::states::AppState;
use planetarium::game::GamePlugin;
use planetarium::launcher::LauncherPlugin;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

/// Default logging configuration level.
const DEFAULT_LOG_FILTER: &str = "info,wgpu=error,naga=error";

fn main() {
    // 1. Parse CLI arguments
    let args = CliArgs::parse_args();
    let initial_state = parse_initial_state(&args);

    // 2. Setup logging system
    let paths = AppPaths::from_env();
    setup_logging(&initial_state, &paths);

    // 3. Configure and run Bevy app
    build_app(args, initial_state, paths).run();
}

/// Parse CLI arguments to determine initial `AppState`.
fn parse_initial_state(args: &CliArgs) -> AppState {
    args.state
        .as_ref()
        .and_then(|state_str| match state_str.to_lowercase().as_str() {
            "splash" => Some(AppState::Splash),
            "mainmenu" | "menu" => Some(AppState::MainMenu),
            "loading" => Some(AppState::Loading),
            "ingame" | "game" => Some(AppState::InGame),
            _ => None,
        })
        .unwrap_or(AppState::Booting)
}

/// Initialize logging with file and stdout output.
fn setup_logging(initial_state: &AppState, paths: &AppPaths) {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(DEFAULT_LOG_FILTER));

    let stdout_layer = fmt::layer().with_ansi(true);

    // Setup logging with optional file output
    if let (Some(dir), Some(file)) = (paths.log_file.parent(), paths.log_file.file_name()) {
        let file_appender = tracing_appender::rolling::never(dir, file);
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
        let file_layer = fmt::layer().with_ansi(false).with_writer(non_blocking);

        let registry = tracing_subscriber::registry()
            .with(env_filter)
            .with(stdout_layer)
            .with(file_layer);

        if let Err(e) = tracing::subscriber::set_global_default(registry) {
            eprintln!("[Main] Failed to set tracing subscriber: {}", e);
        }
    } else {
        let registry = tracing_subscriber::registry()
            .with(env_filter)
            .with(stdout_layer);

        if let Err(e) = tracing::subscriber::set_global_default(registry) {
            eprintln!("[Main] Failed to set tracing subscriber: {}", e);
        }
    }

    info!("[Main] Initializing system with state: {:?}", initial_state);
}

/// Build the Bevy application with all plugins and systems.
fn build_app(args: CliArgs, initial_state: AppState, paths: AppPaths) -> App {
    let mut app = App::new();
    let assets_path = paths.assets_dir.clone();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: APP_TITLE.into(),
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                file_path: assets_path.to_string_lossy().to_string(),
                ..default()
            })
            .disable::<LogPlugin>(),
    )
    .insert_state(initial_state)
    .init_resource::<planetarium::core::states::ErrorState>()
    .insert_resource(args)
    .insert_resource(paths)
    .add_systems(Startup, setup_camera)
    .add_plugins((LauncherPlugin, GamePlugin));

    app
}

/// Global system to spawn the 2D camera required for UI rendering.
fn setup_camera(mut commands: Commands) {
    info!("[Main] Spawning 2D Camera...");
    commands.spawn(Camera2d);
}
