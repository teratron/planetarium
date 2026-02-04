//! Stage 4: Loading orchestration.
//! Handles asynchronous asset loading and progress tracking.

use crate::core::states::AppState;
use crate::ui::theme::Theme;
use bevy::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadingTracker>()
            .add_systems(OnEnter(AppState::Loading), setup_loading_screen)
            .add_systems(
                Update,
                (
                    update_loading_progress,
                    update_loading_ui,
                    rotate_loading_hints,
                )
                    .run_if(in_state(AppState::Loading)),
            )
            .add_systems(OnExit(AppState::Loading), cleanup_loading_screen);
    }
}

/// Resource to track the current loading progress and display hints.
#[derive(Resource)]
struct LoadingTracker {
    /// 0.0 to 1.0 progress.
    pub progress: f32,
    /// Index of the currently displayed hint.
    pub current_hint_index: usize,
    /// Timer to rotate hints.
    pub hint_timer: Timer,
}

impl Default for LoadingTracker {
    fn default() -> Self {
        Self {
            progress: 0.0,
            current_hint_index: 0,
            hint_timer: Timer::from_seconds(4.0, TimerMode::Repeating),
        }
    }
}

/// Marker for the loading screen root node.
#[derive(Component)]
struct LoadingRoot;

/// Marker for the bar fill entity to update its width.
#[derive(Component)]
struct ProgressBarFill;

/// Marker for the text entity to update loading hints.
#[derive(Component)]
struct LoadingHintText;

/// Initial loading hints to display.
const LOADING_HINTS: &[&str] = &[
    "Scanning local star clusters...",
    "Calibrating planetary gravity models...",
    "Warming up fusion core reactors...",
    "Synchronizing orbital trajectories...",
    "Optimizing light-speed navigation...",
];

fn setup_loading_screen(mut commands: Commands, theme: Res<Theme>) {
    info!("[LoadingUI] Spawning loading screen...");

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
                Text::new("LOADING CONTENT"),
                TextFont {
                    font: theme.fonts.bold.clone(),
                    font_size: theme.sizes.font_h2,
                    ..default()
                },
                TextColor(theme.colors.accent),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
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
                Text::new(LOADING_HINTS[0]),
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

fn update_loading_progress(
    time: Res<Time>,
    mut tracker: ResMut<LoadingTracker>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    // Mock loading logic: linearly increase progress over 3 seconds
    tracker.progress += time.delta_secs() / 3.0;

    if tracker.progress >= 1.0 {
        tracker.progress = 1.0;
        info!("[LoadingUI] Content loaded. Transitioning to InGame.");
        next_state.set(AppState::InGame);
    }
}

fn update_loading_ui(
    tracker: Res<LoadingTracker>,
    mut fill_query: Query<&mut Node, With<ProgressBarFill>>,
) {
    for mut node in &mut fill_query {
        node.width = Val::Percent(tracker.progress * 100.0);
    }
}

fn rotate_loading_hints(
    time: Res<Time>,
    mut tracker: ResMut<LoadingTracker>,
    mut text_query: Query<&mut Text, With<LoadingHintText>>,
) {
    if tracker.hint_timer.tick(time.delta()).just_finished() {
        tracker.current_hint_index = (tracker.current_hint_index + 1) % LOADING_HINTS.len();
        for mut text in &mut text_query {
            text.0 = LOADING_HINTS[tracker.current_hint_index].to_string();
        }
    }
}

fn cleanup_loading_screen(
    mut commands: Commands,
    query: Query<Entity, With<LoadingRoot>>,
    mut tracker: ResMut<LoadingTracker>,
) {
    info!("[LoadingUI] Cleaning up loading screen.");
    for entity in &query {
        commands.entity(entity).despawn();
    }

    // Reset tracker for next reuse
    *tracker = LoadingTracker::default();
}
