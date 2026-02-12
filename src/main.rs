use bevy::log::LogPlugin;
use bevy::prelude::*;
use planetarium::config::AppPaths;
use planetarium::config::cli::CliArgs;
use planetarium::config::metadata::APP_TITLE;
use planetarium::framework::FrameworkPlugin;
use planetarium::framework::states::AppState;
use planetarium::game::GamePlugin;
use planetarium::utils::single_instance::{
    SingleInstanceError, SingleInstanceLock, acquire_single_instance_lock,
};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

/// Thin `Resource` wrapper so the app can keep the worker guard alive for the
/// duration of the application's lifetime and prevent it from being dropped.
#[derive(Resource)]
struct LogWorkerGuard {
    // Use a named field with leading underscore to indicate intentional unused
    // usage while keeping the value stored for the app lifetime.
    _guard: WorkerGuard,
}

/// Keep the single-instance lock alive during the application's lifetime.
#[derive(Resource)]
struct InstanceLockGuard {
    _guard: SingleInstanceLock,
}

/// Default logging configuration level.
const DEFAULT_LOG_FILTER: &str = "info,wgpu=error,naga=error";
/// Debug logging configuration level used when `--debug` flag is passed.
const DEBUG_LOG_FILTER: &str = "debug,wgpu=error,naga=error";

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

    // 5. Setup logging system (respect --debug flag and optional --log-filter)
    let log_guard = setup_logging(&initial_state, &paths, args.debug, args.log_filter.clone());

    // 6. Configure and run Bevy app, keeping runtime guards alive in resources.
    build_app(args, initial_state, paths, log_guard, instance_lock).run();
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
fn setup_logging(
    initial_state: &AppState,
    paths: &AppPaths,
    debug: bool,
    cli_filter: Option<String>,
) -> Option<WorkerGuard> {
    // Resolve the effective filter in order of precedence:
    // 1. `RUST_LOG` environment variable
    // 2. CLI `--log-filter` if provided
    // 3. Default filter (depends on --debug)
    let default_filter = if debug {
        DEBUG_LOG_FILTER
    } else {
        DEFAULT_LOG_FILTER
    };

    let env_filter = match EnvFilter::try_from_default_env() {
        Ok(filter) => filter,
        Err(_) => {
            if let Some(ref f) = cli_filter {
                EnvFilter::new(f.as_str())
            } else {
                EnvFilter::new(default_filter)
            }
        }
    };

    info!("[Main] Effective log filter: {}", env_filter.to_string());

    let stdout_layer = fmt::layer().with_ansi(true);

    // Setup logging with optional file output. Keep the WorkerGuard alive by
    // returning it so the caller can keep it in App resources for the
    // application lifetime.
    if let (Some(dir), Some(file)) = (paths.log_file.parent(), paths.log_file.file_name()) {
        let file_appender = tracing_appender::rolling::never(dir, file);
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        let file_layer = fmt::layer().with_ansi(false).with_writer(non_blocking);

        let registry = tracing_subscriber::registry()
            .with(env_filter)
            .with(stdout_layer)
            .with(file_layer);

        if let Err(e) = tracing::subscriber::set_global_default(registry) {
            eprintln!("[Main] Failed to set tracing subscriber: {}", e);
            return None;
        }

        info!(
            "[Main] Logging initialized with file output: {:?}",
            paths.log_file
        );
        if debug {
            info!("[Main] Debug logging enabled via CLI flag (--debug)");
        }

        info!("[Main] Initializing system with state: {:?}", initial_state);
        Some(guard)
    } else {
        let registry = tracing_subscriber::registry()
            .with(env_filter)
            .with(stdout_layer);

        if let Err(e) = tracing::subscriber::set_global_default(registry) {
            eprintln!("[Main] Failed to set tracing subscriber: {}", e);
            return None;
        }

        info!("[Main] Logging initialized (stdout only)");
        if debug {
            info!("[Main] Debug logging enabled via CLI flag (--debug)");
        }

        info!("[Main] Initializing system with state: {:?}", initial_state);
        None
    }
}

/// Build the Bevy application with all plugins and systems.
fn build_app(
    args: CliArgs,
    initial_state: AppState,
    paths: AppPaths,
    log_guard: Option<WorkerGuard>,
    instance_lock: Option<SingleInstanceLock>,
) -> App {
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
    .init_resource::<planetarium::framework::states::ErrorState>()
    .insert_resource(args)
    .insert_resource(paths)
    .add_plugins((FrameworkPlugin, GamePlugin));

    // Keep the tracing worker guard alive for the application lifetime by
    // inserting it as a Bevy resource. This prevents log flushing worker
    // from being dropped prematurely.
    if let Some(guard) = log_guard {
        app.insert_resource(LogWorkerGuard { _guard: guard });
    }
    if let Some(guard) = instance_lock {
        app.insert_resource(InstanceLockGuard { _guard: guard });
    }

    app
}
