use bevy::color::{palettes::tailwind, Color};
use bevy::light::PointLight;
use bevy::prelude::*;

pub struct LightPlugin; /*{
pub position: Vec3,
pub color: Vec3,
}*/

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_lights);
    }
}

fn spawn_lights(mut commands: Commands) {
    // Main light
    commands.spawn((
        PointLight {
            color: Color::from(tailwind::ORANGE_300),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 3.0, 0.0),
    ));
    // Light behind wall
    commands.spawn((
        PointLight {
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-3.5, 3.0, 0.0),
    ));
    // Light under floor
    commands.spawn((
        PointLight {
            color: Color::from(tailwind::RED_300),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, -0.5, 0.0),
    ));
}

/*use bevy::prelude::*;

pub struct LightPlugin;

impl Plugin for LightPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, spawn_light);
	}
}

fn spawn_light(mut commands: Commands)
{
	let light = (
		PointLight {
			shadows_enabled: true,
			..default()
		},
		Transform::from_xyz(4.0,8.0,4.0),
	);

	commands.spawn(light);
}*/
