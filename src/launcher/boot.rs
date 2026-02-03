use crate::core::config::setup_config;
use crate::core::states::AppState;
use bevy::prelude::*;

pub struct BootPlugin;

impl Plugin for BootPlugin {
    fn build(&self, app: &mut App) {
        info!("[BootPlugin] Initializing...");

        // Register the configuration setup system
        app.add_systems(OnEnter(AppState::Booting), setup_config);

        // Handle transition to next state
        app.add_systems(
            Update,
            check_boot_finished.run_if(in_state(AppState::Booting)),
        );
    }
}

/// Simple system to move from Booting to Splash or MainMenu.
/// In a real app, this would wait for background tasks, auth, etc.
fn check_boot_finished(
    mut next_state: ResMut<NextState<AppState>>,
    cli_args: Res<crate::core::cli::CliArgs>,
) {
    info!("[BootPlugin] Boot sequence complete. Transitioning...");

    if cli_args.skip_splash {
        next_state.set(AppState::MainMenu);
    } else {
        next_state.set(AppState::Splash);
    }
}
