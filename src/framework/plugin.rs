//! # Framework Plugin
//!
//! The top-level plugin that bundles all framework sub-plugins.
//! Add this single plugin to your Bevy `App` to get all reusable
//! infrastructure: state management, splash, loading, menus, etc.

use bevy::prelude::*;

use super::boot::BootPlugin;
use super::diagnostics::DiagnosticsPlugin;
use super::error::ErrorPlugin;
use super::loading::LoadingPlugin;
use super::menu::MenuPlugin;
use super::splash::SplashPlugin;

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
    fn build(&self, app: &mut App) {
        app.init_resource::<crate::framework::settings::PendingSettings>()
            .add_systems(
                Update,
                (
                    crate::framework::settings::pending_systems::handle_settings_action_buttons,
                    crate::framework::settings::pending_systems::initialize_pending_settings,
                ),
            );
        app.add_plugins((
            BootPlugin,
            SplashPlugin,
            MenuPlugin,
            LoadingPlugin,
            ErrorPlugin,
            DiagnosticsPlugin,
            super::ui::fading::FadingPlugin,
            super::ui::modal::ModalPlugin,
            super::ui::theme::ThemePlugin,
            super::camera::CameraPlugin,
            super::audio::AudioPlugin,
        ));
    }
}
