# Configuration Files Specification

Для игрового проекта на **Rust + Bevy** оптимальный выбор — **`config.toml`** с комбинацией инструментов. Вот детальное сравнение и рекомендации:

## 📊 Сравнение форматов для игр

| Формат | Плюсы | Минусы | Подходит для игр? |
| ------ | ----- | ------ | ----------------- |
| **`.env`** | Простота, секреты | Нет типов, нет вложенности, не для игроков | ❌ Только для CI/деплоя (не для игровых настроек) |
| **`config.json`** | Универсальность, парсится везде | Нет комментариев, строгий синтаксис, легко сломать | ⚠️ Только для автоматических сохранений (не для ручного редактирования) |
| **`config.rs`** | Типобезопасность, компиляция | Требует пересборки, игрок не может менять | ❌ Только для *дефолтных* значений в коде |
| **`config.toml`** | Комментарии, вложенность, человекочитаемость, нативный для экосистемы Rust | Чуть медленнее парсится (но не критично) | ✅ **Идеален для основного конфига** |
| **`config.ron`** | Нативный для Bevy, поддержка всех типов Rust | Менее знаком игрокам, нет комментариев в старых версиях | ✅ Отличен для *сохранений* и *отладки* |

## 🏆 Рекомендуемая архитектура (гибридная)

```plaintext
project/
├── assets/
│   └── config/
│       └── default_config.toml    ← ДЕФОЛТНЫЕ значения (в репозитории)
├── src/
│   ├── config.rs                  ← Логика загрузки + структуры
│   └── ...
└── (игнорируется в .gitignore)
    %APPDATA%/MyGame/config.toml   ← ПЕРЕОПРЕДЕЛЕНИЯ игроком
```

### Почему именно так

1. **`default_config.toml`** — хранится в репозитории, содержит безопасные дефолты
2. **`user config.toml`** — создаётся в `AppData`/`~/.config`, перекрывает дефолты
3. **`config.rs`** — содержит *только структуры и логику*, не сами значения

## 💻 Пример реализации (Rust + Bevy)

### 1. `Cargo.toml` — зависимости

```toml
[dependencies]
bevy = "0.18"
serde = { version = "1.0.228", features = ["derive"] }
toml = "0.9.11"
directories = "6.0"  # Кроссплатформенные пути к данным пользователя
```

### 2. `src/config.rs` — структуры и логика

```rust
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicsConfig {
    pub resolution: (u32, u32),
    pub fullscreen: bool,
    pub vsync: bool,
    pub quality: GraphicsQuality,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GraphicsQuality {
    Low,
    Medium,
    High,
    Ultra,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub graphics: GraphicsConfig,
    pub audio: AudioConfig,
    pub language: String,
    pub enable_telemetry: bool,
}

// Дефолтные значения (без хардкода в файле!)
impl Default for GameConfig {
    fn default() -> Self {
        Self {
            graphics: GraphicsConfig {
                resolution: (1920, 1080),
                fullscreen: false,
                vsync: true,
                quality: GraphicsQuality::High,
            },
            audio: AudioConfig {
                master_volume: 1.0,
                music_volume: 0.7,
                sfx_volume: 0.8,
            },
            language: "en".to_string(),
            enable_telemetry: false,
        }
    }
}

// Система загрузки конфига при старте игры
pub fn load_config(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let user_config_path = get_user_config_path();
    
    let config = if user_config_path.exists() {
        // Загружаем пользовательский конфиг + мержим с дефолтами
        match fs::read_to_string(&user_config_path) {
            Ok(content) => {
                let mut cfg: GameConfig = toml::from_str(&content)
                    .unwrap_or_else(|e| {
                        warn!("Invalid user config ({}), falling back to defaults: {}", user_config_path.display(), e);
                        GameConfig::default()
                    });
                // Валидация границ
                cfg.audio.master_volume = cfg.audio.master_volume.clamp(0.0, 1.0);
                cfg
            }
            Err(e) => {
                warn!("Failed to read config ({}): {}", user_config_path.display(), e);
                GameConfig::default()
            }
        }
    } else {
        // Первый запуск — создаём конфиг с дефолтами
        let cfg = GameConfig::default();
        if let Ok(dir) = user_config_path.parent() {
            let _ = fs::create_dir_all(dir);
        }
        let _ = fs::write(&user_config_path, toml::to_string_pretty(&cfg).unwrap());
        info!("Created default config at {}", user_config_path.display());
        cfg
    };

    commands.insert_resource(config);
}

// Кроссплатформенный путь к конфигу
fn get_user_config_path() -> PathBuf {
    use directories::ProjectDirs;
    
    if let Some(proj_dirs) = ProjectDirs::from("com", "YourStudio", "YourGame") {
        proj_dirs.config_dir().join("config.toml")
    } else {
        // Fallback для портативных версий
        std::env::current_dir().unwrap().join("config.toml")
    }
}
```

