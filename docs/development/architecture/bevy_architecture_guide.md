# Bevy Game Engine Architecture Guide for AI Agents

## Document Purpose

This guide provides comprehensive architectural patterns and best practices for developing games using the Bevy game engine with Entity Component System (ECS) paradigm. It emphasizes a clean separation between reusable framework code and game-specific logic.

---

## Core Architecture Philosophy

### Two-Layer Architecture

Every Bevy project should be structured with two distinct conceptual layers:

1. **Framework Layer** - Reusable, game-agnostic infrastructure
2. **Game Layer** - Project-specific game logic

This separation enables:

- Easy portability of boilerplate code to new projects
- Clear boundaries between infrastructure and business logic
- Maintainable, scalable codebase
- Testable components in isolation

---

## Project Structure

```plaintext
project_name/
├── assets/                    # Game assets (textures, models, audio, fonts, shaders)
│   ├── textures/
│   ├── models/
│   ├── audio/
│   ├── fonts/
│   └── shaders/
│
├── src/
│   ├── main.rs               # Application entry point
│   ├── lib.rs                # Library root (enables testing/benchmarking)
│   │
│   ├── framework/            # 🔧 REUSABLE FRAMEWORK LAYER
│   │   ├── mod.rs
│   │   ├── plugin.rs         # FrameworkPlugin bundle
│   │   │
│   │   ├── states/           # Framework-level state machine
│   │   │   ├── mod.rs
│   │   │   ├── app_state.rs  # Core app states enum
│   │   │   └── transition.rs # State transition logic
│   │   │
│   │   ├── splash/           # Splash screen system
│   │   │   ├── mod.rs
│   │   │   ├── systems.rs
│   │   │   ├── components.rs
│   │   │   └── resources.rs
│   │   │
│   │   ├── loading/          # Asset loading system
│   │   │   ├── mod.rs
│   │   │   ├── systems.rs
│   │   │   ├── resources.rs
│   │   │   └── assets.rs     # Asset handles and paths
│   │   │
│   │   ├── menu/             # Menu systems
│   │   │   ├── mod.rs
│   │   │   ├── main_menu/    # Main menu screen
│   │   │   │   ├── mod.rs
│   │   │   │   ├── systems.rs
│   │   │   │   └── layout.rs
│   │   │   ├── pause_menu/   # In-game pause menu
│   │   │   │   ├── mod.rs
│   │   │   │   └── systems.rs
│   │   │   └── components.rs # Shared menu UI components
│   │   │
│   │   ├── settings/         # Settings management
│   │   │   ├── mod.rs
│   │   │   ├── systems.rs
│   │   │   ├── resources.rs  # SettingsResource
│   │   │   └── ui.rs         # Settings UI
│   │   │
│   │   ├── audio/            # Audio management system
│   │   │   ├── mod.rs
│   │   │   ├── systems.rs
│   │   │   └── resources.rs  # AudioSettings, SoundManager
│   │   │
│   │   ├── camera/           # Camera controllers
│   │   │   ├── mod.rs
│   │   │   ├── orbit.rs      # Orbit camera controller
│   │   │   └── first_person.rs
│   │   │
│   │   └── ui/               # Shared UI utilities
│   │       ├── mod.rs
│   │       ├── styles.rs     # UI style constants
│   │       ├── widgets.rs    # Reusable UI widget components
│   │       └── layout.rs     # Layout helper functions
│   │
│   ├── game/                 # 🎮 GAME-SPECIFIC LOGIC LAYER
│   │   ├── mod.rs
│   │   ├── plugin.rs         # GamePlugin bundle
│   │   │
│   │   ├── states/           # Game-specific substates (optional)
│   │   │   ├── mod.rs
│   │   │   └── game_state.rs # GameState enum (if needed)
│   │   │
│   │   ├── components/       # Game components (ECS data)
│   │   │   ├── mod.rs
│   │   │   ├── player.rs
│   │   │   ├── enemy.rs
│   │   │   ├── physics.rs
│   │   │   └── ...
│   │   │
│   │   ├── systems/          # Game systems (ECS logic)
│   │   │   ├── mod.rs
│   │   │   ├── setup.rs      # Scene initialization
│   │   │   ├── gameplay.rs   # Core gameplay logic
│   │   │   ├── physics.rs    # Physics simulation
│   │   │   ├── combat.rs     # Combat systems
│   │   │   ├── input.rs      # Input handling
│   │   │   └── ...
│   │   │
│   │   ├── resources/        # Game resources (ECS global data)
│   │   │   ├── mod.rs
│   │   │   ├── score.rs
│   │   │   ├── level.rs
│   │   │   └── ...
│   │   │
│   │   ├── entities/         # Entity spawner functions
│   │   │   ├── mod.rs
│   │   │   ├── player.rs     # spawn_player()
│   │   │   ├── enemy.rs      # spawn_enemy()
│   │   │   └── ...
│   │   │
│   │   └── constants.rs      # Game constants and configuration
│   │
│   ├── config/               # Configuration management
│   │   ├── mod.rs
│   │   └── game_config.rs
│   │
│   └── utils/                # Shared utility functions
│       ├── mod.rs
│       ├── math.rs           # Mathematical helpers
│       └── debug.rs          # Debug utilities
│
├── tests/                    # Integration tests
│   ├── framework_tests.rs
│   └── game_tests.rs
│
├── benches/                  # Performance benchmarks
│   └── game_bench.rs
│
├── docs/                     # Documentation
│   ├── architecture.md
│   ├── framework_guide.md
│   └── game_design.md
│
├── Cargo.toml
├── .gitignore
└── README.md
```

