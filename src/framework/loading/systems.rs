//! # Loading Screen Systems
//!
//! Systems for setup, progress tracking, UI updates, hint rotation,
//! and cleanup of the loading screen.

use bevy::prelude::*;

use crate::framework::states::AppState;
use crate::framework::ui::theme::Theme;

use super::components::*;
use super::resources::{AssetLoadingState, LoadingTracker};

/// Initial loading hints to display.
/// Initial loading hints to display.
const LOADING_HINT_KEYS: &[&str] = &[
    "hint-scan-clusters",
    "hint-calibrate-gravity",
    "hint-warm-reactors",
    "hint-sync-trajectories",
    "hint-opt-nav",
];

/// Resets the loading tracker when entering the Loading state.
/// This ensures the `completed_logged` flag is reset for new loading cycles.
pub fn reset_loading_tracker(
    mut tracker: ResMut<LoadingTracker>,
    localization: Res<crate::framework::localization::Localization>,
) {
    *tracker = LoadingTracker::default();
    info!("{}", localization.t("log-loading-reset"));
}

/// Spawns the complex loading screen UI.
/// Includes a title, current asset group info, numerical percentage, progress bar, and lore hints.
pub fn setup_loading_screen(
    mut commands: Commands,
    theme: Res<Theme>,
    localization: Res<crate::framework::localization::Localization>,
) {
    info!("{}", localization.t("log-loading-spawn"));

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
            LoadingRoot,
        ))
        .with_children(|parent| {
            // "LOADING" Title
            parent.spawn((
                Text::new(localization.t("ui-loading-title")),
                TextFont {
                    font: theme.fonts.bold.clone(),
                    font_size: theme.sizes.font_h2,
                    ..default()
                },
                TextColor(theme.colors.accent),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            // ACTIVE ASSET GROUP
            parent.spawn((
                Text::new(localization.t("ui-loading-init")),
                TextFont {
                    font: theme.fonts.main.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(theme.colors.text_secondary),
                Node {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
                LoadingAssetText,
            ));

            // PERCENTAGE
            parent.spawn((
                Text::new("0%"),
                TextFont {
                    font: theme.fonts.bold.clone(),
                    font_size: 18.0,
                    ..default()
                },
                TextColor(theme.colors.text_primary),
                Node {
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                },
                LoadingPercentText,
            ));

            // Progress Bar Container (Track)
            parent
                .spawn((
                    Node {
                        width: Val::Px(400.0),
                        height: Val::Px(10.0),
                        ..default()
                    },
                    BackgroundColor(theme.colors.surface),
                ))
                .with_children(|bar| {
                    // Progress Fill
                    bar.spawn((
                        Node {
                            width: Val::Percent(0.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(theme.colors.accent),
                        ProgressBarFill,
                    ));
                });

            // Loading Hint Text
            parent.spawn((
                Text::new(localization.t(LOADING_HINT_KEYS[0])),
                TextFont {
                    font: theme.fonts.main.clone(),
                    font_size: theme.sizes.font_body,
                    ..default()
                },
                TextColor(theme.colors.text_secondary),
                Node {
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
                LoadingHintText,
            ));
        });
}

/// Tracks real asset loading progress via `AssetServer`.
///
/// When `AssetLoadingState.required_assets` is empty the loading screen
/// progresses instantly (no assets registered yet).
pub fn update_loading_progress(
    asset_server: Res<AssetServer>,
    mut tracker: ResMut<LoadingTracker>,
    mut loading_state: ResMut<AssetLoadingState>,
    mut fade: ResMut<crate::framework::ui::fading::ScreenFade>,
    localization: Res<crate::framework::localization::Localization>,
) {
    let total = loading_state.required_assets.len();

    if total == 0 {
        // No registered assets — complete immediately
        tracker.progress = 1.0;
    } else {
        let mut loaded = 0usize;
        for handle in &loading_state.required_assets {
            match asset_server.get_load_state(handle.id()) {
                Some(bevy::asset::LoadState::Loaded) => loaded += 1,
                Some(bevy::asset::LoadState::Failed(_)) => {
                    let mut args = fluent_bundle::FluentArgs::new();
                    args.set("asset", format!("{:?}", handle));
                    error!(
                        "{}",
                        localization.t_with_args("log-loading-failed", Some(&args))
                    );
                    loaded += 1; // Count as "done" to avoid stalling
                }
                _ => {} // Still loading
            }
        }
        loading_state.loaded_count = loaded;
        loading_state.total_count = total;
        tracker.progress = loaded as f32 / total.max(1) as f32;
    }

    if tracker.progress >= 1.0 {
        tracker.progress = 1.0;
        if !tracker.completed_logged {
            info!("{}", localization.t("log-loading-complete"));
            fade.fade_out(0.5, AppState::InGame);
            tracker.completed_logged = true;
        }
    }
}

/// Updates the loading UI elements (bar, percentage, asset info) based on the current progress.
pub fn update_loading_ui(
    tracker: Res<LoadingTracker>,
    mut fill_query: Query<&mut Node, With<ProgressBarFill>>,
    mut percent_query: Query<&mut Text, (With<LoadingPercentText>, Without<LoadingAssetText>)>,
    mut asset_query: Query<&mut Text, (With<LoadingAssetText>, Without<LoadingPercentText>)>,
    localization: Res<crate::framework::localization::Localization>,
) {
    // 1. Update Bar
    for mut node in &mut fill_query {
        node.width = Val::Percent(tracker.progress * 100.0);
    }

    // 2. Update Numerical Percentage
    let percent = (tracker.progress * 100.0) as u32;
    for mut text in &mut percent_query {
        text.0 = format!("{}%", percent);
    }

    // 3. Update active asset group (Simulated feedback)
    let info_key = match tracker.progress {
        p if p < 0.2 => "info-loading-engine",
        p if p < 0.4 => "info-loading-stars",
        p if p < 0.6 => "info-loading-textures",
        p if p < 0.8 => "info-loading-models",
        _ => "info-loading-finalizing",
    };
    for mut text in &mut asset_query {
        text.0 = localization.t(info_key);
    }
}

pub fn rotate_loading_hints(
    time: Res<Time>,
    mut tracker: ResMut<LoadingTracker>,
    mut text_query: Query<&mut Text, With<LoadingHintText>>,
    localization: Res<crate::framework::localization::Localization>,
) {
    if tracker.hint_timer.tick(time.delta()).just_finished() {
        tracker.current_hint_index = (tracker.current_hint_index + 1) % LOADING_HINT_KEYS.len();
        let hint = localization.t(LOADING_HINT_KEYS[tracker.current_hint_index]);
        for mut text in &mut text_query {
            text.0 = hint.clone();
        }
    }
}

pub fn cleanup_loading_screen(
    mut commands: Commands,
    query: Query<Entity, With<LoadingRoot>>,
    mut tracker: ResMut<LoadingTracker>,
    localization: Res<crate::framework::localization::Localization>,
) {
    info!("{}", localization.t("log-loading-cleanup"));
    for entity in &query {
        commands.entity(entity).despawn();
    }

    // Reset tracker for next reuse
    *tracker = LoadingTracker::default();
}
