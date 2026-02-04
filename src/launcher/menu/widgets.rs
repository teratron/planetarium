//! # UI Widgets
//!
//! Generic, reusable UI widgets for the main menu and settings screens.
//! Built on Bevy's native UI system with custom styling and interaction.

use crate::core::assets::AssetManifest;
use crate::ui::theme::Theme;
use bevy::prelude::*;

pub mod components;
pub mod constants;

pub use components::{PrimaryButton, ButtonAction, ButtonHoverState, Slider, SliderTrack, Dropdown};
pub use constants::button as button_constants;

/// Spawns a high-quality primary button widget.
///
/// # Arguments
/// * `commands` - The entity commands to spawn the button.
/// * `theme` - Visual design tokens for styling.
/// * `label` - The text to display on the button.
/// * `action` - The logic to trigger when clicked.
/// * `parent` - Optional parent entity to attach the button to.
pub fn spawn_primary_button(
    commands: &mut Commands,
    theme: &Theme,
    label: &str,
    action: ButtonAction,
    parent: Entity,
) -> Entity {
    let button_color = theme.colors.accent;
    let button_color_hover = theme.colors.accent_muted;

    let button = commands
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
            BackgroundColor(button_color),
            PrimaryButton {
                label: label.to_string(),
                action: action.clone(),
            },
            ButtonHoverState {
                base_color: button_color,
                hover_color: button_color_hover,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font_size: theme.sizes.font_body,
                    ..default()
                },
                TextColor(theme.colors.text_primary),
            ));
        })
        .id();

    // Add to parent if provided
    if parent != Entity::PLACEHOLDER {
        commands.entity(parent).add_child(button);
    }

    button
}

/// System to create a slider widget.
pub fn spawn_slider(
    commands: &mut Commands,
    theme: &Theme,
    label: &str,
    min: f32,
    max: f32,
    value: f32,
    setting_key: &str,
    parent: Entity,
) -> Entity {
    let slider_height = 40.0;
    let track_height = 8.0;

    let slider = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(slider_height),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Stretch,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            Slider {
                label: label.to_string(),
                min,
                max,
                value,
                setting_key: setting_key.to_string(),
            },
        ))
        .with_children(|parent| {
            // Label
            parent.spawn((
                Text::new(label),
                TextFont {
                    font_size: theme.sizes.font_body,
                    ..default()
                },
                TextColor(theme.colors.text_primary),
            ));

            // Track container
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(track_height),
                        margin: UiRect::top(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(theme.colors.surface),
                ))
                .with_children(|parent| {
                    // Progress fill
                    let progress = (value - min) / (max - min).max(0.001);
                    parent.spawn((
                        Node {
                            width: Val::Percent(progress * 100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(theme.colors.accent),
                    ));
                });
        })
        .id();

    if parent != Entity::PLACEHOLDER {
        commands.entity(parent).add_child(slider);
    }

    slider
}

/// System to create a dropdown widget.
pub fn spawn_dropdown(
    commands: &mut Commands,
    theme: &Theme,
    label: &str,
    options: Vec<String>,
    selected_index: usize,
    setting_key: &str,
    parent: Entity,
) -> Entity {
    let _selected_text = options
        .get(selected_index)
        .map(|s| s.as_str())
        .unwrap_or("Select");

    let dropdown = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Auto,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            Dropdown {
                label: label.to_string(),
                options: options.clone(),
                selected_index,
                setting_key: setting_key.to_string(),
                is_open: false,
            },
        ))
        .with_children(|parent| {
            // Label
            parent.spawn((
                Text::new(label),
                TextFont {
                    font_size: theme.sizes.font_body,
                    ..default()
                },
                TextColor(theme.colors.text_primary),
            ));

            // Button to show/hide options
            parent.spawn((
                Button,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    padding: UiRect::horizontal(Val::Px(10.0)),
                    margin: UiRect::top(Val::Px(5.0)),
                    ..default()
                },
                BackgroundColor(theme.colors.surface),
            ));
        })
        .id();

    if parent != Entity::PLACEHOLDER {
        commands.entity(parent).add_child(dropdown);
    }

    dropdown
}

/// System to handle button interactions, updating visual state and playing audio feedback.
///
/// Visual feedback is handled via `BackgroundColor` and `ButtonHoverState`.
/// Audio feedback uses paths defined in the `AssetManifest` and respects `RuntimeAudioState` volume.
pub fn button_interaction_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    manifest: Res<AssetManifest>,
    audio_state: Res<crate::launcher::menu::reactive::RuntimeAudioState>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonHoverState),
        (Changed<Interaction>, With<Button>, With<PrimaryButton>),
    >,
) {
    for (interaction, mut bg_color, hover_state) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *bg_color = BackgroundColor(hover_state.hover_color);

                // Play hover sound if defined in manifest
                if let Some(path) = manifest.audio("hover") {
                    commands.spawn((
                        AudioPlayer::new(asset_server.load(path)),
                        PlaybackSettings {
                            mode: bevy::audio::PlaybackMode::Despawn,
                            volume: bevy::audio::Volume::Linear(audio_state.sfx * 0.5), // Muted hover
                            ..default()
                        },
                    ));
                }
            }
            Interaction::Pressed => {
                info!("[UI] Button pressed");

                // Play click sound if defined in manifest
                if let Some(path) = manifest.audio("click") {
                    commands.spawn((
                        AudioPlayer::new(asset_server.load(path)),
                        PlaybackSettings {
                            mode: bevy::audio::PlaybackMode::Despawn,
                            volume: bevy::audio::Volume::Linear(audio_state.sfx),
                            ..default()
                        },
                    ));
                }
            }
            Interaction::None => {
                *bg_color = BackgroundColor(hover_state.base_color);
            }
        }
    }
}

/// System to handle slider interactions.
pub fn slider_interaction_system(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            info!("[UI] Slider adjusted");
        }
    }
}

/// System to handle dropdown open/close toggle.
pub fn dropdown_interaction_system(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            info!("[UI] Dropdown toggled");
        }
    }
}