---

## Naming Conventions

### Module Organization

#### Framework Layer

- **Purpose**: Contains reusable, game-agnostic infrastructure
- **Naming**: Use generic, descriptive names (splash, loading, menu, settings)
- **Files**:
  - `mod.rs` - Module declaration and re-exports
  - `plugin.rs` - Bevy plugin implementation
  - `systems.rs` - System functions
  - `components.rs` - Component definitions
  - `resources.rs` - Resource definitions

#### Game Layer

- **Purpose**: Contains project-specific game logic
- **Naming**: Use domain-specific names relevant to your game
- **Organization**: Strictly follow ECS patterns
  - `components/` - Data structures
  - `systems/` - Logic functions
  - `resources/` - Global state
  - `entities/` - Entity spawner functions

### State Naming

```rust
// Framework states - generic application flow
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Splash,           // Logo/splash screen
    Loading,          // Asset loading screen
    MainMenu,         // Main menu
    InGame,           // Active gameplay
    Paused,           // Game paused
    Settings,         // Settings screen
    GameOver,         // Game over screen (if applicable)
}

// Game states - specific to your game (optional, use only if needed)
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Exploring,
    Combat,
    Dialogue,
    Cutscene,
}
```

### Component Naming

```rust
// Components represent data - use nouns
#[derive(Component)]
struct Position { x: f32, y: f32, z: f32 }

#[derive(Component)]
struct Velocity { x: f32, y: f32, z: f32 }

#[derive(Component)]
struct Health { current: f32, max: f32 }

#[derive(Component)]
struct Player;  // Marker component

#[derive(Component)]
struct Enemy { enemy_type: EnemyType }
```

### System Naming

```rust
// Systems represent actions - use verbs
fn setup_game(...)          // Setup/initialization
fn update_physics(...)      // Update logic
fn spawn_enemies(...)       // Entity creation
fn handle_collisions(...)   // Event handling
fn render_ui(...)           // Rendering
fn cleanup_entities(...)    // Cleanup
```

### Resource Naming

```rust
// Resources are global state - use descriptive nouns
#[derive(Resource)]
struct GameConfig { ... }

#[derive(Resource)]
struct Score { value: u32 }

#[derive(Resource)]
struct LevelData { current_level: u32 }

#[derive(Resource)]
struct AudioSettings { 
    master_volume: f32,
    music_volume: f32,
    sfx_volume: f32,
}
```

---

## Plugin Architecture Pattern

### Framework Plugin Structure

```rust
// framework/plugin.rs
use bevy::prelude::*;
use super::{splash, loading, menu, settings, audio};

pub struct FrameworkPlugin;

impl Plugin for FrameworkPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add framework states
            .init_state::<AppState>()
            
            // Add framework sub-plugins
            .add_plugins((
                splash::SplashPlugin,
                loading::LoadingPlugin,
                menu::MenuPlugin,
                settings::SettingsPlugin,
                audio::AudioPlugin,
            ));
    }
}
```

### Individual Module Plugin Pattern

