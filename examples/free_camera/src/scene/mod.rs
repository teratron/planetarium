mod camera;
mod light;
mod world;

use bevy::prelude::*;
use crate::scene::{
    camera::{CameraPlugin, CameraSettingsPlugin},
    light::LightPlugin,
    world::WorldPlugin,
};

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((LightPlugin, WorldPlugin, CameraPlugin, CameraSettingsPlugin));
    }
}
