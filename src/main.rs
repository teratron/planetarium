use bevy::{
    camera_controller::free_camera::{FreeCameraPlugin /*, FreeCamera, FreeCameraState*/},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FreeCameraPlugin)
        //.add_plugins((CameraPlugin, CameraSettingsPlugin, ScenePlugin))
        .run();
}
