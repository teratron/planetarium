//! Button Widget
//!
//! Provides primary button widget creation and interaction handling.

use crate::core::assets::AssetManifest;
use crate::ui::theme::Theme;
use bevy::prelude::*;

use super::components::{ButtonHoverState, PrimaryButton};

/// Component to track button hover animation state
#[derive(Component)]
pub struct HoverAnimationState {
    pub base_scale: Vec3,
    pub target_scale: Vec3,
    pub current_scale: Vec3,
    pub is_hovered: bool,
}

/// Spawns a high-quality primary button widget.
pub fn spawn_primary_button(
    commands: &mut Commands,
    theme: &Theme,
    label: &str,
    action: super::components::ButtonAction,
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
            Transform::from_scale(Vec3::ONE),
            HoverAnimationState {
                base_scale: Vec3::ONE,
                target_scale: Vec3::splat(1.05),
                current_scale: Vec3::ONE,
                is_hovered: false,
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

/// System to handle button interactions, updating visual state and playing audio feedback.
#[allow(clippy::type_complexity)]
pub fn button_interaction_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    manifest: Res<AssetManifest>,
    audio_state: Res<crate::launcher::menu::reactive::RuntimeAudioState>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonHoverState,
            &mut HoverAnimationState,
        ),
        (
            Changed<Interaction>,
            With<Button>,
            Or<(With<PrimaryButton>, With<super::components::Slider>)>,
        ),
    >,
) {
    for (interaction, mut bg_color, hover_state, mut anim_state) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *bg_color = BackgroundColor(hover_state.hover_color);
                anim_state.is_hovered = true;

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
                anim_state.is_hovered = false;
            }
        }
    }
}

/// System to handle hover animations for buttons
pub fn animate_button_hover(
    mut query: Query<(&mut Transform, &HoverAnimationState)>,
    time: Res<Time>,
) {
    for (mut transform, anim_state) in &mut query {
        // Define interpolation speed
        let lerp_speed = 0.25; // Time in seconds for full transition

        // Calculate target scale based on hover state
        let target_scale = if anim_state.is_hovered {
            anim_state.target_scale
        } else {
            anim_state.base_scale
        };

        // Linear interpolation between current and target scale
        let new_scale = transform
            .scale
            .lerp(target_scale, lerp_speed * time.delta_secs());

        // Ensure we don't go past the target
        if (transform.scale - target_scale).length() > 0.01 {
            transform.scale = new_scale;
        } else {
            transform.scale = target_scale;
        }
    }
}
