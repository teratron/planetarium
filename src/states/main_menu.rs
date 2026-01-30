use crate::states::GameState;
use crate::ui::buttons::spawn_menu_button;
use crate::ui::theme::{colors, fonts};
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(
                Update,
                (handle_menu_interaction, animate_menu_hover).run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu);
    }
}

#[derive(Component)]
struct MainMenuRoot;

#[derive(Component, Clone, Copy)]
enum MenuButton {
    NewGame,
    Continue,
    Settings,
    Credits,
    Exit,
}

fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(colors::MENU_BACKGROUND),
            MainMenuRoot,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("PLANETARIUM"),
                TextFont {
                    font_size: fonts::TITLE_SIZE,
                    ..default()
                },
                TextColor(colors::TEXT_PRIMARY),
                Node {
                    margin: UiRect::bottom(Val::Px(60.0)),
                    ..default()
                },
            ));

            // Menu buttons
            spawn_menu_button(parent, MenuButton::NewGame, "New Game");
            spawn_menu_button(parent, MenuButton::Continue, "Continue");
            spawn_menu_button(parent, MenuButton::Settings, "Settings");
            spawn_menu_button(parent, MenuButton::Credits, "Credits");
            spawn_menu_button(parent, MenuButton::Exit, "Exit");
        });

    info!("Main menu setup complete");
}

fn handle_menu_interaction(
    query: Query<(&Interaction, &MenuButton), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit_events: EventWriter<AppExit>,
) {
    for (interaction, button) in &query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match button {
            MenuButton::NewGame => {
                info!("New Game selected");
                next_state.set(GameState::Loading);
            }
            MenuButton::Continue => {
                info!("Continue selected");
                next_state.set(GameState::Loading);
            }
            MenuButton::Settings => {
                info!("Settings selected");
                next_state.set(GameState::Settings);
            }
            MenuButton::Credits => {
                info!("Credits selected");
                next_state.set(GameState::Credits);
            }
            MenuButton::Exit => {
                info!("Exit requested");
                exit_events.send(AppExit::Success);
            }
        }
    }
}

fn animate_menu_hover(
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MenuButton>),
    >,
) {
    for (interaction, mut bg_color) in &mut query {
        *bg_color = match interaction {
            Interaction::Pressed => BackgroundColor(colors::BUTTON_PRESSED),
            Interaction::Hovered => BackgroundColor(colors::BUTTON_HOVER),
            Interaction::None => BackgroundColor(colors::BUTTON_NORMAL),
        };
    }
}

fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
    info!("Main menu cleaned up");
}
