# Config System Specification

Отличная система конфигурации! Вижу продуманный подход с разделением на слои. Давайте интегрируем это в нашу архитектуру Framework/Game и обсудим нюансы.

## 🔍 Анализ и интеграция в архитектуру

### Ключевые наблюдения из ваших документов

1. **Dual-Layer Architecture** идеально ложится на нашу концепцию Framework/Game
2. **Reactive Sync** - отличное дополнение к ECS
3. **Security-first подход** - критически важно

## 📐 Предлагаемая интеграция

### 1. Размещение в структуре проекта

```
planetarium/
├── assets/
│   └── dev_config.ron          # Developer config (hot-reloadable)
│
├── src/
│   ├── framework/
│   │   ├── config/             # 🆕 NEW: Config management framework
│   │   │   ├── mod.rs
│   │   │   ├── plugin.rs       # ConfigPlugin
│   │   │   ├── game_config.rs  # Player-facing settings (TOML)
│   │   │   ├── dev_config.rs   # Dev settings (RON) - debug only
│   │   │   ├── io.rs           # File I/O, paths, validation
│   │   │   └── appliers.rs     # Apply systems (graphics, audio)
│   │   │
│   │   ├── settings/           # Settings UI (uses config/)
│   │   │   ├── mod.rs
│   │   │   └── ui.rs           # Settings menu UI
│   │   └── ...
│   │
│   ├── game/
│   │   ├── constants.rs        # Game-specific constants (compile-time)
│   │   ├── config/             # 🆕 Game-specific config extensions
│   │   │   ├── mod.rs
│   │   │   └── gameplay.rs     # GameplayConfig (difficulty, physics)
│   │   └── ...
│   │
│   └── config/                 # 🔄 MOVE TO: src/framework/config/
│       └── game_config.rs      # (deprecated location)
│
├── .env.example                # Template for local dev secrets
├── .gitignore                  # Must include .env
└── Cargo.toml
```

### 2. Распределение ответственности

| Компонент | Слой | Формат | Назначение | Mutability |
|-----------|------|--------|------------|------------|
| **GameConfig** | Framework | TOML | Графика, звук, управление | Runtime (player) |
| **DevConfig** | Framework | RON | Дебаг флаги, gizmos, метрики | Runtime (dev only) |
| **GameplayConfig** | Game | RON/Code | Баланс игры, физика | Design-time |
| **Constants** | Game | Rust code | G, AU, скорости | Compile-time |
| **Secrets** | Outside repo | .env | API ключи | Build-time injection |

## 🎯 Предложения по улучшению

### Вопрос 1: DevConfig - Framework или Game?

**Моё мнение:** Разделить на два уровня:

```rust
// framework/config/dev_config.rs
#[cfg(debug_assertions)]
#[derive(Resource, Reflect, Debug, Clone)]
pub struct FrameworkDevConfig {
    pub show_fps: bool,
    pub show_state_debug: bool,
    pub hot_reload_assets: bool,
    pub ui_debug_borders: bool,
}

// game/config/dev_config.rs
#[cfg(debug_assertions)]
#[derive(Resource, Reflect, Debug, Clone)]
pub struct GameDevConfig {
    pub debug_physics: bool,           // Планетарные орбиты
    pub show_orbit_paths: bool,
    pub time_scale: f32,                // Ускорение симуляции
    pub spawn_debug_planets: bool,
}
```

**Преимущества:**

- Framework DevConfig переиспользуется в других проектах
- Game DevConfig специфичен для планетария
- Оба живут в `assets/` и hot-reload независимо

### Вопрос 2: Где хранить dev_config.ron?

**Рекомендация:**

```
assets/
├── config/
│   ├── framework_dev.ron       # Framework debug settings
│   └── game_dev.ron            # Game debug settings
├── textures/
└── ...
```

Bevy автоматически отслеживает изменения в `assets/` через AssetServer.

### Вопрос 3: GameplayConfig - код или файл?

Для **баланса игры** (сложность, физические параметры планет):

**Вариант A: Файл RON** (рекомендую)

```ron
// assets/config/gameplay.ron
(
    difficulty: Normal,
    physics: (
        gravitational_constant: 6.674e-11,
        time_step: 0.016,
        max_velocity: 1000.0,
    ),
    planets: (
        default_mass: 5.972e24,  // Earth mass
        min_radius: 100.0,
        max_radius: 10000.0,
    ),
)
```

**Преимущества:**

- Дизайнеры могут редактировать без пересборки
- Hot-reload в debug builds
- Легко создавать пресеты сложности

**Вариант B: Rust константы** (для неизменяемых значений)

```rust
// game/constants.rs
pub const GRAVITATIONAL_CONSTANT: f64 = 6.674e-11;
pub const ASTRONOMICAL_UNIT: f64 = 1.496e11;
pub const LIGHT_SPEED: f64 = 299_792_458.0;
```

**Мой совет:** Комбинируйте оба подхода:

- **Константы** - для физических констант (G, c, AU)
- **Config файлы** - для балансных параметров (масса планет, скорости)

## 📋 Обновлённая архитектура Config System

### Plugin структура

