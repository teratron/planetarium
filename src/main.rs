use bevy::prelude::*;
use bevy::window::WindowResolution;

pub mod resources;
pub mod states;
pub mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Planetarium".into(),
                resolution: WindowResolution::new(1280, 720),
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(states::StatesPlugin)
        .run();
}
