//! # Diagnostics & Debug Overlay
//!
//! Provides a real-time overlay for monitoring performance (FPS) and application state.

use crate::core::states::AppState;
use crate::ui::theme::Theme;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

/// Plugin that provides real-time engine diagnostics and performance monitoring.
///
/// It registers systems for an on-screen overlay (FPS, State, Entity count)
/// that can be toggled using the F1 key.
pub struct DiagnosticsPlugin;

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .init_resource::<DebugSettings>()
            .add_systems(Startup, setup_debug_overlay)
            .add_systems(Update, (toggle_debug_overlay, update_debug_text));
    }
}

#[derive(Resource)]
struct DebugSettings {
    pub visible: bool,
}

impl Default for DebugSettings {
    fn default() -> Self {
        Self { visible: false } // Hidden by default
    }
}

/// Marker for the debug overlay root.
#[derive(Component)]
struct DebugOverlayRoot;

/// Marker for the FPS text.
#[derive(Component)]
struct FpsText;

/// Marker for the State text.
#[derive(Component)]
struct StateText;

/// Marker for the Entities count text.
#[derive(Component)]
struct EntityText;

fn setup_debug_overlay(mut commands: Commands, theme: Res<Theme>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)), // Semi-transparent black
            ZIndex(100),                                       // Always on top
            DebugOverlayRoot,
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            // FPS Label
            parent.spawn((
                Text::new("FPS: 00"),
                TextFont {
                    font: theme.fonts.main.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 1.0, 0.0)), // Green
                FpsText,
            ));

            // AppState Label
            parent.spawn((
                Text::new("STATE: BOOTING"),
                TextFont {
                    font: theme.fonts.main.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(theme.colors.accent),
                StateText,
            ));

            // Entity Count Label
            parent.spawn((
                Text::new("ENTITIES: 0"),
                TextFont {
                    font: theme.fonts.main.clone(),
                    font_size: 12.0,
                    ..default()
                },
                TextColor(theme.colors.text_secondary),
                EntityText,
            ));

            parent.spawn((
                Text::new("Press F1 to Toggle"),
                TextFont {
                    font: theme.fonts.main.clone(),
                    font_size: 10.0,
                    ..default()
                },
                TextColor(theme.colors.text_secondary),
            ));
        });
}

fn toggle_debug_overlay(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<DebugSettings>,
    mut query: Query<&mut Visibility, With<DebugOverlayRoot>>,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        settings.visible = !settings.visible;
        for mut visibility in &mut query {
            *visibility = if settings.visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
        info!("[Diagnostics] Debug overlay toggled: {}", settings.visible);
    }
}

/// Updates the debug overlay text with real-time performance and engine state data.
fn update_debug_text(
    diagnostics: Res<DiagnosticsStore>,
    current_state: Res<State<AppState>>,
    entities: Query<Entity>,
    mut fps_query: Query<&mut Text, (With<FpsText>, Without<StateText>, Without<EntityText>)>,
    mut state_query: Query<&mut Text, (With<StateText>, Without<FpsText>, Without<EntityText>)>,
    mut entity_query: Query<&mut Text, (With<EntityText>, Without<FpsText>, Without<StateText>)>,
) {
    // Update FPS
    if let Some(fps) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|diag| diag.smoothed())
    {
        for mut text in &mut fps_query {
            text.0 = format!("FPS: {:.0}", fps);
        }
    }

    // Update State
    for mut text in &mut state_query {
        text.0 = format!("STATE: {:?}", current_state.get()).to_uppercase();
    }

    // Update Entity Count
    let entity_count = entities.iter().count();
    for mut text in &mut entity_query {
        text.0 = format!("ENTITIES: {}", entity_count);
    }
}
