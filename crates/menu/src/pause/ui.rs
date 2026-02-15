//! Pause menu UI spawning and cleanup.

use super::components::{PauseMenuButton, PauseMenuButtonAction, PauseMenuRoot};
use super::state::{PauseMenuMode, PauseMenuState};
use bevy::prelude::*;
// use launcher::states::AppState; // Removed
use localization::{Localization, LocalizedStrings};
use theme::Theme;

/// Synchronizes pause menu UI with the current pause state.
pub fn sync_pause_menu_ui(
    mut commands: Commands,
    pause_state: Res<PauseMenuState>,
    theme: Res<Theme>,
    loc: Res<Localization>,
    mut strings: ResMut<LocalizedStrings>,
    query: Query<Entity, With<PauseMenuRoot>>,
) {
    if !pause_state.is_changed() {
        return;
    }

    if pause_state.mode == PauseMenuMode::Menu {
        if query.is_empty() {
            spawn_pause_menu(&mut commands, &theme, &loc, &mut strings);
        }
        return;
    }

    despawn_pause_menu(&mut commands, &query);
}

/// Explicit cleanup for pause menu root entities.
pub fn cleanup_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenuRoot>>) {
    despawn_pause_menu(&mut commands, &query);
}

fn despawn_pause_menu(commands: &mut Commands, query: &Query<Entity, With<PauseMenuRoot>>) {
    for entity in query {
        commands.entity(entity).despawn();
    }
}

fn spawn_pause_menu(
    commands: &mut Commands,
    theme: &Theme,
    loc: &Localization,
    strings: &mut LocalizedStrings,
) {
    let root_id = commands
        .spawn((
            PauseMenuRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(theme.colors.background.with_alpha(0.82)),
            ZIndex(80),
        ))
        .id();

    let panel_id = commands
        .spawn((
            Node {
                width: Val::Px(420.0),
                height: Val::Auto,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                padding: UiRect::all(theme.sizes.padding),
                row_gap: Val::Px(10.0),
                ..default()
            },
            BackgroundColor(theme.colors.surface),
        ))
        .id();

    commands.entity(root_id).add_child(panel_id);

    let title_id = commands
        .spawn((
            Text::new(strings.get("pause-title", loc)),
            TextFont {
                font: theme.fonts.bold.clone(),
                font_size: theme.sizes.font_h2,
                ..default()
            },
            TextColor(theme.colors.accent),
            Node {
                margin: UiRect::bottom(Val::Px(10.0)),
                justify_self: JustifySelf::Center,
                ..default()
            },
        ))
        .id();

    commands.entity(panel_id).add_child(title_id);

    spawn_pause_button(
        commands,
        panel_id,
        theme,
        strings.get("pause-resume", loc),
        PauseMenuButtonAction::Resume,
    );
    spawn_pause_button(
        commands,
        panel_id,
        theme,
        strings.get("pause-settings", loc),
        PauseMenuButtonAction::OpenSettings,
    );
    spawn_pause_button(
        commands,
        panel_id,
        theme,
        strings.get("pause-main-menu", loc),
        PauseMenuButtonAction::ExitToMainMenu,
    );
    spawn_pause_button(
        commands,
        panel_id,
        theme,
        strings.get("pause-exit-game", loc),
        PauseMenuButtonAction::ExitGame,
    );
}

fn spawn_pause_button(
    commands: &mut Commands,
    parent: Entity,
    theme: &Theme,
    label: String,
    action: PauseMenuButtonAction,
) {
    let button = commands
        .spawn((
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(theme.colors.accent),
            PauseMenuButton { action },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font: theme.fonts.main.clone(),
                    font_size: theme.sizes.font_body,
                    ..default()
                },
                TextColor(theme.colors.text_primary),
            ));
        })
        .id();

    commands.entity(parent).add_child(button);
}
