//! # UI Widgets
//!
//! Generic, reusable UI widgets for the main menu and settings screens.
//! Built on Bevy's native UI system with custom styling and interaction.

use crate::core::assets::AssetManifest;
use crate::core::config::settings::Quality;
use crate::ui::theme::Theme;
use bevy::prelude::*;

pub mod components;
pub mod constants;

pub use components::{
    ButtonAction, ButtonHoverState, Dropdown, DropdownOption, DropdownOptionsList, DropdownText,
    PrimaryButton, Slider, SliderFill, SliderTrack,
};
pub use constants::button as button_constants;

/// Spawns a high-quality primary button widget.
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
                    font: theme.fonts.main.clone(),
                    font_size: theme.sizes.font_body,
                    ..default()
                },
                TextColor(theme.colors.text_primary),
            ));
        })
        .id();

    if parent != Entity::PLACEHOLDER {
        commands.entity(parent).add_child(button);
    }

    button
}

/// Small helper to encapsulate slider numeric parameters.
pub struct SliderSpec {
    pub min: f32,
    pub max: f32,
    pub value: f32,
}

/// Helper to spawn a slider widget.
pub fn spawn_slider(
    commands: &mut Commands,
    theme: &Theme,
    label: &str,
    spec: SliderSpec,
    setting_key: &str,
    parent: Entity,
) -> Entity {
    let slider_height = 40.0;
    let track_height = 8.0;

    let SliderSpec { min, max, value } = spec;

    let slider_id = commands
        .spawn((
            Button, // Make it interactive
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
        .id();

    commands.entity(slider_id).with_children(|parent| {
        // Label
        parent.spawn((
            Text::new(label),
            TextFont {
                font: theme.fonts.main.clone(),
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
            .with_children(|p| {
                // Progress fill
                let progress = (value - min) / (max - min).max(0.001);
                p.spawn((
                    Node {
                        width: Val::Percent(progress * 100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(theme.colors.accent),
                    SliderFill(slider_id),
                ));
            });
    });

    if parent != Entity::PLACEHOLDER {
        commands.entity(parent).add_child(slider_id);
    }

    slider_id
}

/// System to handle button interactions, updating visual state and playing audio feedback.
#[allow(clippy::type_complexity)]
pub fn button_interaction_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    manifest: Res<AssetManifest>,
    audio_state: Res<crate::launcher::menu::reactive::RuntimeAudioState>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonHoverState),
        (
            Changed<Interaction>,
            With<Button>,
            Or<(With<PrimaryButton>, With<Slider>)>,
        ),
    >,
) {
    for (interaction, mut bg_color, hover_state) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *bg_color = BackgroundColor(hover_state.hover_color);
                if let Some(path) = manifest.audio("hover") {
                    commands.spawn((
                        AudioPlayer::new(asset_server.load(path)),
                        PlaybackSettings {
                            mode: bevy::audio::PlaybackMode::Despawn,
                            volume: bevy::audio::Volume::Linear(audio_state.sfx * 0.5),
                            ..default()
                        },
                    ));
                }
            }
            Interaction::Pressed => {
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

// ...
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
    let selected_text = options
        .get(selected_index)
        .map(|s| s.as_str())
        .unwrap_or("Select");

    let dropdown_node = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Auto,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        })
        .id();

    commands.entity(dropdown_node).with_children(|parent| {
        // Label
        parent.spawn((
            Text::new(label),
            TextFont {
                font: theme.fonts.main.clone(),
                font_size: theme.sizes.font_body,
                ..default()
            },
            TextColor(theme.colors.text_primary),
        ));

        // Button to show/hide options
        parent
            .spawn((
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
                Dropdown {
                    label: label.to_string(),
                    options: options.clone(),
                    selected_index,
                    setting_key: setting_key.to_string(),
                    is_open: false,
                },
                ButtonHoverState {
                    base_color: theme.colors.surface,
                    hover_color: theme.colors.surface_light,
                },
            ))
            .with_children(|btn| {
                // Selected value text
                btn.spawn((
                    Text::new(selected_text),
                    TextFont {
                        font: theme.fonts.main.clone(),
                        font_size: theme.sizes.font_body,
                        ..default()
                    },
                    TextColor(theme.colors.text_primary),
                    DropdownText,
                ));
            });
    });

    if parent != Entity::PLACEHOLDER {
        commands.entity(parent).add_child(dropdown_node);
    }

    dropdown_node
}

/// System to handle slider interaction and update UserSettings.
pub fn slider_interaction_system(
    mut interaction_query: Query<(&Interaction, &GlobalTransform, &ComputedNode, &mut Slider)>,
    mut settings: ResMut<crate::core::config::UserSettings>,
    windows: Query<&Window>,
) {
    let window = if let Ok(w) = windows.single() {
        w
    } else {
        return;
    };
    let mouse_pos = if let Some(pos) = window.cursor_position() {
        pos
    } else {
        return;
    };

    for (interaction, transform, computed, mut slider) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            let width = computed.size().x;
            if width <= 0.0 {
                continue;
            }

            let node_pos = transform.translation().truncate();
            let half_width = width / 2.0;
            let min_x = node_pos.x - half_width;

            let relative_x = (mouse_pos.x - min_x) / width;
            let relative_x = relative_x.clamp(0.0, 1.0);

            let new_value = slider.min + (slider.max - slider.min) * relative_x;
            slider.value = new_value;

            // Apply to settings
            match slider.setting_key.as_str() {
                "master_volume" => settings.audio.master_volume = new_value,
                "music_volume" => settings.audio.music_volume = new_value,
                "sfx_volume" => settings.audio.sfx_volume = new_value,
                _ => warn!("[UI] Unknown slider setting key: {}", slider.setting_key),
            }
        }
    }
}

/// System to update slider fill width based on current value.
pub fn update_slider_visuals(
    slider_query: Query<&Slider>,
    mut fill_query: Query<(&mut Node, &SliderFill)>,
) {
    for (mut node, fill) in &mut fill_query {
        if let Ok(slider) = slider_query.get(fill.0) {
            let progress = (slider.value - slider.min) / (slider.max - slider.min).max(0.001);
            node.width = Val::Percent(progress * 100.0);
        }
    }
}

/// System to handle dropdown toggle.
#[allow(clippy::type_complexity)]
pub fn dropdown_interaction_system(
    mut commands: Commands,
    theme: Res<Theme>,
    mut dropdown_query: Query<
        (Entity, &Interaction, &mut Dropdown, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut option_lists: Query<(Entity, &DropdownOptionsList)>,
) {
    for (entity, interaction, mut dropdown, mut bg_color) in &mut dropdown_query {
        if *interaction == Interaction::Pressed {
            dropdown.is_open = !dropdown.is_open;
            *bg_color = BackgroundColor(theme.colors.surface_light); // Visual feedback

            if dropdown.is_open {
                // Spawn options
                commands.entity(entity).with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                top: Val::Percent(100.0),
                                left: Val::Px(0.0),
                                width: Val::Percent(100.0),
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            BackgroundColor(theme.colors.background),
                            DropdownOptionsList(entity),
                        ))
                        .with_children(|list| {
                            for (i, option_text) in dropdown.options.iter().enumerate() {
                                list.spawn((
                                    Button,
                                    Node {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(35.0),
                                        padding: UiRect::left(Val::Px(10.0)),
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    BackgroundColor(theme.colors.surface),
                                    DropdownOption {
                                        parent_dropdown: entity,
                                        index: i,
                                    },
                                    ButtonHoverState {
                                        base_color: theme.colors.surface,
                                        hover_color: theme.colors.surface_light,
                                    },
                                ))
                                .with_children(|btn| {
                                    btn.spawn((
                                        Text::new(option_text),
                                        TextFont {
                                            font: theme.fonts.main.clone(),
                                            font_size: theme.sizes.font_body,
                                            ..default()
                                        },
                                        TextColor(theme.colors.text_primary),
                                    ));
                                });
                            }
                        });
                });
            } else {
                // Find and despawn options list
                for (list_entity, list) in &mut option_lists {
                    if list.0 == entity {
                        // Use non-recursive despawn for compatibility with current Bevy version.
                        // If recursive removal is later required, consider walking children via `Children`.
                        commands.entity(list_entity).despawn();
                    }
                }
            }
        }
    }
}

