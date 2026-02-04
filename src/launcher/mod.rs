//! # Launcher Module
//!
//! This module manages the application's lifecycle from the initial boot
//! to the transition into the main gameplay.

use bevy::prelude::*;

// Importing sub-modules
pub mod boot;
pub mod loading;
pub mod menu;
pub mod splash;

// Using the plugin structs from sub-modules
use boot::BootPlugin;
use loading::LoadingPlugin;
use menu::MenuPlugin;
use splash::SplashPlugin;

/// The main plugin for the Launcher module.
/// It aggregates sub-plugins for booting, splash screens, and menus.
pub struct LauncherPlugin;

impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        // Registering sub-plugins
        app.add_plugins((
            BootPlugin,
            SplashPlugin,
            MenuPlugin,
            LoadingPlugin,
            crate::ui::fading::FadingPlugin,
            crate::ui::theme::ThemePlugin,
        ));
    }
}
