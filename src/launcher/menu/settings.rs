//! Settings Screen UI
//!
// Implements a simple modal settings panel with tabs: Graphics, Audio, Controls.
// The panel is toggled by the `SettingsOpen` resource.

use bevy::prelude::*;
use crate::ui::theme::Theme;
use super::widgets::ButtonAction;

#[derive(Component)]
pub struct SettingsRoot;

#[derive(Resource, Default, Debug, Clone)]
pub struct SettingsOpen(pub bool);

/// Spawns the settings UI under the given parent (or as a full-screen overlay).
pub fn spawn_settings_menu(commands: &mut Commands, theme: &Theme) -> Entity {
    info!("[Settings] Spawning settings UI...");

    // Root overlay (using raw components for consistency)
    let root = commands
        .spawn((
            SettingsRoot,
            Node {
                width: Val::Percent(80.0),
                height: Val::Percent(80.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(theme.colors.surface),
        ))
        .id();

    // Title
    commands.entity(root).with_children(|parent| {
        parent.spawn((
            Text::new("Settings"),
            TextFont { font_size: theme.sizes.font_h2, ..default() },
            TextColor(theme.colors.text_primary),
        ));

        // Tabs row (visual-only for now)
        parent
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(40.0),
                    margin: UiRect::top(Val::Px(12.0)),
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(theme.colors.background),
            ))
            .with_children(|tabs| {
                tabs.spawn((
                    Text::new("Graphics"),
                    TextFont { font_size: theme.sizes.font_body, ..default() },
                    TextColor(theme.colors.text_secondary),
                ));
                tabs.spawn((
                    Text::new("Audio"),
                    TextFont { font_size: theme.sizes.font_body, ..default() },
                    TextColor(theme.colors.text_secondary),
                ));
                tabs.spawn((
                    Text::new("Controls"),
                    TextFont { font_size: theme.sizes.font_body, ..default() },
                    TextColor(theme.colors.text_secondary),
                ));
            });

        // Content placeholder
        parent
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(70.0),
                    margin: UiRect::all(Val::Px(12.0)),
                    ..default()
                },
                BackgroundColor(theme.colors.background),
            ))
            .with_children(|content| {
                content.spawn((
                    Text::new("Tab content placeholder"),
                    TextFont { font_size: theme.sizes.font_body, ..default() },
                    TextColor(theme.colors.text_primary),
                ));
            });

        // Back button container and button (spawned as children here)
        parent
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(60.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
            ))
            .with_children(|btn_parent| {
                btn_parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(200.0),
                            height: theme.sizes.button_height,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        BackgroundColor(theme.colors.accent),
                        super::widgets::PrimaryButton { label: "Back".to_string(), action: ButtonAction::Back },
                        super::widgets::ButtonHoverState { base_color: theme.colors.accent, hover_color: theme.colors.accent_muted },
                    ))
                    .with_children(|b| {
                        b.spawn((
                            Text::new("Back"),
                            TextFont { font_size: theme.sizes.font_body, ..default() },
                            TextColor(theme.colors.text_primary),
                        ));
                    });
            });
    });

    root
}

/// Spawns or despawns settings depending on `SettingsOpen` resource.
pub fn spawn_settings_if_needed(
    mut commands: Commands,
    theme: Res<Theme>,
    settings_open: Res<SettingsOpen>,
    query: Query<Entity, With<SettingsRoot>>,
) {
    if settings_open.is_changed() {
        if settings_open.0 && query.is_empty() {
            spawn_settings_menu(&mut commands, &theme);
        } else if !settings_open.0 && !query.is_empty() {
            for e in &query {
                commands.entity(e).despawn();
            }
        }
    }
}