```rust
// framework/splash/mod.rs
pub mod systems;
pub mod resources;

use bevy::prelude::*;
use crate::framework::states::AppState;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize resources
            .init_resource::<resources::SplashTimer>()
            
            // Add systems with proper scheduling
            .add_systems(OnEnter(AppState::Splash), systems::setup_splash)
            .add_systems(
                Update,
                systems::update_splash_timer.run_if(in_state(AppState::Splash))
            )
            .add_systems(OnExit(AppState::Splash), systems::cleanup_splash);
    }
}
```

### Game Plugin Structure

```rust
// game/plugin.rs
use bevy::prelude::*;
use crate::framework::states::AppState;
use super::{systems, components, resources};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize game resources
            .init_resource::<resources::Score>()
            .init_resource::<resources::LevelData>()
            
            // Add systems tied to InGame state
            .add_systems(OnEnter(AppState::InGame), systems::setup::initialize_game)
            .add_systems(
                Update,
                (
                    systems::input::handle_player_input,
                    systems::physics::update_physics,
                    systems::gameplay::update_game_logic,
                )
                    .chain()  // Run in sequence if order matters
                    .run_if(in_state(AppState::InGame))
            )
            .add_systems(OnExit(AppState::InGame), systems::cleanup::cleanup_game);
    }
}
```

---

## ECS Pattern Implementation

### Component Definition Pattern

```rust
// game/components/player.rs
use bevy::prelude::*;

// Marker component - no data, just tags an entity
#[derive(Component)]
pub struct Player;

// Data component - stores information
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }
    
    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }
    
    pub fn take_damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }
}

// Complex component with nested data
#[derive(Component)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub capacity: usize,
}

#[derive(Clone)]
pub struct Item {
    pub id: String,
    pub quantity: u32,
}
```

### System Definition Pattern

```rust
// game/systems/gameplay.rs
use bevy::prelude::*;
use crate::game::components::*;
use crate::game::resources::*;

// Simple system - read-only query
pub fn update_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity)>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

// Filtered query system
pub fn damage_enemies(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Health), With<Enemy>>,
    time: Res<Time>,
) {
    for (entity, mut health) in &mut query {
        health.take_damage(10.0 * time.delta_seconds());
        
        if !health.is_alive() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// Multi-query system with different filters
pub fn handle_collisions(
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for enemy_transform in &enemy_query {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
                
            if distance < 1.0 {
                score.value += 10;
            }
        }
    }
}

// System with Commands for entity spawning/despawning
pub fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
) {
    spawn_timer.timer.tick(time.delta());
    
    if spawn_timer.timer.just_finished() {
        commands.spawn((
            Enemy,
            Health::new(100.0),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));
    }
}
```

### Resource Definition Pattern

```rust
// game/resources/score.rs
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
    pub high_score: u32,
}

impl Score {
    pub fn add_points(&mut self, points: u32) {
        self.value += points;
        if self.value > self.high_score {
            self.high_score = self.value;
        }
    }
    
    pub fn reset(&mut self) {
        self.value = 0;
    }
}

// Resource with timer
#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        }
    }
}
```

### Entity Spawner Pattern

```rust
// game/entities/player.rs
use bevy::prelude::*;
use crate::game::components::*;

pub fn spawn_player(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((
            Player,
            Health::new(100.0),
            SpriteBundle {
                texture: asset_server.load("textures/player.png"),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
        ))
        .id()
}

// Complex entity with children
pub fn spawn_enemy_with_healthbar(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
) -> Entity {
    let enemy_entity = commands
        .spawn((
            Enemy,
            Health::new(50.0),
            SpriteBundle {
                texture: asset_server.load("textures/enemy.png"),
                transform: Transform::from_translation(position),
                ..default()
            },
        ))
        .with_children(|parent| {
            // Spawn healthbar as child
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: Some(Vec2::new(40.0, 5.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 30.0, 1.0),
                ..default()
            });
        })
        .id();
    
    enemy_entity
}
```

---

## State Management Patterns

### State Transition System

```rust
// framework/states/transition.rs
use bevy::prelude::*;
use super::app_state::AppState;

pub fn check_splash_complete(
    time: Res<Time>,
    mut splash_timer: ResMut<SplashTimer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    splash_timer.timer.tick(time.delta());
    
    if splash_timer.timer.just_finished() {
        next_state.set(AppState::Loading);
    }
}

pub fn check_loading_complete(
    loading_state: Res<LoadingState>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if loading_state.is_complete() {
        next_state.set(AppState::MainMenu);
    }
}
```

### State-Dependent System Scheduling

