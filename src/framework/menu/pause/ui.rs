//! Pause menu UI spawning and cleanup.

use super::components::{PauseMenuButton, PauseMenuButtonAction, PauseMenuRoot};
use super::state::{PauseMenuMode, PauseMenuState};
use crate::framework::localization::{Localization, LocalizedStrings};
use crate::framework::ui::theme::Theme;
use bevy::prelude::*;

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
    let root = commands
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

    commands.entity(root).with_children(|parent| {
        parent
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
            .with_children(|panel| {
                panel.spawn((
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
                ));

                spawn_pause_button(
                    panel,
                    theme,
                    strings.get("pause-resume", loc),
                    PauseMenuButtonAction::Resume,
                );
                spawn_pause_button(
                    panel,
                    theme,
                    strings.get("pause-settings", loc),
                    PauseMenuButtonAction::OpenSettings,
                );
                spawn_pause_button(
                    panel,
                    theme,
                    strings.get("pause-main-menu", loc),
                    PauseMenuButtonAction::ExitToMainMenu,
                );
                spawn_pause_button(
                    panel,
                    theme,
                    strings.get("pause-exit-game", loc),
                    PauseMenuButtonAction::ExitGame,
                );
            });
    });
}

fn spawn_pause_button(
    parent: &mut bevy::ecs::hierarchy::ChildSpawnerCommands,
    theme: &Theme,
    label: String,
    action: PauseMenuButtonAction,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: theme.sizes.button_height,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(theme.colors.accent),
            PauseMenuButton { action },
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font: theme.fonts.main.clone(),
                    font_size: theme.sizes.font_body,
                    ..default()
                },
                TextColor(theme.colors.text_primary),
            ));
        });
}
