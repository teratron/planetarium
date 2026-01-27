use bevy::prelude::*;
use bevy::input::mouse::AccumulatedMouseMotion; //Provided by bevy: mouse input resource sense last frame

use std::f32::consts::FRAC_PI_2; //Provided by bevy: pi/2 rad

use crate::cameras::camera::CameraSensitivity; //Component defined in camera.rs

pub struct RotateCameraPlugin;

impl Plugin for RotateCameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(RunFixedMainLoop, rotate_camera);
	}
}

fn rotate_camera(
	accumulated_mouse_motion: Res<AccumulatedMouseMotion>,	
	mut camera: Query<(&mut Transform, &CameraSensitivity), With<Camera>>, //Note the camera component filters for entities that have a camera component
) {
	for (mut movementv, camsense) in camera.iter_mut()
	{	
		let delta = accumulated_mouse_motion.delta; //delta is a 2D vector that stores mouse inputs 

		if delta != Vec2::ZERO { //Makes sure there has been inputs

			let delta_yaw = -delta.x * camsense.x; //yaw - vertical axis y // Rotation is turning left or right
			let delta_pitch = -delta.y * camsense.y; // pitch - horizonal axis // looking up or down
			// Read rotation in YXZ order (yaw, pitch, rollow)
			let (yaw, pitch, roll) = movementv.rotation.to_euler(EulerRot::YXZ); // return f32 radian values from raw data

			//Update yaw and pitch
			let yaw = yaw + delta_yaw; //rotates around y axis
			const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
			let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT); //rotates arond x axis

			movementv.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll); // converts radian values back into engine useable math
		}
	}
}

// ==== accumulated_mouse_motion: Res<AccumulatedMouseMotion>, ====
/*
- A resource type provided by bevy
- Stores the total mouse movement since last frame
pub struct AccumulatedMouseMotion {
    pub delta: Vec2,
}
- We are defining the variable which stores
  two dimention vector "accumukated_mouse_motion" from AccumulatedMouseMotion 
*/

// ==== camera: Query<(&mut Transform, &CameraSensitivity), With<Camera>, ====
/*
-Query<T> is a query wrapper
Find exactly one entity with the required component.
It will panic when:
- Each Interation, the next entity components are pulled out of ECS and assinded to the LCV of (mut movementv, mut camsense)
- looks for a any entity that has both Transform and CameraSensitivity
- player is a local label
*/

// ==== for (mut movementv, camsense) in camera.iter_mut() ====
/*
LCV = number of entitys that match Query<(&mut Transform, &CameraSensitivity), With<Camera>
- (mut movementv, camsense) stores each entity data
- Interator over all entities that match the query from Query
- loop gets skiped if no entitys exist
*/

// ==== RunFixedMainLoop ====
/*
- internal driver system
- Advances the fixed timestep clock
- Decidees how many times FixedUpdate runes thsi frame
- Bridges Update <-> FixedUpdate
┌─ PreUpdate
│
├─ Update
│
├─ RunFixedMainLoop
│   ├─ FixedUpdate   (maybe 0 times)
│   ├─ FixedUpdate   (maybe 1+ times)
│   └─ (catch-up loop)
│
└─ PostUpdate
*/
