//! # Main Menu UI Layout
//!
//! Implements the landing screen with Play, Settings, and Exit buttons.
//! Manages menu state and transitions.

use bevy::prelude::*;
use crate::core::states::AppState;
use crate::ui::theme::Theme;
use super::widgets::{
    spawn_primary_button, ButtonAction, PrimaryButton,
};

/// Marker component for menu root entity.
#[derive(Component)]
pub struct MainMenuRoot;

/// Marker for menu background.
#[derive(Component)]
pub struct MenuBackground;

/// System to spawn the main menu UI.
pub fn spawn_main_menu(
    mut commands: Commands,
    theme: Res<Theme>,
) {
    info!("[MenuPlugin] Spawning main menu...");

    // Root container
    let root = commands
        .spawn((
            MainMenuRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(theme.colors.background),
        ))
        .id();

    // Menu content panel
    let menu_panel = commands
        .spawn((
            Node {
                width: Val::Px(400.0),
                height: Val::Auto,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(theme.sizes.padding),
                ..default()
            },
            BackgroundColor(theme.colors.surface),
        ))
        .id();

    // Title
    commands.entity(menu_panel).with_children(|parent| {
        parent.spawn((
            Text::new("PLANETARIUM"),
            TextFont {
                font_size: theme.sizes.font_h1,
                ..default()
            },
            TextColor(theme.colors.accent),
            Node {
                margin: UiRect::bottom(Val::Px(40.0)),
                ..default()
            },
        ));
    });

    // Buttons container
    let buttons_container = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Auto,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                ..default()
            },
        ))
        .id();

    // Spawn buttons
    spawn_primary_button(
        &mut commands,
        &theme,
        "PLAY",
        ButtonAction::Play,
        buttons_container,
    );

    spawn_primary_button(
        &mut commands,
        &theme,
        "SETTINGS",
        ButtonAction::Settings,
        buttons_container,
    );

    spawn_primary_button(
        &mut commands,
        &theme,
        "EXIT",
        ButtonAction::Exit,
        buttons_container,
    );

    commands.entity(menu_panel).add_child(buttons_container);
    commands.entity(root).add_child(menu_panel);
}

/// System to handle main menu button clicks.
pub fn handle_menu_button_clicks(
    interaction_query: Query<
        (&Interaction, &PrimaryButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button.action {
                ButtonAction::Play => {
                    info!("[MainMenu] Play button clicked. Transitioning to Loading...");
                    next_state.set(AppState::Loading);
                }
                ButtonAction::Settings => {
                    info!("[MainMenu] Settings button clicked. Opening settings...");
                    // TODO: Transition to settings screen or show modal
                }
                ButtonAction::Exit => {
                    info!("[MainMenu] Exit button clicked. Exiting application...");
                    std::process::exit(0);
                }
                ButtonAction::Back => {
                    info!("[MainMenu] Back button clicked.");
                }
            }
        }
    }
}

/// System to despawn the menu UI when exiting MainMenu state.
pub fn despawn_main_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuRoot>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
