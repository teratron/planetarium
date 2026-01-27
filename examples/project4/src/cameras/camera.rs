use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, spawn_camera);
	}
}

#[derive(Debug, Component, Deref, DerefMut)]
pub struct CameraSensitivity(Vec2);

//Here we define the default as we don't want it tobe <0,0> or it won't move
impl Default for CameraSensitivity {
	fn default() ->  Self {
		Self(
			// configure untill seems normal
			//Vec2::new(horizontal_sensitivity, vertical_sensitivity)
			Vec2::new(0.003, 0.002),
		)
	}
}

fn spawn_camera(
	mut commands: Commands,
) {

	let camera = (
		Camera3d::default(),
		CameraSensitivity::default(),
		Transform::from_xyz(-2.5,4.5,9.0).looking_at(Vec3::ZERO, Vec3::Y),
	);

	commands.spawn(camera);
}

// Camera3d::default()
/*
Several components are attached to the entity
- Camera
- Transform
- GlobalTransform
- Projection
*/
