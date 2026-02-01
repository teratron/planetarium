use bevy::prelude::*;
use crate::states::GameState;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gameplay), setup_gameplay)
            .add_systems(
                Update,
                (gameplay_update, handle_pause_input).run_if(in_state(GameState::Gameplay)),
            )
            .add_systems(OnExit(GameState::Gameplay), cleanup_gameplay);
    }
}

#[derive(Component)]
struct GameplayCamera;

fn setup_gameplay(mut commands: Commands) {
    // Spawn 3D camera for gameplay
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        GameplayCamera,
    ));

    // Ambient light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(4.0, 8.0, 4.0),
        GameplayCamera, // Cleanup with camera for now
    ));

    info!("Gameplay started");
}

fn gameplay_update() {
    // Main game logic goes here
}

fn handle_pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        info!("Game paused");
        next_state.set(GameState::Paused);
    }
}

fn cleanup_gameplay(mut commands: Commands, query: Query<Entity, With<GameplayCamera>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    info!("Gameplay cleaned up");
}
