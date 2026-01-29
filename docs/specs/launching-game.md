# Launching Game

A professionally structured game launch sequence algorithm using **State Machine** and **Scene Management** architecture — the industry-standard approach adapted for Bevy game engine.

## 📋 Launch Sequence Overview

```plaintext
┌─────────────────────────────────────────────────────────────────────────────┐
│                         LAUNCH SEQUENCE FLOW                                │
├─────────────────────────────────────────────────────────────────────────────┤
│   Boot → Splash → [UpdateCheck] → MainMenu → Loading → Gameplay ↔ Paused    │
│                                       ↓                    ↑                │
│                                   Settings              Credits             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Phase 0: Engine Initialization (Pre-First Frame)

```plaintext
1. Executable launch
2. Core engine initialization (handled by Bevy's DefaultPlugins):
   - Graphics API (wgpu: Vulkan/DirectX/Metal/OpenGL)
   - Audio subsystem (via bevy_audio)
   - Input system (keyboard, mouse, gamepad, touch)
   - Asset server initialization
   - Window creation and configuration
3. Resource path resolution & config loading
4. Load global settings (from platform-specific locations)
5. Initialize analytics/crash reporting (non-blocking)
```

## Phase 1: Splash Screen (2–4 seconds)

```plaintext
Purpose: Branding + background preparation
Flow:
├─ Display engine/studio logos (sequential or overlapped)
├─ Background tasks (async, non-blocking):
│   • Preload minimal assets for main menu
│   • Verify critical file integrity
│   • Initialize audio system
│   • Warm up shader caches
└─ Auto-transition after timer or user skip input
```

> ⚠️ Never block rendering thread — heavy operations must be async!

## Phase 2: Content Validation / Update Check *(Optional)*

```plaintext
Condition: Skip for pure offline games; optional for hybrid titles
├─ Check server for latest version
├─ Compare local file hashes vs. manifests
├─ Download patches/DLC if needed:
│   • Show progress bar with cancel option
│   • Allow offline play if non-critical (graceful degradation)
├─ Validate license/account session (if required)
└─ Apply hotfixes/modifications
```

> 💡 For production: Integrate with platform store APIs (Steam, Epic, etc.)

## Phase 3: Main Menu / Home Screen

```plaintext
Trigger: After all critical systems are ready
UI Elements:
├─ [New Game] → scenario/mode selection → loading
├─ [Continue] → auto-load last session (if exists)
├─ [Load Game] → save slot browser with thumbnails & timestamps
├─ [Settings]
│   ├─ Graphics (resolution, quality presets, VSync, FPS cap)
│   ├─ Audio (master/music/SFX volumes)
│   ├─ Controls (key rebinding, sensitivity, invert Y)
│   ├─ Language & subtitles
│   └─ Accessibility (colorblind modes, UI scaling)
├─ [Extras] → Credits, Gallery, Statistics (optional)
└─ [Exit Game] → Confirmation dialog → graceful shutdown
```

## 🔁 Bevy State Machine Architecture

Bevy handles state transitions natively via the `States` trait. Define states, attach systems to them, and trigger transitions declaratively.

### 1. Game States (Enum)

```rust
use bevy::prelude::*;

/// Primary game state machine controlling the application flow.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    /// Initial boot phase - engine initialization
    #[default]
    Boot,
    /// Splash screen with logos and branding
    Splash,
    /// Optional: Checking for updates
    UpdateCheck,
    /// Main menu - primary navigation hub
    MainMenu,
    /// Settings menu (graphics, audio, controls)
    Settings,
    /// Loading screen - asset preparation before gameplay
    Loading,
    /// Active gameplay
    Gameplay,
    /// Paused state (overlay on gameplay)
    Paused,
    /// Credits/about screen
    Credits,
}

