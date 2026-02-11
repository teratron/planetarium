//! # Game World Logic
//!
//! Handles the setup, orchestration, and cleanup of the 3D game world.

use crate::core::states::AppState;
use crate::ui::theme::Theme;
use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_game_world)
            .add_systems(
                Update,
                rotate_planet
                    .run_if(in_state(AppState::InGame))
                    .run_if(crate::game::pause_menu::state::gameplay_active),
            )
            .add_systems(OnExit(AppState::InGame), cleanup_game_world);
    }
}

/// Marker component for the game world entities.
#[derive(Component)]
pub struct GameWorldRoot;

/// Marker for entities with idle rotation while gameplay is active.
#[derive(Component)]
struct Rotates;

fn setup_game_world(
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

fn rotate_planet(mut query: Query<&mut Transform, With<Rotates>>, time: Res<Time>) {
    let delta = time.delta_secs() * 0.3;
    for mut transform in &mut query {
        transform.rotate_y(delta);
    }
}

fn cleanup_game_world(mut commands: Commands, query: Query<Entity, With<GameWorldRoot>>) {
    info!("[Game] Cleaning up world...");
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
