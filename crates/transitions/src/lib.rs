//! # Screen Fading System
//!
//! Provides a resource to manage global screen transitions (Fade In / Fade Out).

use bevy::prelude::*;
use bevy::state::state::FreelyMutableState;
use std::marker::PhantomData;

/// Plugin for screen fading transitions.
/// Generic over the State type `S` to allow state transitions.
pub struct FadingPlugin<S: States + FreelyMutableState>(PhantomData<S>);

impl<S: States + FreelyMutableState> Default for FadingPlugin<S> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<S: States + FreelyMutableState> Plugin for FadingPlugin<S> {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScreenFade<S>>()
            .add_systems(Startup, setup_fade_overlay)
            .add_systems(Update, update_fade_system::<S>);
    }
}

/// State of the fading animation.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum FadeState {
    #[default]
    Idle,
    FadingIn,              // Black -> Transparent
    FadingOut,             // Transparent -> Black
    WaitingForStateChange, // Waiting for AppState transition before fading in
}

/// Resource to control screen transitions.
#[derive(Resource)]
pub struct ScreenFade<S: States + FreelyMutableState> {
    pub state: FadeState,
    pub timer: Timer,
    pub next_app_state: Option<S>,
    alpha: f32,
}

impl<S: States + FreelyMutableState> Default for ScreenFade<S> {
    fn default() -> Self {
        Self {
            state: FadeState::Idle,
            timer: Timer::from_seconds(0.5, TimerMode::Once),
            next_app_state: None,
            alpha: 0.0,
        }
    }
}

impl<S: States + FreelyMutableState> ScreenFade<S> {
    /// Start fading out to black. Upon completion, switch to `target_state`.
    pub fn fade_out(&mut self, duration: f32, target_state: S) {
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

fn update_fade_system<S: States + FreelyMutableState>(
    mut fade: ResMut<ScreenFade<S>>,
    time: Res<Time>,
    current_state: Res<State<S>>,
    mut next_state: ResMut<NextState<S>>,
    mut query: Query<&mut BackgroundColor, With<FadeOverlay>>,
) {
    if fade.state == FadeState::Idle {
        return;
    }

    // Tick timer only for animation states (not WaitingForStateChange)
    if fade.state != FadeState::WaitingForStateChange {
        fade.timer.tick(time.delta());
    }

    match fade.state {
        FadeState::FadingIn => {
            // 1.0 -> 0.0
            fade.alpha = 1.0 - fade.timer.fraction();
            if fade.timer.just_finished() {
                fade.state = FadeState::Idle;
                fade.alpha = 0.0;
            }
        }
        FadeState::FadingOut => {
            // 0.0 -> 1.0
            fade.alpha = fade.timer.fraction();
            if fade.timer.just_finished() {
                fade.alpha = 1.0;

                // Request state change if configured
                if let Some(target) = fade.next_app_state.clone() {
                    info!("[FadingPlugin] Requesting state transition to {:?}", target);
                    next_state.set(target);
                    // Wait for the state to actually change before fading in
                    fade.state = FadeState::WaitingForStateChange;
                } else {
                    fade.state = FadeState::Idle;
                }
            }
        }
        FadeState::WaitingForStateChange => {
            // Wait for the new state to become active before starting fade-in
            if let Some(target) = &fade.next_app_state
                && current_state.get() == target
            {
                info!(
                    "[FadingPlugin] State changed to {:?}, starting fade in",
                    target
                );
                fade.state = FadeState::FadingIn;
                fade.timer = Timer::from_seconds(0.5, TimerMode::Once);
                fade.timer.reset();
                fade.next_app_state = None;
                // Alpha is already 1.0, perfect for FadingIn start
            }
        }
        FadeState::Idle => {}
    }

    // Update overlay color
    for mut bg in query.iter_mut() {
        bg.0 = Color::BLACK.with_alpha(fade.alpha);
    }
}
