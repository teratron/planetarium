use bevy::prelude::*;
use std::time::Duration;
use crate::states::GameState;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Splash), setup_splash)
            .add_systems(
                Update,
                (splash_timer_tick, splash_skip_input).run_if(in_state(GameState::Splash)),
            )
            .add_systems(OnExit(GameState::Splash), cleanup_splash);
    }
}

#[derive(Component)]
struct SplashScreen;

#[derive(Component)]
struct SplashLogo;

#[derive(Resource)]
struct SplashTimer {
    timer: Timer,
    can_skip: bool,
}

fn setup_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Splash screen container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::BLACK),
            SplashScreen,
        ))
        .with_children(|parent| {
            // Logo image
            parent.spawn((
                ImageNode::new(asset_server.load("textures/logo.png")),
                Node {
                    width: Val::Px(300.0),
                    height: Val::Px(300.0),
                    ..default()
                },
                SplashLogo,
            ));
            
            // Loading indicator text
            parent.spawn((
                Text::new("Loading..."),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE.with_alpha(0.7)),
                Node {
                    margin: UiRect::top(Val::Px(40.0)),
                    ..default()
                },
            ));
        });

    // Auto-transition timer (3 seconds, skippable after 1 second)
    commands.insert_resource(SplashTimer {
        timer: Timer::new(Duration::from_secs(3), TimerMode::Once),
        can_skip: false,
    });
    
    info!("Splash screen started");
}

fn splash_timer_tick(
    time: Res<Time>,
    mut splash_timer: ResMut<SplashTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    splash_timer.timer.tick(time.delta());
    
    // Allow skipping after 1 second
    if splash_timer.timer.elapsed_secs() > 1.0 {
        splash_timer.can_skip = true;
    }
    
    // Auto-transition when finished
    if splash_timer.timer.finished() {
        info!("Splash timer finished, transitioning to MainMenu");
        next_state.set(GameState::MainMenu);
    }
}

fn splash_skip_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    splash_timer: Res<SplashTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !splash_timer.can_skip {
        return;
    }
    
    let skip_pressed = keyboard.just_pressed(KeyCode::Space)
        || keyboard.just_pressed(KeyCode::Enter)
        || keyboard.just_pressed(KeyCode::Escape)
        || mouse.just_pressed(MouseButton::Left);
    
    if skip_pressed {
        info!("Splash skipped by user input");
        next_state.set(GameState::MainMenu);
    }
}

fn cleanup_splash(mut commands: Commands, query: Query<Entity, With<SplashScreen>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
    commands.remove_resource::<SplashTimer>();
    info!("Splash screen cleaned up");
}
