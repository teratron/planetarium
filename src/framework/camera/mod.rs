//! # Camera Framework
//!
//! Provides core camera setup and diagnostics (e.g., UI camera).
//! For more specialized game cameras (orbit, flycam), see `game/camera`.

use bevy::prelude::*;

/// Registers standard framework cameras and diagnostics.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui_camera)
            .add_systems(Startup, diagnose_cameras.after(setup_ui_camera));
    }
}

/// Global system to spawn the 2D camera required for UI rendering.
fn setup_ui_camera(mut commands: Commands) {
    info!("[Camera] Spawning 2D UI Camera...");
    // Assign a higher camera order for the UI camera so it doesn't share the
    // same order as 3D cameras (prevents repeated Camera order ambiguity warnings).
    // Spawn a 2D camera entity and set an explicit order to avoid conflicts
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
    ));
}

/// Diagnostic system to help detect duplicate camera orders at startup.
fn diagnose_cameras(query: Query<(Entity, &Camera, Option<&Camera2d>, Option<&Camera3d>)>) {
    use std::collections::BTreeMap;

    info!("[Camera] Running camera diagnostics...");
    let mut counts: BTreeMap<isize, Vec<String>> = BTreeMap::new();

    for (entity, camera, cam2d, cam3d) in &query {
        let kind = if cam2d.is_some() {
            "Camera2d"
        } else if cam3d.is_some() {
            "Camera3d"
        } else {
            "Camera"
        };

        counts
            .entry(camera.order)
            .or_default()
            .push(format!("{}(id={:?})", kind, entity));

        debug!(
            "[Camera] Found camera: entity={:?} order={} kind={}",
            entity, camera.order, kind
        );
    }

    for (order, entities) in counts {
        if entities.len() > 1 {
            warn!(
                "[Camera] Duplicate cameras detected with order {}: {:?}. This may cause rendering ambiguities.",
                order, entities
            );
        }
    }
}
