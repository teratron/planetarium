//! Slider Widget
//!
//! Provides slider widget creation and interaction handling for numeric values.

use crate::framework::loading::assets::AssetCache;
use crate::framework::menu::events::{UiAudioEvent, play_ui_audio};
use crate::framework::settings::SettingKey;
use crate::framework::ui::theme::Theme;
use bevy::prelude::*;

use super::base::Widget;
use super::components::{Slider, SliderFill};

/// Small helper to encapsulate slider numeric parameters.
#[derive(Debug, Clone)]
pub struct SliderSpec {
    pub min: f32,
    pub max: f32,
    pub value: f32,
}

/// Spec for spawning a slider widget.
#[derive(Debug, Clone)]
pub struct SliderWidgetSpec {
    pub label: String,
    pub spec: SliderSpec,
    pub setting_key: SettingKey,
}

/// Widget adapter for sliders.
pub struct SliderWidget;

impl Widget for SliderWidget {
    type Spec = SliderWidgetSpec;

    fn spawn(commands: &mut Commands, theme: &Theme, spec: Self::Spec, parent: Entity) -> Entity {
        spawn_slider(
            commands,
            theme,
            &spec.label,
            spec.spec,
            spec.setting_key,
            parent,
        )
    }
}

/// Helper to spawn a slider widget.
pub fn spawn_slider(
    commands: &mut Commands,
    theme: &Theme,
    label: &str,
    spec: SliderSpec,
    setting_key: SettingKey,
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
                setting_key,
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

/// System to handle slider interaction and update UserSettings.
#[allow(clippy::too_many_arguments)]
pub fn slider_interaction_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    manifest: Res<crate::framework::loading::assets::AssetManifest>,
    mut cache: ResMut<AssetCache>,
    runtime_audio: Res<crate::framework::settings::RuntimeAudioState>,
    mut interaction_query: Query<(&Interaction, &GlobalTransform, &ComputedNode, &mut Slider)>,
    mut settings: ResMut<crate::config::UserSettings>,
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
        if *interaction != Interaction::Pressed {
            continue;
        }

        let width = computed.size().x;
        if width <= f32::EPSILON {
            warn!("[Slider] Invalid slider width: {}", width);
            continue;
        }

        let node_pos = transform.translation().truncate();
        let half_width = width / 2.0;
        let min_x = node_pos.x - half_width;

        let relative_x = ((mouse_pos.x - min_x) / width).clamp(0.0, 1.0);

        let new_value =
            (slider.min + (slider.max - slider.min) * relative_x).clamp(slider.min, slider.max);

        // Extra validation for volume settings
        let new_value = match slider.setting_key {
            SettingKey::MasterVolume | SettingKey::MusicVolume | SettingKey::SfxVolume => {
                new_value.clamp(0.0, 1.0)
            }
            _ => new_value,
        };

        // Only update and play sound if value changed significantly
        let delta = (slider.value - new_value).abs();
        slider.value = new_value;

        // Play subtle scroll sound on value change
        if delta >= 0.001 {
            play_ui_audio(
                UiAudioEvent::Scroll,
                runtime_audio.sfx * 0.7,
                &mut commands,
                &mut cache,
                &asset_server,
                &manifest,
            );
        }
        // Apply to settings with validation
        apply_slider_setting(&slider.setting_key, new_value, &mut settings);
    }
}

/// Applies a slider value to the corresponding user setting with validation and logging.
fn apply_slider_setting(key: &SettingKey, value: f32, settings: &mut crate::config::UserSettings) {
    match key {
        SettingKey::MasterVolume => {
            settings.audio.master_volume = value.clamp(0.0, 1.0);
            info!("[Settings] Master volume: {:.2}", value);
        }
        SettingKey::MusicVolume => {
            settings.audio.music_volume = value.clamp(0.0, 1.0);
            info!("[Settings] Music volume: {:.2}", value);
        }
        SettingKey::SfxVolume => {
            settings.audio.sfx_volume = value.clamp(0.0, 1.0);
            info!("[Settings] SFX volume: {:.2}", value);
        }
        _ => warn!("[Settings] Unknown slider key: {:?}", key),
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