```rust
// framework/config/plugin.rs
pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<GameConfig>()     // Loaded from TOML
            
            // Debug-only resources
            #[cfg(debug_assertions)]
            .init_resource::<FrameworkDevConfig>()
            
            // Startup: Load configs
            .add_systems(Startup, (
                load_game_config,
                setup_config_watchers,
            ))
            
            // Update: Reactive sync
            .add_systems(Update, (
                save_config_on_change
                    .run_if(resource_changed::<GameConfig>),
                apply_graphics_settings
                    .run_if(resource_changed::<GameConfig>),
                apply_audio_settings
                    .run_if(resource_changed::<GameConfig>),
            ))
            
            // Debug systems
            #[cfg(debug_assertions)]
            .add_systems(Update, (
                hot_reload_dev_config,
                toggle_debug_overlays,
            ));
    }
}
```

### GameConfig структура (Framework)

```rust
// framework/config/game_config.rs
use serde::{Deserialize, Serialize};
use bevy::prelude::*;

#[derive(Resource, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct GameConfig {
    pub version: u32,
    pub graphics: GraphicsConfig,
    pub audio: AudioConfig,
    pub accessibility: AccessibilityConfig,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct GraphicsConfig {
    #[serde(default = "default_resolution")]
    pub resolution: [u32; 2],
    
    #[serde(default)]
    pub fullscreen: bool,
    
    #[serde(default = "default_true")]
    pub vsync: bool,
    
    #[serde(default)]
    pub quality: QualityPreset,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub enum QualityPreset {
    Low,
    #[default]
    Medium,
    High,
    Ultra,
}

// ... AudioConfig, AccessibilityConfig
```

### Applier Systems (Framework)

```rust
// framework/config/appliers.rs
use bevy::prelude::*;
use bevy::window::{Window, WindowMode};

pub fn apply_graphics_settings(
    config: Res<GameConfig>,
    mut windows: Query<&mut Window>,
) {
    if let Ok(mut window) = windows.get_single_mut() {
        let [width, height] = config.graphics.resolution;
        window.resolution.set(width as f32, height as f32);
        
        window.mode = if config.graphics.fullscreen {
            WindowMode::BorderlessFullscreen
        } else {
            WindowMode::Windowed
        };
        
        window.present_mode = if config.graphics.vsync {
            bevy::window::PresentMode::AutoVsync
        } else {
            bevy::window::PresentMode::AutoNoVsync
        };
    }
}

pub fn apply_audio_settings(
    config: Res<GameConfig>,
    // TODO: Integration with audio system
) {
    // Update global audio volume
    info!("Audio: Master={}, Music={}, SFX={}", 
        config.audio.master_volume,
        config.audio.music_volume,
        config.audio.sfx_volume
    );
}
```

## 🔐 Security Best Practices

### .env файл структура

```env
# .env.example - COMMIT THIS
# Copy to .env and fill with real values

# === Development Secrets (LOCAL ONLY) ===
STEAM_API_KEY="your_dev_key_here"
DISCORD_APP_ID="999999999999999999"

# === Analytics ===
ANALYTICS_ENDPOINT="https://dev.yourstudio.com/events"
ANALYTICS_API_KEY="dev_analytics_key"

# === CI/CD (Override in GitHub Actions) ===
RELEASE_SIGNING_KEY="path/to/key.p12"
```

### .gitignore обновление

```gitignore
# Environment variables
.env
.env.local

# User configs (never commit)
config.toml
*.toml.bak

# Build artifacts
/target/
```

## 🚀 Интеграция с существующей архитектурой

### Обновлённый main.rs

```rust
// src/main.rs
use bevy::prelude::*;

mod framework;
mod game;

fn main() {
    // Load secrets (dev only)
    #[cfg(debug_assertions)]
    dotenvy::dotenv().ok();
    
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Planetarium".into(),
                    // Will be overridden by GameConfig
                    resolution: (1280.0, 720.0).into(),
                    ..default()
                }),
                ..default()
            })
        )
        
        // Framework plugins
        .add_plugins(framework::FrameworkPlugin)
        
        // Game plugins
        .add_plugins(game::GamePlugin)
        
        .run();
}
```

### Framework Plugin с Config

```rust
// framework/plugin.rs
pub struct FrameworkPlugin;

impl Plugin for FrameworkPlugin {
    fn build(&self, app: &mut App) {
        app
            // Config FIRST - other plugins depend on it
            .add_plugins(config::ConfigPlugin)
            
            // Then other framework plugins
            .add_plugins((
                states::StatesPlugin,
                splash::SplashPlugin,
                // ...
            ));
    }
}
```

## 💡 Рекомендации и вопросы

### Вопросы для обсуждения

1. **Hot-reload GameConfig в продакшене?**
   - Моё мнение: НЕТ, только в debug builds
   - В релизе только load-on-startup

2. **Миграция config versions:**

   ```rust
   fn migrate_config(old: GameConfig) -> GameConfig {
       match old.version {
           1 => migrate_v1_to_v2(old),
           2 => old, // current
           _ => GameConfig::default(),
       }
   }
   ```

   Стоит ли добавить автоматическую миграцию?

3. **Validation стратегия:**

   ```rust
   impl GraphicsConfig {
       pub fn validate(&mut self) {
           self.resolution[0] = self.resolution[0].clamp(800, 7680);
           self.resolution[1] = self.resolution[1].clamp(600, 4320);
       }
   }
   ```

   Где лучше валидировать - при load или при apply?

4. **GameplayConfig расположение:**
   - `assets/config/gameplay.ron` - для hot-reload
   - `game/constants.rs` - для физических констант
   - Или всё в один файл?

### Что вы думаете?

Готов обновить документацию и схемы с учётом системы конфигурации. Хотите обсудить какой-то конкретный аспект подробнее?
