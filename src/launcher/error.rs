//! # Error Handling Module
//!
//! Provides the UI and logic for the application's error state.

use crate::core::states::{AppState, ErrorState};
use crate::launcher::menu::widgets::{ButtonAction, spawn_primary_button};
use crate::ui::theme::Theme;
use bevy::prelude::*;

pub struct ErrorPlugin;

impl Plugin for ErrorPlugin {
    fn build(&self, app: &mut App) {
        // Resource is now managed globally in main.rs/core
        app.add_systems(OnEnter(AppState::Error), setup_error_screen)
            .add_systems(OnExit(AppState::Error), cleanup_error_screen);
    }
}

/// Marker for the error screen root node.
#[derive(Component)]
struct ErrorRoot;

fn setup_error_screen(mut commands: Commands, theme: Res<Theme>, error_state: Res<ErrorState>) {
    warn!("[ErrorUI] Entering Error State: {}", error_state.message);

    let mut actions_container = Entity::PLACEHOLDER;

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
            BackgroundColor(theme.colors.background),
            ErrorRoot,
        ))
        .with_children(|parent| {
            // Icon or Title
            parent.spawn((
                Text::new("SYSTEM CRITICAL ERROR"),
                TextFont {
                    font: theme.fonts.fallback.clone(),
                    font_size: theme.sizes.font_h1,
                    ..default()
                },
                TextColor(theme.colors.danger),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));

            // Error Message Box
            parent
                .spawn((
                    Node {
                        width: Val::Px(600.0),
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(theme.colors.surface),
                ))
                .with_children(|box_node| {
                    box_node.spawn((
                        Text::new(&error_state.message),
                        TextFont {
                            font: theme.fonts.fallback.clone(),
                            font_size: theme.sizes.font_body,
                            ..default()
                        },
                        TextColor(theme.colors.text_primary),
                    ));
                });

            // Action Buttons
            actions_container = parent
                .spawn(Node {
                    margin: UiRect::top(Val::Px(40.0)),
                    ..default()
                })
                .id();
        });

    // Now that the main borrow of 'commands' is over, spawn the button.
    spawn_primary_button(
        &mut commands,
        &theme,
        "EXIT TO DESKTOP",
        ButtonAction::Exit,
        actions_container,
    );
}

fn cleanup_error_screen(mut commands: Commands, query: Query<Entity, With<ErrorRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
