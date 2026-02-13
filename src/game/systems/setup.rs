//! # Game Setup Systems
//!
//! Scene initialization and world spawning systems.

use bevy::prelude::*;

use crate::framework::ui::theme::Theme;
use crate::game::components::{GameWorldRoot, Rotates};

/// Spawns the 3D game world with a placeholder planet, light, and camera.
pub fn setup_game_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    theme: Res<Theme>,
) {
    info!("[Game] Initializing 3D game world...");

    // Spawn a root entity to group world objects
    commands
        .spawn((Visibility::default(), Transform::default(), GameWorldRoot))
        .with_children(|parent| {
            // A placeholder planet (Sphere)
            if let Ok(sphere_mesh) = Sphere::new(5.0).mesh().ico(5) {
                parent.spawn((
                    Mesh3d(meshes.add(sphere_mesh)),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: theme.colors.accent,
                        reflectance: 0.5,
                        perceptual_roughness: 0.2,
                        ..default()
                    })),
                    Transform::from_xyz(0.0, 0.0, 0.0),
                    Rotates,
                ));
            } else {
                warn!("[Game] Failed to generate sphere mesh; skipping planet placeholder.");
            }

            // Let there be light!
            parent.spawn((
                PointLight {
                    intensity: 1500000.0,
                    shadows_enabled: true,
                    ..default()
                },
                Transform::from_xyz(10.0, 10.0, 10.0),
            ));

            // Setup the camera
            parent.spawn((
                Camera3d::default(),
                Transform::from_xyz(0.0, 0.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
            ));
        });

    info!("[Game] Handover complete. Enjoy the Cosmos!");
}
