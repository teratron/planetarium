use bevy::log::LogPlugin;
use bevy::prelude::*;
use planetarium::core::cli::CliArgs;
use planetarium::core::config::AppPaths;
use planetarium::core::config::metadata::APP_TITLE;
use planetarium::core::states::AppState;
use planetarium::game::GamePlugin;
use planetarium::launcher::LauncherPlugin;
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

/// Default logging configuration level.
const DEFAULT_LOG_FILTER: &str = "info,wgpu=error,naga=error";
/// Debug logging configuration level used when `--debug` flag is passed.
const DEBUG_LOG_FILTER: &str = "debug,wgpu=error,naga=error";

fn main() {
    // 1. Parse CLI arguments
    let args = CliArgs::parse_args();
    let initial_state = parse_initial_state(&args);

    // 2. Setup logging system (respect --debug flag and optional --log-filter)
    let paths = AppPaths::from_env();
    let log_guard = setup_logging(&initial_state, &paths, args.debug, args.log_filter.clone());

    // 3. Configure and run Bevy app (keep the log guard alive by inserting it into App resources)
    build_app(args, initial_state, paths, log_guard).run();
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
    .init_resource::<planetarium::core::states::ErrorState>()
    .insert_resource(args)
    .insert_resource(paths)
    .add_systems(Startup, setup_camera)
    .add_systems(Startup, diagnose_cameras.after(setup_camera))
    .add_plugins((LauncherPlugin, GamePlugin));

    // Keep the tracing worker guard alive for the application lifetime by
    // inserting it as a Bevy resource. This prevents log flushing worker
    // from being dropped prematurely.
    if let Some(guard) = log_guard {
        app.insert_resource(LogWorkerGuard { _guard: guard });
    }

    app
}

/// Global system to spawn the 2D camera required for UI rendering.
fn setup_camera(mut commands: Commands) {
    info!("[Main] Spawning 2D Camera...");
    // Assign a higher camera order for the UI camera so it doesn't share the
    // same order as 3D cameras (prevents repeated Camera order ambiguity warnings).
    // Spawn a 2D camera entity and set an explicit order to avoid conflicts
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
    ));
}

/// Diagnostic system to help detect duplicate camera orders at startup.
fn diagnose_cameras(query: Query<(Entity, &Camera, Option<&Camera2d>, Option<&Camera3d>)>) {
    use std::collections::BTreeMap;

    info!("[Main] Running camera diagnostics...");
    let mut counts: BTreeMap<isize, Vec<String>> = BTreeMap::new();

    for (entity, camera, cam2d, cam3d) in &query {
        let kind = if cam2d.is_some() {
            "Camera2d"
        } else if cam3d.is_some() {
            "Camera3d"
        } else {
            "Camera"
        };

        let entry = counts.entry(camera.order).or_default();
        entry.push(format!("{}(id={:?})", kind, entity));

        info!(
            "[Main] Found camera: entity={:?} order={} kind={}",
            entity, camera.order, kind
        );
    }

    for (order, entities) in counts {
        if entities.len() > 1 {
            warn!(
                "[Main] Duplicate cameras detected with order {}: {:?}. This may cause rendering ambiguities.",
                order, entities
            );
        }
    }
}
