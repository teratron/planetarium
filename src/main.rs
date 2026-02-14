use bevy::prelude::*;
use planetarium::config::AppPaths;
use planetarium::config::cli::CliArgs;
use planetarium::config::metadata::{APP_TITLE, DEBUG_LOG_FILTER, DEFAULT_LOG_FILTER};
use planetarium::framework::FrameworkPlugin;
use planetarium::framework::states::AppState;
use planetarium::game::GamePlugin;
use planetarium::utils::single_instance::{
    SingleInstanceError, SingleInstanceLock, acquire_single_instance_lock,
};

/// Resource that keeps the single-instance lock alive during the application's lifetime.
///
/// This guard holds an OS-level file lock. It must be kept alive until the application exits
/// to prevent other instances from starting. It is implemented as a `NonSend` resource
/// to ensure it stays on the main thread, which is safer for handling OS file handles.
struct InstanceLockGuard {
    _guard: SingleInstanceLock,
}

fn main() {
    // 1. Parse CLI arguments
    let args = CliArgs::parse_args();
    let initial_state = parse_initial_state(&args);

    // 2. Resolve paths and ensure required directories exist.
    let paths = AppPaths::from_env();
    if let Err(e) = paths.ensure_dirs() {
        eprintln!(
            "[Main] Failed to prepare data directory {:?}: {}",
            paths.data_dir, e
        );
        return;
    }

    // 3. Load settings early to read startup behavior flags.
    let settings = planetarium::config::settings::load_settings(&paths);

    // 4. Protect against launching a second instance unless explicitly allowed.
    let instance_lock = match acquire_single_instance_lock(
        &paths,
        settings.allow_multiple_instances,
    ) {
        Ok(lock) => lock,
        Err(SingleInstanceError::AlreadyRunning { .. }) => {
            eprintln!(
                "[Main] Another game instance is already running. Set `allow_multiple_instances = true` in {:?} to override.",
                paths.settings_file
            );
            return;
        }
        Err(e) => {
            eprintln!("[Main] {}", e);
            return;
        }
    };

    // 5. Build and run Bevy app.
    build_app(args, initial_state, paths, instance_lock).run();
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

/// Build the Bevy application with all plugins and systems.
fn build_app(
    args: CliArgs,
    initial_state: AppState,
    paths: AppPaths,
    instance_lock: Option<SingleInstanceLock>,
) -> App {
    let mut app = App::new();
    let assets_path = paths.assets_dir.clone();

    // Determine log level and filter
    let log_level = if args.debug {
        bevy::log::Level::DEBUG
    } else {
        bevy::log::Level::INFO
    };

    let log_filter = args.log_filter.clone().unwrap_or_else(|| {
        if args.debug {
            DEBUG_LOG_FILTER.to_string()
        } else {
            DEFAULT_LOG_FILTER.to_string()
        }
    });

    app.add_plugins(
        DefaultPlugins
            .set(bevy::log::LogPlugin {
                level: log_level,
                filter: log_filter,
                ..default()
            })
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
            }),
    )
    .insert_state(initial_state)
    .init_resource::<planetarium::framework::states::ErrorState>()
    .insert_resource(args)
    .insert_resource(paths)
    .add_plugins((FrameworkPlugin, GamePlugin));

    if let Some(guard) = instance_lock {
        app.insert_non_send_resource(InstanceLockGuard { _guard: guard });
    }

    app
}
