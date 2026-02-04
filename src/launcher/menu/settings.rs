//! Settings Screen UI
//!
// Implements a simple modal settings panel with tabs: Graphics, Audio, Controls.
// The panel is toggled by the `SettingsOpen` resource.

use super::widgets::ButtonAction;
use crate::core::config::UserSettings;
use crate::ui::theme::Theme;
use bevy::prelude::*;

pub mod components;
pub mod layout;

pub use components::{
    FullscreenToggle, MasterVolumeControl, MusicVolumeControl, ResolutionHeightControl,
    ResolutionWidthControl, SFXVolumeControl, SettingsRoot,
};
pub use layout::panel as panel_layout;

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
            TextFont {
                font_size: theme.sizes.font_h2,
                ..default()
            },
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
                    TextFont {
                        font_size: theme.sizes.font_body,
                        ..default()
                    },
                    TextColor(theme.colors.text_secondary),
                ));
                tabs.spawn((
                    Text::new("Audio"),
                    TextFont {
                        font_size: theme.sizes.font_body,
                        ..default()
                    },
                    TextColor(theme.colors.text_secondary),
                ));
                tabs.spawn((
                    Text::new("Controls"),
                    TextFont {
                        font_size: theme.sizes.font_body,
                        ..default()
                    },
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
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    row_gap: Val::Px(12.0),
                    padding: UiRect::all(Val::Px(12.0)),
                    ..default()
                },
                BackgroundColor(theme.colors.background),
            ))
            .with_children(|content| {
                // Graphics Settings
                content.spawn((
                    Text::new("Graphics Settings"),
                    TextFont {
                        font_size: theme.sizes.font_body,
                        ..default()
                    },
                    TextColor(theme.colors.text_secondary),
                ));

                // Resolution label and value
                content
                    .spawn((Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(12.0),
                        ..default()
                    },))
                    .with_children(|row| {
                        row.spawn((
                            Text::new("Width (px):"),
                            TextFont {
                                font_size: theme.sizes.font_body,
                                ..default()
                            },
                            TextColor(theme.colors.text_primary),
                        ));
                        row.spawn((
                            ResolutionWidthControl,
                            Text::new("1280"),
                            TextFont {
                                font_size: theme.sizes.font_body,
                                ..default()
                            },
                            TextColor(theme.colors.text_primary),
                        ));
                    });

                content
                    .spawn((Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(12.0),
                        ..default()
                    },))
                    .with_children(|row| {
                        row.spawn((
                            Text::new("Height (px):"),
                            TextFont {
                                font_size: theme.sizes.font_body,
                                ..default()
                            },
                            TextColor(theme.colors.text_primary),
                        ));
                        row.spawn((
                            ResolutionHeightControl,
                            Text::new("720"),
                            TextFont {
                                font_size: theme.sizes.font_body,
                                ..default()
                            },
                            TextColor(theme.colors.text_primary),
                        ));
                    });

                // Fullscreen toggle
                content
                    .spawn((Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(12.0),
                        ..default()
                    },))
                    .with_children(|row| {
                        row.spawn((
                            Text::new("Fullscreen:"),
                            TextFont {
                                font_size: theme.sizes.font_body,
                                ..default()
                            },
                            TextColor(theme.colors.text_primary),
                        ));
                        row.spawn((
                            FullscreenToggle,
                            Text::new("OFF"),
                            TextFont {
                                font_size: theme.sizes.font_body,
                                ..default()
                            },
                            TextColor(theme.colors.text_primary),
                        ));
                    });

                // Audio Settings
                content.spawn((
                    Text::new("Audio Settings"),
                    TextFont {
                        font_size: theme.sizes.font_body,
                        ..default()
                    },
                    TextColor(theme.colors.text_secondary),
                ));

                content
                    .spawn((Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(12.0),
                        ..default()
                    },))
                    .with_children(|row| {
                        row.spawn((
                            Text::new("Master Volume:"),
                            TextFont {
                                font_size: theme.sizes.font_body,
                                ..default()
                            },
                            TextColor(theme.colors.text_primary),
                        ));
                        row.spawn((
                            MasterVolumeControl,
                            Text::new("0.80"),
                            TextFont {
                                font_size: theme.sizes.font_body,
                                ..default()
                            },
                            TextColor(theme.colors.text_primary),
                        ));
                    });

                content
                    .spawn((Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(12.0),
                        ..default()
                    },))
                    .with_children(|row| {
                        row.spawn((
                            Text::new("Music Volume:"),
                            TextFont {
                                font_size: theme.sizes.font_body,
                                ..default()
                            },
                            TextColor(theme.colors.text_primary),
                        ));
                        row.spawn((
                            MusicVolumeControl,
                            Text::new("0.70"),
                            TextFont {
                                font_size: theme.sizes.font_body,
                                ..default()
                            },
                            TextColor(theme.colors.text_primary),
                        ));
                    });

                content
                    .spawn((Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(12.0),
                        ..default()
                    },))
                    .with_children(|row| {
                        row.spawn((
                            Text::new("SFX Volume:"),
                            TextFont {
                                font_size: theme.sizes.font_body,
                                ..default()
                            },
                            TextColor(theme.colors.text_primary),
                        ));
                        row.spawn((
                            SFXVolumeControl,
                            Text::new("1.00"),
                            TextFont {
                                font_size: theme.sizes.font_body,
                                ..default()
                            },
                            TextColor(theme.colors.text_primary),
                        ));
                    });
            });

        // Back button container and button (spawned as children here)
        parent
            .spawn((Node {
                width: Val::Percent(100.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },))
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
                        super::widgets::PrimaryButton {
                            label: "Back".to_string(),
                            action: ButtonAction::Back,
                        },
                        super::widgets::ButtonHoverState {
                            base_color: theme.colors.accent,
                            hover_color: theme.colors.accent_muted,
                        },
                    ))
                    .with_children(|b| {
                        b.spawn((
                            Text::new("Back"),
                            TextFont {
                                font_size: theme.sizes.font_body,
                                ..default()
                            },
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

/// Update UI display values to match current UserSettings.
pub fn update_settings_ui(
    settings: Res<UserSettings>,
    mut width_q: Query<
        &mut Text,
        (
            With<ResolutionWidthControl>,
            Without<ResolutionHeightControl>,
            Without<FullscreenToggle>,
            Without<MasterVolumeControl>,
            Without<MusicVolumeControl>,
            Without<SFXVolumeControl>,
        ),
    >,
    mut height_q: Query<
        &mut Text,
        (
            With<ResolutionHeightControl>,
            Without<ResolutionWidthControl>,
            Without<FullscreenToggle>,
            Without<MasterVolumeControl>,
            Without<MusicVolumeControl>,
            Without<SFXVolumeControl>,
        ),
    >,
    mut fullscreen_q: Query<
        &mut Text,
        (
            With<FullscreenToggle>,
            Without<ResolutionWidthControl>,
            Without<ResolutionHeightControl>,
            Without<MasterVolumeControl>,
            Without<MusicVolumeControl>,
            Without<SFXVolumeControl>,
        ),
    >,
    mut master_q: Query<
        &mut Text,
        (
            With<MasterVolumeControl>,
            Without<ResolutionWidthControl>,
            Without<ResolutionHeightControl>,
            Without<FullscreenToggle>,
            Without<MusicVolumeControl>,
            Without<SFXVolumeControl>,
        ),
    >,
    mut music_q: Query<
        &mut Text,
        (
            With<MusicVolumeControl>,
            Without<ResolutionWidthControl>,
            Without<ResolutionHeightControl>,
            Without<FullscreenToggle>,
            Without<MasterVolumeControl>,
            Without<SFXVolumeControl>,
        ),
    >,
    mut sfx_q: Query<
        &mut Text,
        (
            With<SFXVolumeControl>,
            Without<ResolutionWidthControl>,
            Without<ResolutionHeightControl>,
            Without<FullscreenToggle>,
            Without<MasterVolumeControl>,
            Without<MusicVolumeControl>,
        ),
    >,
) {
    if let Ok(mut text) = width_q.single_mut() {
        text.0 = settings.display.width.to_string();
    }
    if let Ok(mut text) = height_q.single_mut() {
        text.0 = settings.display.height.to_string();
    }
    if let Ok(mut text) = fullscreen_q.single_mut() {
        text.0 = if settings.display.fullscreen {
            "ON"
        } else {
            "OFF"
        }
        .to_string();
    }
    if let Ok(mut text) = master_q.single_mut() {
        text.0 = format!("{:.2}", settings.audio.master_volume);
    }
    if let Ok(mut text) = music_q.single_mut() {
        text.0 = format!("{:.2}", settings.audio.music_volume);
    }
    if let Ok(mut text) = sfx_q.single_mut() {
        text.0 = format!("{:.2}", settings.audio.sfx_volume);
    }
}
