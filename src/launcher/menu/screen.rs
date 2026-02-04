//! # Main Menu UI Layout
//!
//! Implements the landing screen with Play, Settings, and Exit buttons.
//! Manages menu state and transitions using professional visual fading.

use super::settings::SettingsOpen;
use super::widgets::{ButtonAction, PrimaryButton, spawn_primary_button};
use crate::core::states::AppState;
use crate::ui::fading::ScreenFade;
use crate::ui::theme::Theme;
use bevy::prelude::*;

/// Marker component for menu root entity.
#[derive(Component)]
pub struct MainMenuRoot;

/// Marker for menu background.
#[derive(Component)]
pub struct MenuBackground;

/// Import menu layout constants from parent module.
use super::layout;

/// System to spawn the main menu UI.
/// Includes a title and professional primary buttons (Play, Settings, Exit).
pub fn spawn_main_menu(mut commands: Commands, theme: Res<Theme>) {
    info!("[MenuPlugin] Spawning main menu...");

    let root_id = spawn_root(&mut commands, &theme);
    let panel_id = spawn_panel(&mut commands, &theme);
    let buttons_id = spawn_buttons_container(&mut commands);

    spawn_menu_buttons(&mut commands, &theme, buttons_id);
    spawn_title(&mut commands, &theme, panel_id);

    commands.entity(panel_id).add_child(buttons_id);
    commands.entity(root_id).add_child(panel_id);
}

/// Spawn the root container for the entire menu.
fn spawn_root(commands: &mut Commands, theme: &Theme) -> Entity {
    commands
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
        .id()
}

/// Spawn the menu content panel.
fn spawn_panel(commands: &mut Commands, theme: &Theme) -> Entity {
    commands
        .spawn((
            Node {
                width: layout::PANEL_WIDTH,
                height: layout::PANEL_HEIGHT,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(theme.sizes.padding),
                ..default()
            },
            BackgroundColor(theme.colors.surface),
        ))
        .id()
}

/// Spawn the buttons container.
fn spawn_buttons_container(commands: &mut Commands) -> Entity {
    commands
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Auto,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Stretch,
            ..default()
        },))
        .id()
}

/// Spawn the menu title.
fn spawn_title(commands: &mut Commands, theme: &Theme, panel_id: Entity) {
    commands.entity(panel_id).with_children(|parent| {
        parent.spawn((
            Text::new("PLANETARIUM"),
            TextFont {
                font_size: theme.sizes.font_h1,
                ..default()
            },
            TextColor(theme.colors.accent),
            Node {
                margin: UiRect::bottom(layout::TITLE_MARGIN_BOTTOM),
                ..default()
            },
        ));
    });
}

/// Spawn all menu buttons (Play, Settings, Exit).
fn spawn_menu_buttons(commands: &mut Commands, theme: &Theme, container_id: Entity) {
    let buttons = [
        ("PLAY", ButtonAction::Play),
        ("SETTINGS", ButtonAction::Settings),
        ("EXIT", ButtonAction::Exit),
    ];

    for (label, action) in buttons {
        spawn_primary_button(commands, theme, label, action, container_id);
    }
}

// Filter alias to reduce clippy `type_complexity` warnings.
type MenuButtonFilter = (Changed<Interaction>, With<Button>);

/// System to handle main menu button clicks, initiating transitions or opening panels.
///
/// Uses `ScreenFade` for professional visual transitions between application states.
pub fn handle_menu_button_clicks(
    interaction_query: Query<(&Interaction, &PrimaryButton), MenuButtonFilter>,
    mut settings_open: ResMut<SettingsOpen>,
    mut fade: ResMut<ScreenFade>,
) {
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            handle_button_action(&button.action, &mut settings_open, &mut fade);
        }
    }
}

/// Handle individual button action based on button type.
fn handle_button_action(
    action: &ButtonAction,
    settings_open: &mut ResMut<SettingsOpen>,
    fade: &mut ResMut<ScreenFade>,
) {
    match action {
        ButtonAction::Play => {
            info!("[MainMenu] Play button clicked. Fading out to Loading...");
            fade.fade_out(layout::FADE_DURATION_LOADING, AppState::Loading);
        }
        ButtonAction::Settings => {
            info!("[MainMenu] Settings button clicked. Opening settings...");
            settings_open.0 = true;
        }
        ButtonAction::Exit => {
            info!("[MainMenu] Exit button clicked. Exiting application...");
            std::process::exit(0);
        }
        ButtonAction::Back => {
            info!("[MainMenu] Back button clicked.");
            settings_open.0 = false;
        }
    }
}

/// System to despawn the menu UI when exiting MainMenu state.
pub fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
