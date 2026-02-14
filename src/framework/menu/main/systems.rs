//! # Main Menu UI Systems
//!
//! Implements the landing screen with Play, Settings, and Exit buttons.
//! Manages menu state and transitions using professional visual fading.

use crate::framework::localization::LocalizedStrings;
use crate::framework::settings::SettingsOpen;
use crate::framework::states::AppState;
use crate::framework::ui::fading::ScreenFade;
use crate::framework::ui::theme::Theme;
use crate::framework::ui::widgets::{ButtonAction, PrimaryButton, spawn_primary_button};
use crate::framework::utils::despawn_recursive;
use bevy::prelude::*;

/// Marker component for menu root entity.
#[derive(Component)]
pub struct MainMenuRoot;

/// Marker for menu background.
#[derive(Component)]
pub struct MenuBackground;

/// Import menu layout constants from layout module.
use super::layout;

/// System to spawn the main menu UI.
/// Includes a title and professional primary buttons (Play, Settings, Exit).
pub fn spawn_main_menu(
    mut commands: Commands,
    theme: Res<Theme>,
    loc: Res<crate::framework::localization::Localization>,
    mut strings: ResMut<LocalizedStrings>,
) {
    info!("{}", loc.t("log-loading-spawn")); // Assuming we want a spawn log, reuse or add new

    let root_id = spawn_root(&mut commands, &theme);
    let panel_id = spawn_panel(&mut commands, &theme);
    let buttons_id = spawn_buttons_container(&mut commands);

    spawn_menu_buttons(&mut commands, &theme, &loc, &mut strings, buttons_id);
    spawn_title(&mut commands, &theme, &loc, &mut strings, panel_id);

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
fn spawn_title(
    commands: &mut Commands,
    theme: &Theme,
    loc: &crate::framework::localization::Localization,
    strings: &mut LocalizedStrings,
    panel_id: Entity,
) {
    commands.entity(panel_id).with_children(|parent| {
        parent.spawn((
            Text::new(strings.get("menu-title", loc)),
            TextFont {
                font: theme.fonts.bold.clone(),
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
fn spawn_menu_buttons(
    commands: &mut Commands,
    theme: &Theme,
    loc: &crate::framework::localization::Localization,
    strings: &mut LocalizedStrings,
    container_id: Entity,
) {
    let buttons = [
        (strings.get("menu-play", loc), ButtonAction::Play),
        (strings.get("menu-settings", loc), ButtonAction::Settings),
        (strings.get("menu-exit", loc), ButtonAction::Exit),
    ];

    for (label, action) in buttons {
        spawn_primary_button(commands, theme, &label, action, container_id);
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
    mut modal_state: ResMut<crate::framework::ui::modal::ModalState>,
    localization: Res<crate::framework::localization::Localization>,
) {
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            handle_button_action(
                &button.action,
                &mut settings_open,
                &mut fade,
                &mut modal_state,
                &localization,
            );
        }
    }
}

/// Handle individual button action based on button type.
fn handle_button_action(
    action: &ButtonAction,
    settings_open: &mut ResMut<SettingsOpen>,
    fade: &mut ResMut<ScreenFade>,
    modal_state: &mut crate::framework::ui::modal::ModalState,
    localization: &crate::framework::localization::Localization,
) {
    match action {
        ButtonAction::Play => {
            info!("{}", localization.t("log-loading-complete")); // Reuse complete or add specific?
            fade.fade_out(layout::FADE_DURATION_LOADING, AppState::Loading);
        }
        ButtonAction::Settings => {
            let mut args = fluent_bundle::FluentArgs::new();
            args.set("tab", "default"); // Opening settings
            info!(
                "{}",
                localization.t_with_args("log-settings-switch-tab", Some(&args))
            );
            settings_open.0 = true;
        }
        ButtonAction::Exit => {
            info!("{}", localization.t("menu-exit"));
            modal_state.active = Some(crate::framework::ui::modal::ModalType::ConfirmExit);
        }
        ButtonAction::Back => {
            settings_open.0 = false;
        }
    }
}

/// System to despawn the menu UI when exiting MainMenu state.
pub fn despawn_main_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuRoot>>,
    children_query: Query<&Children>,
) {
    for entity in query.iter() {
        despawn_recursive(&mut commands, entity, &children_query);
    }
}
