//! # Launcher Crate
//!
//! The central framework crate that ties together booting, configuration, states,
//! splash screens, and core plugins.

pub mod boot;
pub mod config;
pub mod diagnostics;
pub mod loading;
pub mod splash;
pub mod states;
pub mod utils;

pub use boot::BootPlugin;
pub use config::setup_config;
pub use diagnostics::DiagnosticsPlugin;
pub use loading::LoadingPlugin;
pub use splash::SplashPlugin;
pub use states::AppState;

use bevy::prelude::*;

pub mod prelude {
    pub use crate::LauncherPlugin;
    pub use crate::config::{AppPaths, UserSettings};
    pub use crate::loading::assets::AssetManifest;
    pub use crate::states::AppState;
}

pub struct LauncherPlugin;

impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        // 1. Register States
        app.init_state::<AppState>();
        app.insert_resource(states::ErrorState::default());

        // 2. Add Core Framework Plugins (from other crates)
        app.add_plugins((
            theme::ThemePlugin,
            localization::LocalizationPlugin,
            transitions::FadingPlugin::<AppState>::default(),
        ));

        // 3. Add Launcher Modules
        app.add_plugins((DiagnosticsPlugin, BootPlugin, SplashPlugin, LoadingPlugin));
    }
}
