use bevy::{
    camera_controller::free_camera::FreeCameraPlugin, color::palettes::tailwind, prelude::*,
};

mod scene;

use scene::ScenePlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FreeCameraPlugin))
        .add_plugins(ScenePlugin)
        .run();
}