/// System to handle dropdown option selection.
#[allow(clippy::type_complexity)]
pub fn dropdown_option_interaction_system(
    mut commands: Commands,
    mut settings: ResMut<crate::core::config::UserSettings>,
    mut dropdown_query: Query<(&mut Dropdown, &Children)>,
    mut text_query: Query<&mut Text, With<DropdownText>>,
    option_query: Query<(&Interaction, &DropdownOption), (Changed<Interaction>, With<Button>)>,
    option_list_query: Query<(Entity, &DropdownOptionsList)>,
) {
    for (interaction, option) in &option_query {
        if *interaction == Interaction::Pressed
            && let Ok((mut dropdown, children)) = dropdown_query.get_mut(option.parent_dropdown)
        {
            dropdown.selected_index = option.index;
            dropdown.is_open = false;

            // Update text
            if let Some(selected_text) = dropdown.options.get(option.index) {
                for child in children.iter() {
                    if let Ok(mut text) = text_query.get_mut(child) {
                        text.0 = selected_text.clone();
                    }
                }
            }

            // Apply setting
            match dropdown.setting_key.as_str() {
                "quality" => {
                    // Map index to Quality enum and apply to settings
                    let quality = match option.index {
                        0 => Quality::Low,
                        1 => Quality::Medium,
                        2 => Quality::High,
                        3 => Quality::Ultra,
                        _ => Quality::Medium,
                    };
                    settings.graphics.quality = quality.clone();
                    info!("[Settings] Quality set to {:?}", quality);
                }
                "resolution" => {
                    let res_str = &dropdown.options[option.index];
                    if let Some((w, h)) = parse_resolution(res_str) {
                        settings.display.width = w;
                        settings.display.height = h;
                        info!("[Settings] Resolution set to {}x{}", w, h);
                    }
                }
                "language" => {
                    // Apply language string directly (options should contain locale IDs like "en-US"/"ru-RU")
                    if let Some(lang) = dropdown.options.get(option.index) {
                        settings.language = lang.clone();
                        info!("[Settings] Language set to {}", lang);
                    }
                }
                _ => warn!("[Settings] Unknown dropdown key: {}", dropdown.setting_key),
            }

            // Close dropdown (despawn list)
            for (list_entity, list) in &option_list_query {
                if list.0 == option.parent_dropdown {
                    commands.entity(list_entity).despawn();
                }
            }
        }
    }
}

/// Helper to convert dropdown index into a `Quality` value.
pub fn quality_from_index(index: usize) -> Quality {
    match index {
        0 => Quality::Low,
        1 => Quality::Medium,
        2 => Quality::High,
        3 => Quality::Ultra,
        _ => Quality::Medium,
    }
}

fn parse_resolution(res: &str) -> Option<(u32, u32)> {
    let parts: Vec<&str> = res.split('x').collect();
    if parts.len() == 2 {
        let w = parts[0].parse().ok()?;
        let h = parts[1].parse().ok()?;
        Some((w, h))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quality_mapping() {
        assert_eq!(quality_from_index(0), Quality::Low);
        assert_eq!(quality_from_index(1), Quality::Medium);
        assert_eq!(quality_from_index(2), Quality::High);
        assert_eq!(quality_from_index(3), Quality::Ultra);
        assert_eq!(quality_from_index(999), Quality::Medium);
    }
}
