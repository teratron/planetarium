use bevy::prelude::*;

pub mod resources;
pub mod states;
pub mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Planetarium".into(),
                resolution: (1280., 720.).into(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(states::StatesPlugin)
        .run();
}