/// Sub-states for complex menu navigation (optional).
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(GameState = GameState::Settings)]
pub enum SettingsTab {
    #[default]
    Graphics,
    Audio,
    Controls,
    Accessibility,
}
```

### 2. Plugin Setup (Entry Point)

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Planetarium".into(),
                resolution: (1280., 720.).into(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        // State machine initialization
        .init_state::<GameState>()
        // Add state-specific systems
        .add_plugins((
            BootPlugin,
            SplashPlugin,
            MainMenuPlugin,
            SettingsPlugin,
            LoadingPlugin,
            GameplayPlugin,
        ))
        .run();
}
```

### 3. Boot Phase Plugin

```rust
pub struct BootPlugin;

impl Plugin for BootPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Boot), setup_boot)
            .add_systems(
                Update,
                boot_complete_check.run_if(in_state(GameState::Boot)),
            );
    }
}

fn setup_boot(mut commands: Commands) {
    // Spawn 2D camera for UI
    commands.spawn(Camera2d);
    
    // Initialize global resources
    commands.insert_resource(GameSettings::default());
    commands.insert_resource(SaveData::default());
    
    info!("Boot phase started");
}

fn boot_complete_check(
    mut next_state: ResMut<NextState<GameState>>,
    // Add resource checks here as needed
) {
    // Boot is instant in most cases - proceed to splash
    next_state.set(GameState::Splash);
}

/// Global game settings resource.
#[derive(Resource, Default)]
pub struct GameSettings {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub graphics_quality: GraphicsQuality,
}

#[derive(Default, Clone, Copy)]
pub enum GraphicsQuality {
    Low,
    #[default]
    Medium,
    High,
    Ultra,
}

/// Save data container.
#[derive(Resource, Default)]
pub struct SaveData {
    pub last_session: Option<String>,
    pub total_playtime: f64,
}
```

### 4. Splash Screen Plugin

```rust
use std::time::Duration;

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
```

### 5. Main Menu Plugin

```rust
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(
                Update,
                (handle_menu_interaction, animate_menu_hover)
                    .run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu);
    }
}

#[derive(Component)]
struct MainMenuRoot;

#[derive(Component, Clone, Copy)]
enum MenuButton {
    NewGame,
    Continue,
    LoadGame,
    Settings,
    Credits,
    Exit,
}

fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
            MainMenuRoot,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("PLANETARIUM"),
                TextFont {
                    font_size: 72.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(60.0)),
                    ..default()
                },
            ));

            // Menu buttons
            let buttons = [
                (MenuButton::NewGame, "New Game"),
                (MenuButton::Continue, "Continue"),
                (MenuButton::Settings, "Settings"),
                (MenuButton::Credits, "Credits"),
                (MenuButton::Exit, "Exit"),
            ];

            for (button_type, label) in buttons {
                spawn_menu_button(parent, button_type, label);
            }
        });

    info!("Main menu setup complete");
}

fn spawn_menu_button(parent: &mut ChildBuilder, button_type: MenuButton, label: &str) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(280.0),
                height: Val::Px(56.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.3)),
            BorderColor(Color::srgb(0.4, 0.4, 0.6)),
            BorderRadius::all(Val::Px(8.0)),
            button_type,
        ))
        .with_child((
            Text::new(label),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
}

fn handle_menu_interaction(
    query: Query<(&Interaction, &MenuButton), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit_events: EventWriter<AppExit>,
) {
    for (interaction, button) in &query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match button {
            MenuButton::NewGame => {
                info!("New Game selected");
                next_state.set(GameState::Loading);
            }
            MenuButton::Continue => {
                info!("Continue selected");
                next_state.set(GameState::Loading);
            }
            MenuButton::LoadGame => {
                info!("Load Game selected");
                // TODO: Open save browser
            }
            MenuButton::Settings => {
                info!("Settings selected");
                next_state.set(GameState::Settings);
            }
            MenuButton::Credits => {
                info!("Credits selected");
                next_state.set(GameState::Credits);
            }
            MenuButton::Exit => {
                info!("Exit requested");
                exit_events.send(AppExit::Success);
            }
        }
    }
}

fn animate_menu_hover(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<MenuButton>)>,
) {
    for (interaction, mut bg_color) in &mut query {
        *bg_color = match interaction {
            Interaction::Pressed => BackgroundColor(Color::srgb(0.15, 0.15, 0.25)),
            Interaction::Hovered => BackgroundColor(Color::srgb(0.3, 0.3, 0.45)),
            Interaction::None => BackgroundColor(Color::srgb(0.2, 0.2, 0.3)),
        };
    }
}

fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
    info!("Main menu cleaned up");
}
```

