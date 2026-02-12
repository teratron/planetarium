use crate::framework::assets::AssetManifest;
use crate::framework::states::AppState;
use bevy::prelude::*;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Splash), setup_splash)
            .add_systems(Update, countdown_splash.run_if(in_state(AppState::Splash)))
            .add_systems(OnExit(AppState::Splash), cleanup_splash);
    }
}

/// Resource to track the splash screen timer.
#[derive(Resource)]
struct SplashTimer(Timer);

/// Marker component for the splash screen root node.
#[derive(Component)]
struct SplashRoot;

fn setup_splash(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    manifest: Res<AssetManifest>,
    mut fade: ResMut<crate::framework::ui::fading::ScreenFade>,
) {
    info!("[SplashPlugin] Showing splash screen...");

    // Start fading in
    fade.fade_in(1.0);

    // Default to a 2.0 second timer
    commands.insert_resource(SplashTimer(Timer::from_seconds(2.0, TimerMode::Once)));

    // Try to get the logo path from the manifest, fallback if missing
    let logo_path = manifest
        .branding("splash_logo")
        .cloned()
        .unwrap_or_else(|| "branding/logo.png".to_string());

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::BLACK),
            SplashRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageNode {
                    image: asset_server.load(logo_path),
                    ..default()
                },
                Node::default(),
            ));
        });
}

fn countdown_splash(
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut fade: ResMut<crate::framework::ui::fading::ScreenFade>,
) {
    timer.0.tick(time.delta());

    // Check if timer finished naturally
    if timer.0.just_finished() {
        info!("[SplashPlugin] Splash finished. Fading out...");
        fade.fade_out(0.5, AppState::MainMenu);
        return;
    }

    // Skip logic: Allow skip after 1.0 seconds (as per plan)
    if timer.0.elapsed_secs() > 1.0 {
        let skip_input = keys.any_just_pressed([KeyCode::Space, KeyCode::Enter, KeyCode::Escape])
            || mouse.any_just_pressed([MouseButton::Left, MouseButton::Right]);

        if skip_input {
            info!("[SplashPlugin] Splash skipped by user. Fading out...");
            fade.fade_out(0.3, AppState::MainMenu); // Faster fade on skip
        }
    }
}

fn cleanup_splash(mut commands: Commands, query: Query<Entity, With<SplashRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<SplashTimer>();
}
