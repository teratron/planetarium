use bevy::prelude::*;
use crate::states::GameState;
use crate::ui::theme::{colors, fonts};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<AppExit>()
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
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
pub enum MenuButton {
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
            for (button_type, label) in [
                (MenuButton::NewGame, "New Game"),
                (MenuButton::Continue, "Continue"),
                (MenuButton::Settings, "Settings"),
                (MenuButton::Credits, "Credits"),
                (MenuButton::Exit, "Exit"),
            ] {
                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(280.0),
                            height: Val::Px(56.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::bottom(Val::Px(16.0)),
                            ..default()
                        },
                        BackgroundColor(colors::BUTTON_NORMAL),
                        button_type,
                    ))
                    .with_child((
                        Text::new(label),
                        TextFont {
                            font_size: fonts::BUTTON_TEXT_SIZE,
                            ..default()
                        },
                        TextColor(colors::TEXT_PRIMARY),
                    ));
            }
        });

    info!("Main menu setup complete");
}

fn handle_menu_interaction(
    query: Query<(&Interaction, &MenuButton), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut app_exit: MessageWriter<AppExit>,
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
                app_exit.write(AppExit::Success);
            }
        }
    }
}

fn animate_menu_hover(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<MenuButton>)>,
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
        commands.entity(entity).despawn();
    }
    info!("Main menu cleaned up");
}
