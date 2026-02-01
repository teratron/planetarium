use bevy::prelude::*;
use bevy::asset::LoadState;
use std::time::Duration;
use crate::states::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), setup_loading)
            .add_systems(
                Update,
                (update_loading_progress, check_loading_complete)
                    .run_if(in_state(GameState::Loading)),
            )
            .add_systems(OnExit(GameState::Loading), cleanup_loading);
    }
}

#[derive(Component)]
struct LoadingScreen;

#[derive(Component)]
struct LoadingProgressBar;

#[derive(Component)]
struct LoadingProgressText;

#[derive(Resource)]
struct LoadingAssets {
    handles: Vec<UntypedHandle>,
    total_count: usize,
    min_display_timer: Timer,
}

fn setup_loading(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // UI
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
            BackgroundColor(Color::BLACK),
            LoadingScreen,
        ))
        .with_children(|parent| {
            // Loading text
            parent.spawn((
                Text::new("Loading..."),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                LoadingProgressText,
            ));

            // Progress bar container
            parent
                .spawn(Node {
                    width: Val::Percent(60.0),
                    height: Val::Px(20.0),
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                })
                .with_children(|bar_parent| {
                    // Progress fill
                    bar_parent.spawn((
                        Node {
                            width: Val::Percent(0.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.6, 0.9)),
                        LoadingProgressBar,
                    ));
                });
        });

    // Simulate loading with empty handles for now (instant load)
    let handles: Vec<UntypedHandle> = vec![];

    let total = handles.len();
    commands.insert_resource(LoadingAssets {
        handles,
        total_count: total,
        min_display_timer: Timer::new(Duration::from_secs(2), TimerMode::Once),
    });

    info!("Loading {} assets", total);
}

fn update_loading_progress(
    asset_server: Res<AssetServer>,
    loading: Res<LoadingAssets>,
    mut progress_bar: Query<&mut Node, With<LoadingProgressBar>>,
    mut progress_text: Query<&mut Text, With<LoadingProgressText>>,
) {
    let loaded = loading
        .handles
        .iter()
        .filter(|h| matches!(asset_server.get_load_state(h.id()), Some(LoadState::Loaded)))
        .count();

    let progress = if loading.total_count > 0 {
        loaded as f32 / loading.total_count as f32
    } else {
        1.0
    };

    // Update progress bar width
    if let Ok(mut bar) = progress_bar.single_mut() {
        bar.width = Val::Percent(progress * 100.0);
    }

    // Update text
    if let Ok(mut text) = progress_text.single_mut() {
        **text = format!("Loading... {}%", (progress * 100.0) as i32);
    }
}

fn check_loading_complete(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut loading: ResMut<LoadingAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Tick the minimum display timer
    loading.min_display_timer.tick(time.delta());
    
    // If we have no assets to load, simulate a delay or proceed immediately
    let all_loaded = if loading.handles.is_empty() {
        true
    } else {
        loading.handles.iter().all(|h| {
            matches!(
                asset_server.get_load_state(h.id()),
                Some(LoadState::Loaded) | Some(LoadState::Failed(_)) 
                // Treat failed as loaded for now to prevent getting stuck
            )
        })
    };

    // Only transition if assets are loaded AND minimum display time has elapsed
    if all_loaded && loading.min_display_timer.just_finished() {
        info!("All assets loaded, entering gameplay");
        next_state.set(GameState::Gameplay);
    }
}

fn cleanup_loading(mut commands: Commands, query: Query<Entity, With<LoadingScreen>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<LoadingAssets>();
    info!("Loading screen cleaned up");
}
