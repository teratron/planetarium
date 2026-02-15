//! # Boot Module
//!
//! Handles application startup, path resolution, and initial configuration.

pub mod localization;
pub mod paths;
pub mod platform_paths;
pub mod theme;

pub use paths::AppPaths;

use crate::boot::localization::Localization;
use crate::config::setup_config;
use crate::loading::assets::setup_asset_manifest;
use crate::states::{AppState, ErrorState};
use bevy::prelude::*;

pub struct BootPlugin;

impl Plugin for BootPlugin {
    fn build(&self, app: &mut App) {
        info!("[BootPlugin] Initializing...");

        // Register initialization systems sequentially
        app.add_systems(
            OnEnter(AppState::Booting),
            (
                setup_config,
                setup_asset_manifest,
                localization::setup_localization,
                theme::setup_theme,
            )
                .chain(),
        );

        // Handle transition to next state
        app.add_systems(
            Update,
            (
                localization::apply_language_change_system,
                theme::check_theme_ready,
                check_boot_finished,
            )
                .run_if(in_state(AppState::Booting)),
        );
    }
}

/// Simple system to move from Booting to Splash or MainMenu.
fn check_boot_finished(
    mut next_state: ResMut<NextState<AppState>>,
    cli_args: Res<crate::config::cli::CliArgs>,
    error_state: Res<ErrorState>,
    localization: Option<Res<Localization>>,
    theme_loading: Res<theme::ThemeLoadingState>,
) {
    // 1. Check for critical errors
    if !error_state.message.is_empty() {
        error!("[Boot] Critical error detected: {}", error_state.message);
        // We let the ErrorPlugin (in Menu) handle the transition to Error state if it's monitoring?
        // Or we force it here. Since we are in Booting, we might need to go to Error state manually.
        // For now, let's assume the error handler system picks it up or we set it.
        // app.add_systems(Update, check_error_state) ??
        // In the original code, error state transition happens via a state change.
        return;
    }

    // 2. Wait for dependencies
    if !theme_loading.is_ready {
        return;
    }

    if let Some(loc) = localization {
        info!("{}", loc.t("log-boot-complete"));

        if cli_args.skip_splash {
            next_state.set(AppState::MainMenu);
        } else {
            next_state.set(AppState::Splash);
        }
    } else {
        // Localization not ready yet? It should be synchronous in setup, but if we made it async...
        // For now, setup_localization is synchronous, so this branch shouldn't be hit unless something failed.
    }
}
