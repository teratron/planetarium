use bevy::prelude::*;

use crate::player::{Velocity, Accumulateinput, Player}; 

use bevy::prelude::Camera; //Provides the Camera component from bevy

pub struct AccumulateInputPlugin;

impl Plugin for AccumulateInputPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, spawn_accumulateinput);
	}
}

const SPEED: f32 = 0.09;

fn spawn_accumulateinput (
	keyboard_input: Res<ButtonInput<KeyCode>>,
	player: Single<(&mut Accumulateinput, &mut Velocity, &mut Transform), (Without<Camera>, With<Player>)>, //obtains pointers to all matching enetities
	camera: Single<&Transform, (With<Camera>, Without<Player>)>,
) {

	let (mut input, mut velocity, mut movementv) = player.into_inner();

	input.movement = Vec2::ZERO;

	if keyboard_input.pressed(KeyCode::KeyW) {
		input.movement.y += 1.0;
	}

	if keyboard_input.pressed(KeyCode::KeyS) {
		input.movement.y -= 1.0;
	}

	if keyboard_input.pressed(KeyCode::KeyA) {
		input.movement.x -= 1.0;
	}

	if keyboard_input.pressed(KeyCode::KeyD) {
		input.movement.x += 1.0;
	}

	let input_3d = Vec3 {
		x: input.movement.x,
		y: 0.0,
		z: -input.movement.y,
	};

	//Rotate the input forward is aligned with the camera forward direction
	let rotated_input = camera.rotation * input_3d;

	velocity.0 = rotated_input.clamp_length_max(1.0) * SPEED; 
//input_3d * SPEED;

	movementv.translation += velocity.0;
}

// ==== let rotated_input = camera.rotation * input_3d; ====
/*
Take the user input direction and rotate it so it matches where the camera is facing
- Movement goes forward relative to the camera not character
- "Quat * Vec3 -> Vec3" Is quaternion vector multication 
- The Camera defines the axis of chacter movement or coordinate system
*/

// ==== mut player: Query<(&mut Accumulateinput, &mut Velocity, &mut Transform)>, ===
/*
- The above function query for any entity with that following members
*/