```rust
// Systems that only run in specific states
app.add_systems(
    Update,
    (
        // These run ONLY when in InGame state
        handle_player_input,
        update_physics,
        check_collisions,
    )
        .run_if(in_state(AppState::InGame))
);

// Systems that run in multiple states
app.add_systems(
    Update,
    render_ui.run_if(
        in_state(AppState::InGame)
            .or_else(in_state(AppState::Paused))
    )
);

// OnEnter/OnExit systems for state transitions
app.add_systems(OnEnter(AppState::InGame), setup_game);
app.add_systems(OnExit(AppState::InGame), cleanup_game);
```

---

## Main Entry Point Pattern

```rust
// src/main.rs
use bevy::prelude::*;

mod framework;
mod game;
mod config;
mod utils;

fn main() {
    App::new()
        // Default Bevy plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "My Game".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        
        // Add framework plugin (reusable infrastructure)
        .add_plugins(framework::FrameworkPlugin)
        
        // Add game plugin (game-specific logic)
        .add_plugins(game::GamePlugin)
        
        // Run the app
        .run();
}
```

---

## Best Practices for AI Agents

### 1. **Always Separate Framework from Game Logic**

✅ **CORRECT:**

```rust
// framework/audio/systems.rs - Generic, reusable
pub fn update_audio_volume(
    settings: Res<AudioSettings>,
    audio: Res<Audio>,
) {
    audio.set_volume(settings.master_volume);
}

// game/systems/gameplay.rs - Game-specific
pub fn play_collision_sound(
    collision_events: EventReader<CollisionEvent>,
    audio: Res<Audio>,
) {
    for event in collision_events.read() {
        audio.play("sounds/hit.ogg");
    }
}
```

❌ **INCORRECT:**

```rust
// Mixing framework and game logic in same file
pub fn handle_audio(
    settings: Res<AudioSettings>,
    collision_events: EventReader<CollisionEvent>,
    audio: Res<Audio>,
) {
    // Don't mix generic settings with game-specific events
}
```

### 2. **Follow ECS Component Design Principles**

✅ **CORRECT - Small, focused components:**

```rust
#[derive(Component)]
struct Position(Vec3);

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct Health { current: f32, max: f32 }

#[derive(Component)]
struct Player;
```

❌ **INCORRECT - Monolithic component:**

```rust
#[derive(Component)]
struct PlayerData {
    position: Vec3,
    velocity: Vec3,
    health: f32,
    max_health: f32,
    inventory: Vec<Item>,
    stats: PlayerStats,
    // Too much data in one component!
}
```

### 3. **Use Proper System Scheduling**

✅ **CORRECT:**

```rust
app.add_systems(
    Update,
    (
        // Systems with dependencies run in sequence
        handle_input,
        apply_velocity,
        check_collisions,
    )
        .chain()  // Enforces order
        .run_if(in_state(AppState::InGame))
);
```

❌ **INCORRECT:**

```rust
// Don't assume execution order without .chain()
app.add_systems(
    Update,
    (
        check_collisions,  // Might run before apply_velocity!
        apply_velocity,
        handle_input,
    )
);
```

### 4. **Implement Clean State Transitions**

✅ **CORRECT:**

```rust
// OnEnter: Setup state
.add_systems(OnEnter(AppState::InGame), (
    spawn_player,
    spawn_level,
    initialize_camera,
))

// Update: Run during state
.add_systems(Update, gameplay_systems.run_if(in_state(AppState::InGame)))

// OnExit: Cleanup state
.add_systems(OnExit(AppState::InGame), (
    despawn_entities,
    save_progress,
))
```

### 5. **Use Entity Spawner Functions**

✅ **CORRECT:**

```rust
// game/entities/enemy.rs
pub fn spawn_enemy(
    commands: &mut Commands,
    position: Vec3,
    enemy_type: EnemyType,
) -> Entity {
    commands.spawn((
        Enemy { enemy_type },
        Health::new(100.0),
        Transform::from_translation(position),
    )).id()
}

// Usage in system:
fn spawn_wave(mut commands: Commands) {
    spawn_enemy(&mut commands, Vec3::ZERO, EnemyType::Basic);
}
```

❌ **INCORRECT:**

```rust
// Don't spawn entities directly in every system
fn spawn_wave(mut commands: Commands) {
    commands.spawn((
        Enemy { enemy_type: EnemyType::Basic },
        Health { current: 100.0, max: 100.0 },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
```

