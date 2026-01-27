use bevy::prelude::*;

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
}

