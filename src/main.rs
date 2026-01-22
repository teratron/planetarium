use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, spawn_camera)
        .add_plugins(DefaultPlugins)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3d::default());
    commands.spawn(DirectionalLight::default());
    commands.spawn(PointLight::default());
}