### 6. **Module Organization Pattern**

Each module should follow this structure:

```rust
// module/mod.rs
pub mod systems;
pub mod components;
pub mod resources;

pub use systems::*;
pub use components::*;
pub use resources::*;

use bevy::prelude::*;

pub struct ModulePlugin;

impl Plugin for ModulePlugin {
    fn build(&self, app: &mut App) {
        // Plugin implementation
    }
}
```

### 7. **Query Filter Best Practices**

```rust
// Use With<> for filtering without accessing component
fn system_for_players(query: Query<&Transform, With<Player>>) {
    // More efficient than Query<(&Transform, &Player)>
}

// Use Without<> for negative filters
fn system_for_alive_entities(query: Query<&Transform, Without<Dead>>) {
    // Only entities that DON'T have Dead component
}

// Combine filters with tuples
fn complex_query(
    query: Query<
        &Transform,
        (With<Player>, Without<Dead>, With<Active>)
    >
) {
    // Player entities that are active and not dead
}
```

### 8. **Resource Management**

```rust
// Initialize resources in plugin
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Score>()  // Uses Default trait
            .insert_resource(GameConfig {  // Custom initialization
                difficulty: Difficulty::Normal,
            });
    }
}

// Access resources in systems
fn update_score(mut score: ResMut<Score>) {
    score.value += 1;
}

fn read_config(config: Res<GameConfig>) {
    println!("Difficulty: {:?}", config.difficulty);
}
```

### 9. **Error Handling in Queries**

```rust
// Single entity query - handle errors
fn update_player(
    mut query: Query<&mut Health, With<Player>>
) {
    if let Ok(mut health) = query.get_single_mut() {
        health.current += 1.0;
    } else {
        // Handle no player or multiple players
        warn!("Expected exactly one player entity");
    }
}

// Iterate safely
fn update_enemies(
    mut query: Query<&mut Health, With<Enemy>>
) {
    for mut health in &mut query {
        // Automatically safe - iterates over existing entities
        health.current -= 1.0;
    }
}
```

### 10. **Asset Loading Pattern**

```rust
// framework/loading/resources.rs
#[derive(Resource, Default)]
pub struct GameAssets {
    pub player_texture: Handle<Image>,
    pub enemy_texture: Handle<Image>,
    pub background_music: Handle<AudioSource>,
}

// framework/loading/systems.rs
pub fn load_assets(
    mut game_assets: ResMut<GameAssets>,
    asset_server: Res<AssetServer>,
) {
    game_assets.player_texture = asset_server.load("textures/player.png");
    game_assets.enemy_texture = asset_server.load("textures/enemy.png");
    game_assets.background_music = asset_server.load("audio/music.ogg");
}

pub fn check_assets_loaded(
    game_assets: Res<GameAssets>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if asset_server.is_loaded_with_dependencies(&game_assets.player_texture)
        && asset_server.is_loaded_with_dependencies(&game_assets.enemy_texture)
        && asset_server.is_loaded_with_dependencies(&game_assets.background_music)
    {
        next_state.set(AppState::MainMenu);
    }
}
```

---

## Common Patterns Reference

### Timer Pattern

```rust
#[derive(Resource)]
pub struct SpawnTimer {
    pub timer: Timer,
}

impl Default for SpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        }
    }
}

fn spawn_system(
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    mut commands: Commands,
) {
    timer.timer.tick(time.delta());
    
    if timer.timer.just_finished() {
        // Spawn entity every 2 seconds
    }
}
```

### Cleanup Pattern

```rust
// Marker component for entities to cleanup
#[derive(Component)]
pub struct CleanupOnExit;

// Cleanup system
fn cleanup_entities(
    mut commands: Commands,
    query: Query<Entity, With<CleanupOnExit>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

// Register as OnExit system
app.add_systems(OnExit(AppState::InGame), cleanup_entities);
```

### Event Pattern

```rust
// Define event
#[derive(Event)]
pub struct CollisionEvent {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

// Add event to app
app.add_event::<CollisionEvent>();

// Send events
fn detect_collisions(
    query: Query<(Entity, &Transform)>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    // Detection logic...
    collision_events.send(CollisionEvent {
        entity_a: entity1,
        entity_b: entity2,
    });
}

// Receive events
fn handle_collisions(
    mut collision_events: EventReader<CollisionEvent>,
) {
    for event in collision_events.read() {
        println!("Collision between {:?} and {:?}", event.entity_a, event.entity_b);
    }
}
```

