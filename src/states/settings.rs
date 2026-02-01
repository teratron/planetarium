use bevy::prelude::*;
use crate::states::GameState;
use crate::ui::theme::{colors, fonts};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Settings), setup_settings)
            .add_systems(
                Update,
                handle_settings_back.run_if(in_state(GameState::Settings)),
            )
            .add_systems(OnExit(GameState::Settings), cleanup_settings);
    }
}

#[derive(Component)]
struct SettingsScreen;

fn setup_settings(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(colors::MENU_BACKGROUND),
        SettingsScreen,
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("Settings\n\nPress ESC to return"),
            TextFont {
                font_size: fonts::HEADER_SIZE,
                ..default()
            },
            TextColor(colors::TEXT_PRIMARY),
            TextLayout::new_with_justify(Justify::Center),
        ));
    });
    
    info!("Settings screen setup complete");
}

fn handle_settings_back(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        info!("Returning to Main Menu from Settings");
        next_state.set(GameState::MainMenu);
    }
}

fn cleanup_settings(mut commands: Commands, query: Query<Entity, With<SettingsScreen>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    info!("Settings screen cleaned up");
}
