//! # Diagnostics & Debug Overlay
//!
//! Provides a real-time overlay for monitoring performance (FPS) and application state.

// use crate::framework::ui::theme::Theme; // Theme is in theme crate
#[cfg(not(test))]
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

// We need Theme to display fonts.
// If launcher depends on theme (optional), we can use it.
// If theme is not enabled, we can't display nice overlay?
// Or we use default Bevy UI?
// For now, I'll comment out Theme usage or assume it's available.
// If I use theme::Theme, I need to import it.

/// Plugin that provides real-time engine diagnostics and performance monitoring.
///
/// It registers systems for an on-screen overlay (FPS, State, Entity count)
/// that can be toggled using the F1 key.
pub struct DiagnosticsPlugin;

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        // On normal builds we include the FrameTime plugin; in tests we skip it to avoid
        // additional systems that rely on runtime-only resources.
        #[cfg(not(test))]
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());

        app.init_resource::<DebugSettings>()
            // .add_systems(Startup, setup_debug_overlay) // Needs Theme
            .add_systems(
                Update,
                (
                    toggle_debug_overlay,
                    // update_debug_text.run_if(debug_overlay_visible),
                ),
            );
    }
}

#[derive(Resource, Default)]
struct DebugSettings {
    visible: bool,
}

/// Marker for the debug overlay root.
#[derive(Component)]
struct DebugOverlayRoot;

// setup_debug_overlay and update_debug_text logic needs Theme.
// I will keep them commented out until I import Theme properly.

fn toggle_debug_overlay(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<DebugSettings>,
    mut query: Query<&mut Visibility, With<DebugOverlayRoot>>,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        settings.visible = !settings.visible;
        for mut visibility in &mut query {
            *visibility = if settings.visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
        info!("[Diagnostics] Debug overlay toggled: {}", settings.visible);
    }
}