### 6. Loading Screen Plugin

```rust
use bevy::asset::LoadState;

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
}

fn setup_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                        BorderRadius::all(Val::Px(4.0)),
                        LoadingProgressBar,
                    ));
                });
        });

    // Start loading assets
    let handles: Vec<UntypedHandle> = vec![
        asset_server.load::<Image>("textures/planets/earth.png").untyped(),
        asset_server.load::<Image>("textures/planets/moon.png").untyped(),
        asset_server.load::<Image>("textures/stars/starfield.png").untyped(),
        // Add more assets here
    ];

    let total = handles.len();
    commands.insert_resource(LoadingAssets {
        handles,
        total_count: total,
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

    let progress = loaded as f32 / loading.total_count.max(1) as f32;

    // Update progress bar width
    if let Ok(mut bar) = progress_bar.get_single_mut() {
        bar.width = Val::Percent(progress * 100.0);
    }

    // Update text
    if let Ok(mut text) = progress_text.get_single_mut() {
        **text = format!("Loading... {}%", (progress * 100.0) as i32);
    }
}

fn check_loading_complete(
    asset_server: Res<AssetServer>,
    loading: Res<LoadingAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let all_loaded = loading.handles.iter().all(|h| {
        matches!(
            asset_server.get_load_state(h.id()),
            Some(LoadState::Loaded)
        )
    });

    if all_loaded {
        info!("All assets loaded, entering gameplay");
        next_state.set(GameState::Gameplay);
    }
}

fn cleanup_loading(mut commands: Commands, query: Query<Entity, With<LoadingScreen>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
    commands.remove_resource::<LoadingAssets>();
    info!("Loading screen cleaned up");
}
```

### 7. Settings Plugin (Stub)

```rust
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Settings), setup_settings)
            .add_systems(
                Update,
                handle_settings_back.run_if(in_state(GameState::Settings)),
            )
            .add_systems(OnExit(GameState::Settings), cleanup_settings);
    }
}

#[derive(Component)]
struct SettingsScreen;

fn setup_settings(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
        SettingsScreen,
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("Settings\n\nPress ESC to return"),
            TextFont {
                font_size: 32.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
    });
}

fn handle_settings_back(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
}

fn cleanup_settings(mut commands: Commands, query: Query<Entity, With<SettingsScreen>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
```

### 8. Gameplay Plugin (Stub)

```rust
pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gameplay), setup_gameplay)
            .add_systems(
                Update,
                (gameplay_update, handle_pause_input).run_if(in_state(GameState::Gameplay)),
            )
            .add_systems(OnExit(GameState::Gameplay), cleanup_gameplay);
    }
}

#[derive(Component)]
struct GameplayCamera;

fn setup_gameplay(mut commands: Commands) {
    // Spawn 3D camera for gameplay
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        GameplayCamera,
    ));

    info!("Gameplay started");
}

fn gameplay_update() {
    // Main game logic goes here
}

fn handle_pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        info!("Game paused");
        next_state.set(GameState::Paused);
    }
}

fn cleanup_gameplay(mut commands: Commands, query: Query<Entity, With<GameplayCamera>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
    info!("Gameplay cleaned up");
}
```

## ⚙️ Key Bevy Patterns Used

