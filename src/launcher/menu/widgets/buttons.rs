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
        (&Interaction, &mut HoverAnimationState),
        (
            Changed<Interaction>,
            With<Button>,
            Or<(With<PrimaryButton>, With<super::components::Slider>)>,
        ),
    >,
) {
    for (interaction, mut anim_state) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
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
                anim_state.is_hovered = false;
            }
        }
    }
}

/// System to handle hover animations for buttons (scale and color)
pub fn animate_button_hover(
    mut query: Query<(
        &mut Transform,
        &mut BackgroundColor,
        &HoverAnimationState,
        &ButtonHoverState,
    )>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    // Fast interpolation speed for snappy but smooth feel (approx 0.2s full transition)
    let lerp_speed = 15.0;
    let t = (lerp_speed * dt).min(1.0);

    for (mut transform, mut bg, anim_state, color_state) in &mut query {
        // --- Scale Animation ---
        let target_scale = if anim_state.is_hovered {
            anim_state.target_scale
        } else {
            anim_state.base_scale
        };

        transform.scale = transform.scale.lerp(target_scale, t);

        // --- Color Animation ---
        let target_color = if anim_state.is_hovered {
            color_state.hover_color
        } else {
            color_state.base_color
        };

        let current: LinearRgba = bg.0.into();
        let target: LinearRgba = target_color.into();

        // Manual lerp for Color (safe and explicit)
        let r = current.red + (target.red - current.red) * t;
        let g = current.green + (target.green - current.green) * t;
        let b = current.blue + (target.blue - current.blue) * t;
        let a = current.alpha + (target.alpha - current.alpha) * t;

        bg.0 = Color::LinearRgba(LinearRgba::new(r, g, b, a));
    }
}