### 3. `assets/config/default_config.toml` — пример содержимого

```toml
# ========================================
# GRAPHICS SETTINGS
# ========================================
[graphics]
resolution = [1920, 1080]  # Width, Height
fullscreen = false
vsync = true

# Quality preset: "Low" | "Medium" | "High" | "Ultra"
quality = "High"

# ========================================
# AUDIO SETTINGS
# ========================================
[audio]
master_volume = 1.0   # 0.0 to 1.0
music_volume = 0.7
sfx_volume = 0.8

# ========================================
# GAMEPLAY
# ========================================
language = "en"  # "en", "ru", "de", etc.
enable_telemetry = false  # Anonymous usage stats
```

## 🔐 Для чувствительных данных (если нужны)

- **Никогда не храните секреты в `config.toml`** — используйте:
  - `.env` + `dotenvy` — только для **разработки** (добавьте в `.gitignore`)
  - Системные переменные окружения — для продакшн-серверов
  - Платформенные хранилища (Windows Credential Manager, macOS Keychain)

```rust
// Пример безопасной загрузки токена (только для онлайн-сервисов)
fn load_auth_token() -> Option<String> {
    std::env::var("GAME_AUTH_TOKEN").ok()
        .or_else(|| std::fs::read_to_string("secrets.token").ok())
}
```

## 🚀 Продвинутые паттерны

### 1. Горячая перезагрузка конфига (для отладки)

```rust
fn hot_reload_config(
    mut config: ResMut<GameConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::F5) {
        if let Ok(content) = fs::read_to_string(get_user_config_path()) {
            if let Ok(new_cfg) = toml::from_str::<GameConfig>(&content) {
                *config = new_cfg;
                info!("Config reloaded!");
            }
        }
    }
}
```

### 2. RON для отладки (альтернатива)

```rust
// В режиме отладки можно дампить конфиг в RON для быстрой проверки
use ron::ser::to_string_pretty;

if cfg!(debug_assertions) {
    let ron = to_string_pretty(&*config, ron::ser::PrettyConfig::default()).unwrap();
    fs::write("debug_config.ron", ron).ok();
}
```

## ✅ Итоговые рекомендации

| Задача | Формат | Расположение |
| ------ | ------ | ------------ |
| Основные настройки игрока | **TOML** | `%APPDATA%/YourGame/config.toml` |
| Дефолтные значения | **Код (Default trait)** | `src/config.rs` |
| Сохранения игры | **RON** или **bincode** | `%APPDATA%/YourGame/saves/` |
| Секреты/токены | **Переменные окружения** | `.env` (только для разработки) |
| Конфигурация сборки | **`build.rs` + фичи** | `Cargo.toml` `[features]` |

> 💡 **Главное правило**: игрок должен иметь возможность отредактировать `config.toml` в Блокноте без риска сломать игру. Поэтому:
>
> - Всегда валидируйте значения при загрузке
> - Используйте дефолты как fallback
> - Добавляйте комментарии в пример конфига

Этот подход используется в реальных проектах на Bevy (например, *Fish Fight*, *Bevy Pong*) и соответствует философии Rust: **безопасность + гибкость + человекоцентричность**.
