# Launching Game

Here's a professionally structured game launch sequence algorithm in English, using a **State Machine** or **Scene Management** architecture — the industry-standard approach.

## 📋 Recommended Launch Sequence (Step-by-Step)

### Phase 0: Engine Initialization (Pre-First Frame)

```plaintext
1. Executable launch
2. Core engine initialization:
   - Graphics API (DirectX/Vulkan/Metal/OpenGL)
   - Audio subsystem
   - Input system (keyboard, mouse, gamepad, touch)
   - Physics engine
   - Networking layer (if required)
3. Resource path resolution & config loading
4. Load global settings (from %APPDATA%, ~/Library, or equivalent)
5. Initialize analytics/crash reporting (non-blocking)
```

### Phase 1: Splash Screen (3–5 seconds)

```plaintext
Purpose: Branding + background preparation
Flow:
├─ Display engine logo (if required by license, e.g., Unreal)
├─ Display studio/publisher logo(s)
└─ Background tasks (async, non-blocking):
    • Preload minimal assets for next screen
    • Verify critical file integrity (checksums/hashes)
    • Initialize analytics/tracking SDKs
    • Warm up shader caches (if applicable)
```

> ⚠️ Never block rendering thread — heavy operations must be async!

### Phase 2: Content Validation / Update Check *(Online games only)*

```plaintext
Condition: Skip for pure offline games; optional for hybrid titles
├─ Check server for latest version
├─ Compare local file hashes vs. CDN manifests
├─ Download patches/DLC if needed:
│   • Show progress bar with cancel option
│   • Allow offline play if non-critical (graceful degradation)
├─ Validate license/account session (if DRM required)
└─ Apply hotfixes/modifications to local files
```

> 💡 For mobile/console: Integrate with platform store APIs (Steam, Epic, Google Play, App Store)

### Phase 3: Main Menu / Home Screen

```plaintext
Trigger: After all critical systems are ready
UI Elements:
├─ [Play / New Game] → profile creation / save slot selection
├─ [Continue] → auto-load last save (if exists)
├─ [Load Game] → save slot browser with thumbnails & timestamps
├─ [Settings]
│   ├─ Graphics (resolution, quality presets, VSync, FPS cap)
│   ├─ Audio (master/music/SFX/voice volumes, audio device)
│   ├─ Controls (key rebinding, sensitivity, invert Y)
│   ├─ Language & subtitles
│   └─ Accessibility (colorblind modes, UI scaling)
├─ [Extras] → Achievements, Stats, Credits, Art Gallery (optional)
├─ [Store / DLC] → In-game marketplace (if applicable)
└─ [Exit Game] → Confirmation dialog → graceful shutdown
```

## 🔁 State Machine Architecture (Rust + Bevy)

Here's the **State Machine Architecture** section rewritten with idiomatic Rust + Bevy implementation — leveraging Bevy's built-in `States` system rather than manual state management:

Bevy handles state transitions natively via the `States` trait. No manual state managers needed — just define states, attach systems to them, and trigger transitions.

### 1. Define Game States (Enum)

```rust
use bevy::prelude::*;

// Derive macros required for Bevy's state system
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Boot,
    Splash,
    UpdateCheck,
    MainMenu,
    Settings,
    Gameplay,
    Quit,
}
```

### 2. Minimal Plugin Setup (Entry Point)

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Register state machine BEFORE systems
        .init_state::<GameState>()
        // Phase 0: Engine init happens automatically via DefaultPlugins
        
        // Phase 1: Splash screen systems
        .add_systems(OnEnter(GameState::Splash), show_splash_screen)
        .add_systems(Update, splash_timer.run_if(in_state(GameState::Splash)))
        .add_systems(OnExit(GameState::Splash), cleanup_splash)
        
        // Phase 2: Update check (optional)
        .add_systems(OnEnter(GameState::UpdateCheck), check_for_updates)
        .add_systems(Update, update_progress.run_if(in_state(GameState::UpdateCheck)))
        
        // Phase 3: Main menu
        .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
        .add_systems(Update, (
            handle_menu_input,
            animate_menu_ui,
        ).run_if(in_state(GameState::MainMenu)))
        .add_systems(OnExit(GameState::MainMenu), cleanup_menu)
        
        // Graceful shutdown
        .add_systems(Update, handle_quit.run_if(in_state(GameState::Quit)))
        .run();
}
```

### 3. State Transition Example: Splash → Main Menu

```rust
#[derive(Component)]
struct SplashLogo;

fn show_splash_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn logo with fade-in animation
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("logo.png"),
            transform: Transform::from_scale(Vec3::splat(0.0)),
            ..default()
        },
        SplashLogo,
        Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_secs(1),
            scale(Vec3::ONE),
        ),
    ));
    
    // Auto-transition after 3 seconds
    commands.insert_resource(SplashTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}

#[derive(Resource)]
struct SplashTimer(Timer);

fn splash_timer(
    mut timer: ResMut<SplashTimer>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if timer.0.tick(time.delta()).finished() {
        next_state.set(GameState::MainMenu); // ← Bevy handles transition automatically
    }
}

