//! # Launcher Module
//!
//! This module manages the application's lifecycle from the initial boot
//! to the transition into the main gameplay.

use bevy::prelude::*;

/// The main plugin for the Launcher module.
/// It aggregates sub-plugins for booting, splash screens, and menus.
pub struct LauncherPlugin;

impl Plugin for LauncherPlugin {
    fn build(&self, _app: &mut App) {
        // Sub-plugins will be registered here in Task [L-102]
    }
}
