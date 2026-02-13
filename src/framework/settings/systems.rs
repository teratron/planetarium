//! Settings UI systems.
//!
//! Systems for interaction handling, content hydration, fade animation, and UI sync.

use super::components::{
    FadeDirection, MasterVolumeControl, MusicVolumeControl, SFXVolumeControl, SettingsContentArea,
    SettingsFade, SettingsRoot, SettingsTabButton,
};
use super::tabs;
use super::{ActiveSettingsTab, SettingsOpen, SettingsTab};
use crate::config::UserSettings;
use crate::framework::localization::{Localization, LocalizedStrings};
use crate::framework::ui::theme::Theme;
use crate::framework::ui::theme::constants;
use bevy::prelude::*;

/// System to handle tab switching.
#[allow(clippy::type_complexity)]
pub fn handle_settings_tab_clicks(
    mut tab_query: Query<(&Interaction, &SettingsTabButton), (Changed<Interaction>, With<Button>)>,
    mut active_tab: ResMut<ActiveSettingsTab>,
) {
    for (interaction, tab_btn) in &mut tab_query {
        if *interaction == Interaction::Pressed && active_tab.0 != tab_btn.0 {
            info!("[Settings] Switching to tab: {:?}", tab_btn.0);
            active_tab.0 = tab_btn.0;
        }
    }
}

/// System to hydrate/refresh the content area when tab changes.
#[allow(clippy::too_many_arguments)]
pub fn update_settings_tab_content(
    mut commands: Commands,
    active_tab: Res<ActiveSettingsTab>,
    theme: Res<Theme>,
    loc: Res<Localization>,
    mut strings: ResMut<LocalizedStrings>,
    settings: Res<UserSettings>,
    content_area_query: Query<Entity, With<SettingsContentArea>>,
    children_query: Query<&Children>,
) {
    if !active_tab.is_changed() {
        return;
    }

    if let Ok(content_area) = content_area_query.single() {
        // 1. Clear existing content
        if let Ok(children) = children_query.get(content_area) {
            for child in children.iter() {
                commands.entity(child).despawn();
            }
        }

        // 2. Spawn new panel based on tab
        commands
            .entity(content_area)
            .with_children(|parent| match active_tab.0 {
                SettingsTab::Graphics => {
                    tabs::spawn_graphics_tab(parent, &theme, &loc, &mut strings, &settings)
                }
                SettingsTab::Audio => {
                    tabs::spawn_audio_tab(parent, &theme, &loc, &mut strings, &settings)
                }
                SettingsTab::Controls => {
                    tabs::spawn_controls_tab(parent, &theme, &loc, &mut strings, &settings)
                }
                SettingsTab::General => {
                    tabs::spawn_general_tab(parent, &theme, &loc, &mut strings, &settings)
                }
            });
    }
}

/// Spawns or despawns settings depending on `SettingsOpen` resource.
pub fn spawn_settings_if_needed(
    mut commands: Commands,
    theme: Res<Theme>,
    loc: Res<Localization>,
    mut strings: ResMut<LocalizedStrings>,
    settings_open: Res<SettingsOpen>,
    query: Query<Entity, With<SettingsRoot>>,
    mut active_tab: ResMut<ActiveSettingsTab>,
) {
    if settings_open.is_changed() {
        if settings_open.0 && query.is_empty() {
            // Reset tab to default when opening
            active_tab.0 = SettingsTab::default();
            super::ui::spawn_settings_menu(&mut commands, &theme, &loc, &mut strings);
        } else if !settings_open.0 && !query.is_empty() {
            // Trigger fade out instead of immediate despawn
            for e in &query {
                commands.entity(e).insert(SettingsFade {
                    timer: Timer::from_seconds(
                        constants::timing::SETTINGS_FADE_OUT,
                        TimerMode::Once,
                    ),
                    direction: FadeDirection::Out,
                });
            }
        }
    }
}

/// System to animate settings fade transition.
pub fn animate_settings_fade(
    mut commands: Commands,
    time: Res<Time>,
    theme: Res<Theme>,
    mut query: Query<(
        Entity,
        &mut BackgroundColor,
        &mut Transform,
        &mut SettingsFade,
    )>,
) {
    for (entity, mut bg_color, mut transform, mut fade) in &mut query {
        fade.timer.tick(time.delta());
        let progress = fade.timer.fraction(); // 0.0 to 1.0

        let (alpha, scale) = match fade.direction {
            FadeDirection::In => {
                // Ease out cubic
                let t = 1.0 - (1.0 - progress).powi(3);
                (t, 0.9 + 0.1 * t)
            }
            FadeDirection::Out => {
                // Ease in cubic
                let t = 1.0 - progress.powi(3);
                (t, 0.9 + 0.1 * t)
            }
        };

        // Update visual state
        *bg_color = BackgroundColor(
            theme
                .colors
                .surface
                .with_alpha(theme.colors.surface.alpha() * alpha),
        );
        transform.scale = Vec3::splat(scale);

        // Despawn on finish if fading out
        if fade.direction == FadeDirection::Out && fade.timer.just_finished() {
            commands.entity(entity).despawn();
        }

        // Remove Fade component if fading in finished
        if fade.direction == FadeDirection::In && fade.timer.just_finished() {
            commands.entity(entity).remove::<SettingsFade>();
            // Ensure final state
            *bg_color = BackgroundColor(theme.colors.surface);
            transform.scale = Vec3::ONE;
        }
    }
}

/// Update UI display values to match current UserSettings.
#[allow(clippy::type_complexity)]
pub fn update_settings_ui(
    settings: Res<UserSettings>,
    mut queries: ParamSet<(
        Query<&mut Text, With<MasterVolumeControl>>,
        Query<&mut Text, With<MusicVolumeControl>>,
        Query<&mut Text, With<SFXVolumeControl>>,
    )>,
) {
    if !settings.is_changed() {
        return;
    }

    if let Ok(mut text) = queries.p0().single_mut() {
        text.0 = format!("{:.2}", settings.audio.master_volume);
    }
    if let Ok(mut text) = queries.p1().single_mut() {
        text.0 = format!("{:.2}", settings.audio.music_volume);
    }
    if let Ok(mut text) = queries.p2().single_mut() {
        text.0 = format!("{:.2}", settings.audio.sfx_volume);
    }
}
