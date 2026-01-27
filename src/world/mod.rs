use bevy::prelude::*;
use std::f32::consts::{FRAC_PI_4, PI};

use crate::tailwind;

pub mod floor;
pub mod light;

fn spawn_world(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let floor = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0)));
    let sphere = meshes.add(Sphere::new(0.5));
    let wall = meshes.add(Cuboid::new(0.2, 4.0, 3.0));

    let blue_material = materials.add(Color::from(tailwind::BLUE_700));
    let red_material = materials.add(Color::from(tailwind::RED_950));
    let white_material = materials.add(Color::WHITE);

    // Top side of floor
    commands.spawn((
        Mesh3d(floor.clone()),
        MeshMaterial3d(white_material.clone()),
    ));
    // Under side of floor
    commands.spawn((
        Mesh3d(floor.clone()),
        MeshMaterial3d(white_material.clone()),
        Transform::from_xyz(0.0, -0.01, 0.0).with_rotation(Quat::from_rotation_x(PI)),
    ));
    // Blue sphere
    commands.spawn((
        Mesh3d(sphere.clone()),
        MeshMaterial3d(blue_material.clone()),
        Transform::from_xyz(3.0, 1.5, 0.0),
    ));
    // Tall wall
    commands.spawn((
        Mesh3d(wall.clone()),
        MeshMaterial3d(white_material.clone()),
        Transform::from_xyz(-3.0, 2.0, 0.0),
    ));
    // Cube behind wall
    commands.spawn((
        Mesh3d(cube.clone()),
        MeshMaterial3d(blue_material.clone()),
        Transform::from_xyz(-4.2, 0.5, 0.0),
    ));
    // Hidden cube under floor
    commands.spawn((
        Mesh3d(cube.clone()),
        MeshMaterial3d(red_material.clone()),
        Transform {
            translation: Vec3::new(3.0, -2.0, 0.0),
            rotation: Quat::from_euler(EulerRot::YXZEx, FRAC_PI_4, FRAC_PI_4, 0.0),
            ..default()
        },
    ));
}