fn cleanup_splash(
    mut commands: Commands,
    query: Query<Entity, With<SplashLogo>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn(); // Clean up splash assets
    }
}
```

### 4. Main Menu Input Handling

```rust
#[derive(Component)]
enum MenuItem {
    Play,
    Settings,
    Quit,
}

fn handle_menu_input(
    mut interaction_query: Query<(&Interaction, &MenuItem), Changed<Interaction>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, menu_item) in &mut interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            match menu_item {
                MenuItem::Play => next_game_state.set(GameState::Gameplay),
                MenuItem::Settings => next_game_state.set(GameState::Settings),
                MenuItem::Quit => app_exit_events.send(AppExit), // Immediate exit
            }
        }
    }
}
```

### 5. Optional: Async Update Check (Using Bevy Tasks)

```rust
use bevy::tasks::{IoTaskPool, Task};

#[derive(Resource)]
struct UpdateCheckTask(Task<Result<bool, String>>); // true = needs update

fn check_for_updates(mut commands: Commands) {
    let task = IoTaskPool::get().spawn(async move {
        // Simulate network request
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        Ok(false) // false = no update needed
    });
    
    commands.insert_resource(UpdateCheckTask(task));
}

fn update_progress(
    mut commands: Commands,
    mut task_res: Option<ResMut<UpdateCheckTask>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Some(mut task) = task_res {
        if let Some(result) = futures::executor::block_on(futures::poll_once(&mut task.0)) {
            match result {
                Ok(needs_update) => {
                    if needs_update {
                        // Show patch UI instead of main menu
                    } else {
                        next_state.set(GameState::MainMenu);
                    }
                }
                Err(e) => {
                    warn!("Update check failed: {}", e);
                    next_state.set(GameState::MainMenu); // Fallback to offline mode
                }
            }
            commands.remove_resource::<UpdateCheckTask>();
        }
    }
}
```

## ⚙️ Key Bevy-Specific Patterns

| Concept | Bevy Implementation |
| ------- | ------------------- |
| **State transitions** | `ResMut<NextState<T>>` or `StateTransition` events |
| **Per-state logic** | `OnEnter`, `Update.run_if(in_state())`, `OnExit` system sets |
| **Async operations** | `IoTaskPool` + `Task<T>` with polling in update systems |
| **Resource cleanup** | `OnExit` systems + manual despawn of state-specific entities |
| **Transition animations** | Use `Tween` (via `bevy_tweening`) or custom animation systems during state overlap |

> 💡 **Pro Tip**: For production games, wrap state transitions in a `TransitionPlugin` that handles fade-to-black animations between states using Bevy's `Assets<ColorMaterial>` and overlay cameras.

This approach is **idiomatic Bevy** — leveraging the engine's ECS architecture instead of forcing OOP patterns. Transitions are non-blocking, memory-safe, and fully integrated with Bevy's parallel scheduler. Need a complete minimal example project structure or networking integration?

## ⚙️ Critical Technical Recommendations

| Aspect | Best Practice |
| ------ | ------------- |
| **Async Loading** | Use background threads/async APIs; show progress indicators only when >500ms |
| **Graceful Degradation** | Allow offline play when possible; cache auth tokens |
| **Input Responsiveness** | Main menu must accept input within 2 seconds of splash end |
| **Memory Management** | Unload splash assets immediately after transition |
| **Platform Compliance** | Respect platform-specific requirements (e.g., Nintendo's boot time limits) |
| **Analytics** | Log stage durations for optimization, but never block gameplay |
| **Accessibility** | Support system-level accessibility features from first frame |

## 📊 Typical Launch Timeline (Target Benchmarks)

```plaintext
0–300 ms:     Engine core initialization
300–1800 ms:  Splash screen (with async asset preloading)
1800–2200 ms: Update check (if online; skip if offline)
2200 ms:      Main menu interactive (target for AAA/indie titles)
```

> 🎮 *Mobile target*: <1.5 seconds to interactive menu (App Store/Google Play expectations)

## ❌ Anti-Patterns to Avoid

- ❌ Black screens longer than 500ms without feedback
- ❌ Blocking main thread for downloads/decryption
- ❌ Mandatory updates without offline fallback (unless security-critical)
- ❌ Missing "Back" button in nested menus
- ❌ Loading entire game assets before main menu (wastes memory/time)
- ❌ Ignoring system language/locale settings

## 💡 Pro Tips

1. **Progressive Disclosure**: Only load what's needed for the *current* screen.
2. **Predictive Loading**: While player reads main menu, preload first-level assets in background.
3. **Cache Warmup**: On subsequent launches, reuse shader caches & asset bundles for 2–5× faster startup.
4. **Platform Hooks**: Integrate with OS suspend/resume events (mobile/consoles).
5. **Telemetry**: Measure `time_to_main_menu` as a core KPI — players abandon games with slow startups.

This architecture is engine-agnostic (works with Unity, Unreal, Godot, custom engines) and scales from mobile hyper-casual to AAA titles. Need implementation examples for a specific engine or deeper dive into async asset loading patterns?
