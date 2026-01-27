use bevy::prelude::*;

use crate::player::Accumulateinput;

pub struct TranslateCameraPlugin;

impl Plugin for TranslateCameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(RunFixedMainLoop, translate_camera);
	}
}

fn translate_camera(
    mut camera: Single<&mut Transform, With<Camera>>,
    player: Single<&Transform, (With<Accumulateinput>, Without<Camera>)>,
) {
    camera.translation = player.translation;
//	camera.translation.x += 0.001;
//	camera.translation.y += 0.5;
//	camera.translation.z += 0.5;
}

/*
- The goal of this function is have the camera follow the player so they keep the same position 
*/
