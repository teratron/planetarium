//! # Application State Machine
//!
//! Defines the primary states of the application lifecycle.
//! These states drive the high-level logic and determine which systems
//! should be running at any given time.

use bevy::prelude::*;

use strum_macros::{AsRefStr, EnumString, VariantNames};

/// The primary state machine for the application.
///
/// We use the `States` derive to let Bevy manage transitions and
/// allow us to use `OnEnter`, `OnExit`, and `run_if(in_state(...))` schedules.
#[derive(
    States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default, AsRefStr, EnumString, VariantNames,
)]
#[strum(serialize_all = "lowercase")]
#[non_exhaustive]
pub enum AppState {
    /// Stage 1: System initialization, configuration loading, and environment checks.
    #[default]
    Booting,

    /// Stage 2: Displaying branding and splash screens.
    #[strum(serialize = "splash")]
    Splash,

    /// Stage 3: The main menu where the user interacts and configures settings.
    #[strum(serialize = "mainmenu", serialize = "menu")]
    MainMenu,

    /// Stage 4: Asynchronous loading of game assets for the active world.
    #[strum(serialize = "loading")]
    Loading,

    /// Stage 5: The active gameplay state.
    #[strum(serialize = "ingame", serialize = "game")]
    InGame,

    /// The game is paused (pause menu visible, gameplay frozen).
    Paused,

    /// Settings screen (accessible from main menu or pause menu).
    Settings,

    /// Game over screen showing final results.
    GameOver,

    /// Critical error state: shows a crash/error screen to the user.
    Error,
}

/// Global resource to hold the last critical error message.
#[derive(Resource, Debug, Clone, Default)]
#[non_exhaustive]
pub struct ErrorState {
    /// Human-readable description of the error.
    pub message: String,
}