| Concept | Implementation |
| ------- | -------------- |
| **State transitions** | `ResMut<NextState<T>>` with `set()` method |
| **Per-state logic** | `OnEnter`, `Update.run_if(in_state())`, `OnExit` |
| **Async operations** | `Res<AssetServer>` with load state polling |
| **Resource cleanup** | `OnExit` systems + `despawn_recursive()` |
| **Component markers** | Tag components for state-specific entities |
| **Plugin organization** | One plugin per major state/feature |

## ⚙️ Technical Recommendations

| Aspect | Best Practice |
| ------ | ------------- |
| **Async Loading** | Use `AssetServer` with load state checks; show progress only when >500ms |
| **Graceful Degradation** | Default to offline mode if network unavailable |
| **Input Responsiveness** | Main menu must accept input within 2 seconds of splash end |
| **Memory Management** | `despawn_recursive()` on state exit; remove resources |
| **Platform Compliance** | Respect platform boot time limits |
| **Logging** | Use `info!()`, `warn!()`, `error!()` for diagnostics |
| **Accessibility** | Support system-level accessibility from first frame |

## 📊 Launch Timeline Targets

```plaintext
0–200 ms:     Engine core initialization (Boot)
200–2500 ms:  Splash screen (with async asset preloading)
2500–3000 ms: Update check (if online; skip otherwise)
~3000 ms:     Main menu interactive
```

> 🎮 **Mobile target**: <1.5 seconds to interactive menu

## ❌ Anti-Patterns to Avoid

- ❌ Black screens longer than 500ms without feedback
- ❌ Blocking main thread for I/O operations
- ❌ Mandatory updates without offline fallback
- ❌ Missing "Back" button in nested menus
- ❌ Loading all assets before main menu
- ❌ Ignoring system language/locale settings
- ❌ Forgetting to clean up state-specific entities on exit
- ❌ Using raw `despawn()` instead of `despawn_recursive()`

## 💡 Pro Tips

1. **Progressive Disclosure** — Only load assets needed for the current screen
2. **Predictive Loading** — While in main menu, preload first-level assets in background
3. **Cache Warmup** — On subsequent launches, reuse shader caches for faster startup
4. **Platform Hooks** — Handle OS suspend/resume events (mobile/consoles)
5. **Telemetry** — Measure `time_to_main_menu` as a core KPI
6. **Error Handling** — Always have fallback states for failed loads
7. **Hot Reload** — Use Bevy's asset hot-reload in development builds

## 📁 Recommended Project Structure

```plaintext
src/
├── main.rs              # Entry point with App setup
├── lib.rs               # Optional: library root
├── states/
│   ├── mod.rs           # GameState enum + exports
│   ├── boot.rs          # BootPlugin
│   ├── splash.rs        # SplashPlugin
│   ├── main_menu.rs     # MainMenuPlugin
│   ├── settings.rs      # SettingsPlugin
│   ├── loading.rs       # LoadingPlugin
│   ├── gameplay.rs      # GameplayPlugin
│   └── paused.rs        # PausePlugin
├── ui/
│   ├── mod.rs
│   ├── buttons.rs       # Reusable button components
│   └── theme.rs         # Colors, fonts, spacing constants
├── resources/
│   ├── mod.rs
│   ├── settings.rs      # GameSettings resource
│   └── save_data.rs     # SaveData resource
└── utils/
    └── mod.rs           # Utility functions
```

## 🔧 Dependencies Required

Add these to your `Cargo.toml`:

```toml
[dependencies]
bevy = "0.18"

# Optional: Enhanced logging
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

## ✅ Implementation Checklist

- [ ] Define `GameState` enum with all states
- [ ] Create `BootPlugin` with initialization
- [ ] Create `SplashPlugin` with skip functionality
- [ ] Create `MainMenuPlugin` with navigation
- [ ] Create `SettingsPlugin` with tabs
- [ ] Create `LoadingPlugin` with progress bar
- [ ] Create `GameplayPlugin` with pause handling
- [ ] Add cleanup systems for all states
- [ ] Test state transitions in all directions
- [ ] Measure and optimize boot time
