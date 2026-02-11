//! Dropdown Widget
//!
//! Provides dropdown/select widget creation and interaction handling.

use crate::core::assets::AssetCache;
use crate::core::config::settings::SettingKey;
use crate::ui::theme::Theme;
use bevy::prelude::*;

use super::base::Widget;
use super::components::{
    ButtonHoverState, Dropdown, DropdownOption, DropdownOptionsList, DropdownText,
};

/// Configuration for a dropdown. Grouping related arguments makes the API
/// easier to maintain and keeps function signatures small (fewer than 7 args).
pub struct DropdownSpec {
    pub label: String,
    pub options: Vec<String>,
    pub display_values: Option<Vec<String>>,
    pub selected_index: usize,
    pub setting_key: SettingKey,
}

/// Widget adapter for dropdowns.
pub struct DropdownWidget;

impl Widget for DropdownWidget {
    type Spec = DropdownSpec;

    fn spawn(commands: &mut Commands, theme: &Theme, spec: Self::Spec, parent: Entity) -> Entity {
        spawn_dropdown(commands, theme, spec, parent)
    }
}

/// System to create a dropdown widget.
pub fn spawn_dropdown(
    commands: &mut Commands,
    theme: &Theme,
    spec: DropdownSpec,
    parent: Entity,
) -> Entity {
    let DropdownSpec {
        label,
        options,
        display_values,
        selected_index,
        setting_key,
    } = spec;

    let selected_text = display_values
        .as_ref()
        .and_then(|d| d.get(selected_index))
        .or_else(|| options.get(selected_index))
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
            Text::new(label.clone()),
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
                    display_values: display_values.clone(),
                    selected_index,
                    setting_key,
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

/// System to handle dropdown toggle.
#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
pub fn dropdown_interaction_system(
    mut commands: Commands,
    theme: Res<Theme>,
    asset_server: Res<AssetServer>,
    manifest: Res<crate::core::assets::AssetManifest>,
    mut cache: ResMut<AssetCache>,
    audio_state: Res<crate::launcher::menu::reactive::RuntimeAudioState>,
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

            // Play open/close sound
            let sound_key = if dropdown.is_open { "open" } else { "close" };
            if let Some(handle) = cache.get_or_load_audio(sound_key, &asset_server, &manifest) {
                commands.spawn((
                    AudioPlayer::new(handle),
                    PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Despawn,
                        volume: bevy::audio::Volume::Linear(audio_state.sfx),
                        ..default()
                    },
                ));
            }

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

fn parse_resolution_string(s: &str) -> Option<(u32, u32)> {
    let parts: Vec<&str> = s.split('x').collect();
    if parts.len() != 2 {
        return None;
    }
    match (parts[0].parse(), parts[1].parse()) {
        (Ok(w), Ok(h)) => Some((w, h)),
        _ => None,
    }
}

fn apply_dropdown_setting(
    settings: &mut crate::core::config::UserSettings,
    setting_key: &SettingKey,
    options: &[String],
    index: usize,
) {
    match setting_key {
        SettingKey::Quality => {
            // Map index to Quality enum and apply to settings
            let quality = super::quality_from_index(index);
            settings.graphics.quality = quality.clone();
            info!("[Settings] Quality set to {:?}", quality);
        }
        SettingKey::Resolution => {
            if let Some((w, h)) = options
                .get(index)
                .and_then(|res_str| parse_resolution_string(res_str))
            {
                settings.display.width = w;
                settings.display.height = h;
                info!("[Settings] Resolution set to {}x{}", w, h);
            }
        }
        SettingKey::Fullscreen => {
            if let Some(val) = options.get(index).and_then(|s| s.parse::<bool>().ok()) {
                settings.display.fullscreen = val;
                info!("[Settings] Fullscreen set to {}", val);
            }
        }
        SettingKey::Vsync => {
            if let Some(val) = options.get(index).and_then(|s| s.parse::<bool>().ok()) {
                settings.display.vsync = val;
                info!("[Settings] VSync set to {}", val);
            }
        }
        SettingKey::AllowMultipleInstances => {
            if let Some(val) = options.get(index).and_then(|s| s.parse::<bool>().ok()) {
                settings.allow_multiple_instances = val;
                info!("[Settings] Allow multiple instances set to {}", val);
            }
        }
        SettingKey::Language => {
            // Apply language string directly (options should contain locale IDs like "en-US"/"ru-RU")
            if let Some(lang) = options.get(index) {
                settings.language = lang.clone();
                info!("[Settings] Language set to {}", lang);
            }
        }
        SettingKey::Theme => {
            if let Some(theme) = options.get(index) {
                settings.theme = theme.clone();
                info!("[Settings] Theme set to {}", theme);
            }
        }
        _ => warn!("[Settings] Unknown dropdown key: {:?}", setting_key),
    }
}

/// System to handle dropdown option selection.
#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
pub fn dropdown_option_interaction_system(
    mut commands: Commands,
    mut settings: ResMut<crate::core::config::UserSettings>,
    asset_server: Res<AssetServer>,
    manifest: Res<crate::core::assets::AssetManifest>,
    mut cache: ResMut<AssetCache>,
    audio_state: Res<crate::launcher::menu::reactive::RuntimeAudioState>,
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

            // Play select sound
            if let Some(handle) = cache.get_or_load_audio("select", &asset_server, &manifest) {
                commands.spawn((
                    AudioPlayer::new(handle),
                    PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Despawn,
                        volume: bevy::audio::Volume::Linear(audio_state.sfx),
                        ..default()
                    },
                ));
            }

            // Update text
            // Update button text to use display values when available, otherwise use option value
            let display_text = dropdown
                .display_values
                .as_ref()
                .and_then(|d| d.get(option.index))
                .or_else(|| dropdown.options.get(option.index))
                .cloned();

            if let Some(selected_text) = display_text {
                for child in children.iter() {
                    if let Ok(mut text) = text_query.get_mut(child) {
                        text.0 = selected_text.clone();
                    }
                }
            }

            // Apply setting
            apply_dropdown_setting(
                &mut settings,
                &dropdown.setting_key,
                &dropdown.options,
                option.index,
            );

            // Close dropdown (despawn list)
            for (list_entity, list) in &option_list_query {
                if list.0 == option.parent_dropdown {
                    commands.entity(list_entity).despawn();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::settings::SettingKey;

    #[test]
    fn apply_allow_multiple_instances_updates_user_settings() {
        let mut settings = crate::core::config::UserSettings::default();
        let options = vec!["false".to_string(), "true".to_string()];

        apply_dropdown_setting(
            &mut settings,
            &SettingKey::AllowMultipleInstances,
            &options,
            1,
        );
        assert!(settings.allow_multiple_instances);

        apply_dropdown_setting(
            &mut settings,
            &SettingKey::AllowMultipleInstances,
            &options,
            0,
        );
        assert!(!settings.allow_multiple_instances);
    }
}
