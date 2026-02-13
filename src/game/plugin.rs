//! # Game Plugin
//!
//! The top-level plugin that bundles all game-specific logic.

use bevy::prelude::*;

use super::config::GameplayConfig;
use super::systems;
use crate::config::AppPaths;
use crate::framework::states::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Load gameplay config during initialization
        let gameplay_config = load_gameplay_config(app);

        app.insert_resource(gameplay_config)
            .add_systems(OnEnter(AppState::InGame), systems::setup::setup_game_world)
            .add_systems(
                Update,
                systems::gameplay::rotate_planet
                    .run_if(in_state(AppState::InGame))
                    .run_if(crate::framework::menu::pause::state::gameplay_active),
            )
            .add_systems(
                OnExit(AppState::InGame),
                systems::cleanup::cleanup_game_world,
            );
    }
}

fn load_gameplay_config(app: &App) -> GameplayConfig {
    let paths = app
        .world()
        .get_resource::<AppPaths>()
        .expect("AppPaths must be initialized");
    let config_path = paths.assets_dir.join("configs").join("gameplay.ron");

    if config_path.exists() {
        match std::fs::read_to_string(&config_path) {
            Ok(content) => match ron::from_str::<GameplayConfig>(&content) {
                Ok(config) => {
                    info!("[Config] Loaded gameplay settings from {:?}", config_path);
                    config
                }
                Err(e) => {
                    error!(
                        "[Config] Failed to parse gameplay.ron: {}. Using defaults.",
                        e
                    );
                    GameplayConfig::default()
                }
            },
            Err(e) => {
                error!(
                    "[Config] Failed to read gameplay.ron: {}. Using defaults.",
                    e
                );
                GameplayConfig::default()
            }
        }
    } else {
        warn!(
            "[Config] gameplay.ron not found at {:?}. Using defaults.",
            config_path
        );
        GameplayConfig::default()
    }
}
