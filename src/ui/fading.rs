//! # Screen Fading System
//!
//! Provides a resource to manage global screen transitions (Fade In / Fade Out).

use crate::core::states::AppState;
use bevy::prelude::*;

pub struct FadingPlugin;

impl Plugin for FadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScreenFade>()
            .add_systems(Startup, setup_fade_overlay)
            .add_systems(Update, update_fade_system);
    }
}

/// State of the fading animation.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum FadeState {
    #[default]
    Idle,
    FadingIn,  // Black -> Transparent
    FadingOut, // Transparent -> Black
}

/// Resource to control screen transitions.
#[derive(Resource)]
pub struct ScreenFade {
    pub state: FadeState,
    pub timer: Timer,
    pub next_app_state: Option<AppState>,
    alpha: f32,
}

impl Default for ScreenFade {
    fn default() -> Self {
        Self {
            state: FadeState::Idle,
            timer: Timer::from_seconds(0.5, TimerMode::Once),
            next_app_state: None,
            alpha: 0.0,
        }
    }
}

impl ScreenFade {
    /// Start fading out to black. Upon completion, switch to `target_state`.
    pub fn fade_out(&mut self, duration: f32, target_state: AppState) {
        self.state = FadeState::FadingOut;
        self.timer = Timer::from_seconds(duration, TimerMode::Once);
        self.timer.reset();
        self.next_app_state = Some(target_state);
        self.alpha = 0.0;
    }

    /// Start fading in from black.
    pub fn fade_in(&mut self, duration: f32) {
        self.state = FadeState::FadingIn;
        self.timer = Timer::from_seconds(duration, TimerMode::Once);
        self.timer.reset();
        self.next_app_state = None;
        self.alpha = 1.0;
    }
}

/// Tag component for the UI overlay entity.
#[derive(Component)]
struct FadeOverlay;

fn setup_fade_overlay(mut commands: Commands) {
    info!("[FadingPlugin] Spawning fade overlay...");
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                ..default()
            },
            BackgroundColor(Color::BLACK.with_alpha(0.0)),
            ZIndex(100), // Ensure it's on top of everything
            FadeOverlay,
        ))
        .insert(Pickable::IGNORE); // Let clicks pass through if transparent
}

fn update_fade_system(
    mut fade: ResMut<ScreenFade>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<AppState>>,
    mut query: Query<&mut BackgroundColor, With<FadeOverlay>>,
) {
    if fade.state == FadeState::Idle {
        return;
    }

    fade.timer.tick(time.delta());
    let percent = fade.timer.fraction(); // 0.0 to 1.0

    match fade.state {
        FadeState::FadingIn => {
            // 1.0 -> 0.0
            fade.alpha = 1.0 - percent;
            if fade.timer.just_finished() {
                fade.state = FadeState::Idle;
                fade.alpha = 0.0;
            }
        }
        FadeState::FadingOut => {
            // 0.0 -> 1.0
            fade.alpha = percent;
            if fade.timer.just_finished() {
                fade.state = FadeState::Idle;
                fade.alpha = 1.0;

                // Trigger state change if requested
                if let Some(target) = fade.next_app_state {
                    info!("[FadingPlugin] Transitioning state to {:?}", target);
                    next_state.set(target);
                    // Automatically prepare to fade in on the new state?
                    // Usually better to let the new state trigger fade_in,
                    // or auto-trigger it here. Let's auto-reset to In for continuity.
                    fade.state = FadeState::FadingIn;
                    fade.timer.reset();
                    // We keep next_app_state as None for the In phase
                    fade.next_app_state = None;
                    // Alpha is already 1.0, perfect for FadingIn start
                }
            }
        }
        FadeState::Idle => {}
    }

    // Update overlay color
    for mut bg in query.iter_mut() {
        bg.0 = Color::BLACK.with_alpha(fade.alpha);
    }
}
