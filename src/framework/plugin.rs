//! # Framework Plugin
//!
//! The top-level plugin that bundles all framework sub-plugins.
//! Add this single plugin to your Bevy `App` to get all reusable
//! infrastructure: state management, splash, loading, menus, etc.

use bevy::prelude::*;

/// Aggregates all framework-level sub-plugins into a single registration point.
///
/// # Usage
///
/// ```rust,ignore
/// App::new()
///     .add_plugins(DefaultPlugins)
///     .add_plugins(FrameworkPlugin)
///     .run();
/// ```
pub struct FrameworkPlugin;

impl Plugin for FrameworkPlugin {
    fn build(&self, _app: &mut App) {
        // Sub-plugins will be registered here as modules are migrated.
        // Example (future phases):
        //   app.add_plugins((
        //       super::states::StatesPlugin,
        //       super::splash::SplashPlugin,
        //       super::loading::LoadingPlugin,
        //       super::menu::MenuPlugin,
        //       super::settings::SettingsPlugin,
        //       super::audio::AudioPlugin,
        //       super::camera::CameraPlugin,
        //   ));
    }
}
