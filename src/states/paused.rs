use bevy::prelude::*;
use crate::states::GameState;

pub struct PausedPlugin;

impl Plugin for PausedPlugin {
    fn build(&self, app: &mut App) {
        // Simple pause handling for now
        app.add_systems(OnEnter(GameState::Paused), setup_paused)
           .add_systems(Update, handle_paused_input.run_if(in_state(GameState::Paused)));
    }
}


fn setup_paused() {
    info!("Entered Paused State - Press ESC to resume, Q to Quit to Menu");
}

fn handle_paused_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Gameplay);
    }
    
    if keyboard.just_pressed(KeyCode::KeyQ) {
         next_state.set(GameState::MainMenu);
    }
}
