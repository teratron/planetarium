use crate::resources::{save_data::SaveData, settings::GameSettings};
use crate::states::GameState;
use bevy::prelude::*;

pub struct BootPlugin;

impl Plugin for BootPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Boot), setup_boot)
            .add_systems(
                Update,
                boot_complete_check.run_if(in_state(GameState::Boot)),
            );
    }
}

fn setup_boot(mut commands: Commands) {
    // Spawn 2D camera for UI
    commands.spawn(Camera2d::default());

    // Initialize global resources
    commands.insert_resource(GameSettings::default());
    commands.insert_resource(SaveData::default());

    info!("Boot phase started");
}

fn boot_complete_check(mut next_state: ResMut<NextState<GameState>>) {
    // Boot is instant in most cases - proceed to splash
    info!("Boot complete, transitioning to Splash");
    next_state.set(GameState::Splash);
}