---

## Testing Patterns

### Unit Testing Components

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_take_damage() {
        let mut health = Health::new(100.0);
        health.take_damage(30.0);
        assert_eq!(health.current, 70.0);
    }

    #[test]
    fn test_health_cannot_go_negative() {
        let mut health = Health::new(100.0);
        health.take_damage(150.0);
        assert_eq!(health.current, 0.0);
    }
}
```

### Integration Testing Systems

```rust
// tests/game_tests.rs
use bevy::prelude::*;
use my_game::game::*;

#[test]
fn test_player_movement() {
    let mut app = App::new();
    
    app.add_systems(Update, systems::update_movement);
    
    // Spawn test entity
    let entity = app.world.spawn((
        Transform::default(),
        Velocity { x: 1.0, y: 0.0, z: 0.0 },
    )).id();
    
    // Run one update
    app.update();
    
    // Check results
    let transform = app.world.entity(entity).get::<Transform>().unwrap();
    assert!(transform.translation.x > 0.0);
}
```

---

## Performance Considerations

### 1. Prefer Queries Over Individual Entity Access

```rust
// ✅ Efficient - batch processing
fn update_all(mut query: Query<&mut Health>) {
    for mut health in &mut query {
        health.current += 1.0;
    }
}

// ❌ Inefficient - individual entity access
fn update_one_by_one(
    mut commands: Commands,
    entities: Query<Entity, With<Health>>,
    mut health_query: Query<&mut Health>,
) {
    for entity in &entities {
        if let Ok(mut health) = health_query.get_mut(entity) {
            health.current += 1.0;
        }
    }
}
```

### 2. Use Change Detection

```rust
// Only process entities with changed components
fn react_to_health_changes(
    query: Query<&Health, Changed<Health>>
) {
    for health in &query {
        // Only runs for entities where Health changed this frame
    }
}
```

### 3. Minimize System Ordering Constraints

```rust
// ✅ Allow parallel execution
app.add_systems(Update, (
    independent_system_1,
    independent_system_2,
    independent_system_3,
));

// ⚠️ Use .chain() only when necessary
app.add_systems(Update, (
    must_run_first,
    must_run_second,
).chain());
```

---

## Documentation Standards

### Code Documentation

```rust
/// Represents a player's health status.
///
/// Health decreases when taking damage and increases when healing.
/// When health reaches zero, the entity should be considered dead.
///
/// # Examples
///
/// ```
/// let mut health = Health::new(100.0);
/// health.take_damage(25.0);
/// assert_eq!(health.current, 75.0);
/// ```
#[derive(Component)]
pub struct Health {
    /// Current health points
    pub current: f32,
    /// Maximum health capacity
    pub max: f32,
}

impl Health {
    /// Creates a new Health component with the specified maximum.
    ///
    /// The current health is initialized to the maximum value.
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }
    
    /// Reduces current health by the specified amount.
    ///
    /// Health cannot go below zero.
    pub fn take_damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }
    
    /// Returns true if the entity is still alive (health > 0).
    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }
}
```

---

## Quick Reference Checklist

When creating a new Bevy project, ensure:

- [ ] Project structure separates `framework/` and `game/` layers
- [ ] Each module has `mod.rs`, `plugin.rs`, `systems.rs`, `components.rs`, `resources.rs`
- [ ] Components are small, focused data structures with `#[derive(Component)]`
- [ ] Systems are plain functions that operate on queries
- [ ] Resources are global state with `#[derive(Resource)]`
- [ ] States use clear, descriptive enum variants
- [ ] Plugins bundle related functionality
- [ ] Systems use `.chain()` only when execution order matters
- [ ] Entity spawners are separate functions in `entities/` directory
- [ ] Tests are written for components and systems
- [ ] Documentation includes examples and edge cases

---

## Conclusion

This architecture guide provides a solid foundation for building maintainable, scalable Bevy games. The key principles are:

1. **Separation of Concerns** - Framework vs Game logic
2. **ECS Principles** - Small components, focused systems, global resources
3. **Plugin Architecture** - Modular, composable systems
4. **State Management** - Clear application flow
5. **Performance** - Leverage parallel execution, minimize constraints

When in doubt, refer to this guide and prioritize clean separation, ECS principles, and Bevy's idiomatic patterns.

---

**Document Version**: 1.0  
**Last Updated**: 2026  
**Target Bevy Version**: 0.18+
