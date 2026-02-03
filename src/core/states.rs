//! # Application States
//!
//! This module defines the primary states of the application.
//! These states drive the high-level logic and determine which systems
//! should be running at any given time.

use bevy::prelude::*;

/// The primary state machine for the application.
///
/// We use `States` derive to let Bevy manage transitions and
/// allow us to use `OnEnter`, `OnExit`, and `OnUpdate` schedules.
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    /// Stage 1: System initialization, configuration loading, and environment checks.
    #[default]
    Booting,

    /// Stage 2: Displaying branding and splash screens.
    Splash,

    /// Stage 3: The main menu where the user interacts and configures settings.
    MainMenu,

    /// Stage 4: Asynchronous loading of game assets for the active world.
    Loading,

    /// Stage 5: The active gameplay state.
    InGame,

    /// Critical Error state: Shows a crash/error screen to the user.
    Error,
}
