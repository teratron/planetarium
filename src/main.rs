use bevy::{
    camera_controller::free_camera::{/*FreeCamera,*/ FreeCameraPlugin/*, FreeCameraState*/},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Plugin that enables FreeCamera functionality
        .add_plugins(FreeCameraPlugin)
        // Example code plugins
        //.add_plugins((CameraPlugin, CameraSettingsPlugin, ScenePlugin))
        .run();
}
