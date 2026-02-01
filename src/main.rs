use bevy::prelude::*;
use bevy::window::WindowResolution;

pub mod config;
pub mod resources;
pub mod states;
pub mod ui;

fn main() {
    // Load config before creating the app
    let game_config = config::load_config();
    let (width, height) = game_config.graphics.resolution;
    let vsync = game_config.graphics.vsync;
    let fullscreen = game_config.graphics.fullscreen;
    
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Planetarium".into(),
                resolution: WindowResolution::new(width, height),
                present_mode: if vsync {
                    bevy::window::PresentMode::AutoVsync
                } else {
                    bevy::window::PresentMode::AutoNoVsync
                },
                mode: if fullscreen {
                    bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary)
                } else {
                    bevy::window::WindowMode::Windowed
                },
                ..default()
            }),
            ..default()
        }))
        .add_plugins(config::ConfigPlugin)
        .add_plugins(states::StatesPlugin)
        .run();
}
