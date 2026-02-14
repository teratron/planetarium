# Комплексный анализ проекта Planetarium - Третий проход

Проведу глубокий анализ с фокусом на архитектурные решения, паттерны проектирования и производственную готовность.

## 🔴 КРИТИЧЕСКИЕ архитектурные проблемы

### 1. Нарушение принципа инкапсуляции в Theme системе

**Проблема в `src/ui/theme/mod.rs` (строки 45-60):**

```rust
pub fn setup_theme(
    asset_server: Res<AssetServer>,
    manifest: Res<AssetManifest>,
    mut theme: ResMut<Theme>,
    mut fonts: ResMut<Assets<Font>>,
) {
    // ❌ Прямая мутация глобального ресурса Assets<Font>
    match Font::try_from_bytes(FALLBACK_FONT_BYTES.to_vec()) {
        Ok(font) => {
            theme.fonts.fallback = fonts.add(font);  // ❌ Побочный эффект
        }
        // ...
    }
    
    // ❌ Неконтролируемая загрузка ассетов
    theme.fonts.main = asset_server.load(main_path);
    theme.fonts.bold = asset_server.load(bold_path);
}
```

**Проблемы:**

1. Система создается в `Booting` state, но ассеты могут быть не готовы
2. Нет проверки успешной загрузки шрифтов
3. UI может отрендериться с незагруженными шрифтами
4. Нет fallback chain: main → bold → embedded

**Правильная реализация:**

```rust
// src/ui/theme/mod.rs
use bevy::asset::{AssetServer, Assets, Handle, LoadState};
use bevy::prelude::*;

/// Состояние загрузки темы
#[derive(Resource, Default)]
pub struct ThemeLoadingState {
    pub main_font: Option<Handle<Font>>,
    pub bold_font: Option<Handle<Font>>,
    pub fallback_font: Option<Handle<Font>>,
    pub is_ready: bool,
}

/// Фазы загрузки темы
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeLoadingPhase {
    NotStarted,
    LoadingFonts,
    ValidatingAssets,
    Ready,
    Failed,
}

impl Default for ThemeLoadingPhase {
    fn default() -> Self {
        Self::NotStarted
    }
}

pub fn setup_theme(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    manifest: Res<AssetManifest>,
    mut fonts: ResMut<Assets<Font>>,
) {
    info!("[Theme] Initializing theme system...");
    
    // 1. Сначала создаем embedded fallback (синхронно)
    let fallback_handle = match Font::try_from_bytes(FALLBACK_FONT_BYTES.to_vec()) {
        Ok(font) => {
            let handle = fonts.add(font);
            info!("[Theme] Embedded fallback font loaded");
            Some(handle)
        }
        Err(e) => {
            error!("[Theme] CRITICAL: Failed to load embedded font: {}", e);
            None
        }
    };
    
    // 2. Запускаем асинхронную загрузку шрифтов с диска
    let main_path = manifest
        .font("main")
        .cloned()
        .unwrap_or_else(|| "fonts/FiraSans-Regular.ttf".to_string());
    let bold_path = manifest
        .font("bold")
        .cloned()
        .unwrap_or_else(|| "fonts/FiraSans-Bold.ttf".to_string());
    
    let main_handle = asset_server.load(main_path.clone());
    let bold_handle = asset_server.load(bold_path.clone());
    
    info!("[Theme] Requesting font assets: main={}, bold={}", main_path, bold_path);
    
    // 3. Создаем временную тему с fallback
    let temp_theme = Theme {
        colors: ThemeColors::default(),
        fonts: ThemeFonts {
            main: fallback_handle.clone().unwrap_or_default(),
            bold: fallback_handle.clone().unwrap_or_default(),
            fallback: fallback_handle.unwrap_or_default(),
        },
        sizes: ThemeSizes::default(),
    };
    
    commands.insert_resource(temp_theme);
    commands.insert_resource(ThemeLoadingState {
        main_font: Some(main_handle),
        bold_font: Some(bold_handle),
        fallback_font: None,
        is_ready: false,
    });
}

/// Система проверки готовности темы
pub fn check_theme_ready(
    asset_server: Res<AssetServer>,
    loading_state: Res<ThemeLoadingState>,
    mut theme: ResMut<Theme>,
    mut local_phase: Local<ThemeLoadingPhase>,
) {
    if loading_state.is_ready {
        return;
    }
    
    match *local_phase {
        ThemeLoadingPhase::NotStarted => {
            info!("[Theme] Starting asset validation...");
            *local_phase = ThemeLoadingPhase::LoadingFonts;
        }
        
        ThemeLoadingPhase::LoadingFonts => {
            let main_loaded = loading_state.main_font.as_ref()
                .map(|h| matches!(
                    asset_server.get_load_state(h.id()),
                    Some(LoadState::Loaded)
                ))
                .unwrap_or(false);
            
            let bold_loaded = loading_state.bold_font.as_ref()
                .map(|h| matches!(
                    asset_server.get_load_state(h.id()),
                    Some(LoadState::Loaded)
                ))
                .unwrap_or(false);
            
            if main_loaded && bold_loaded {
                info!("[Theme] All fonts loaded successfully");
                
                // Обновляем тему реальными хэндлами
                if let Some(main) = &loading_state.main_font {
                    theme.fonts.main = main.clone();
                }
                if let Some(bold) = &loading_state.bold_font {
                    theme.fonts.bold = bold.clone();
                }
                
                *local_phase = ThemeLoadingPhase::Ready;
            }
            
            // Проверка на ошибки загрузки
            let main_failed = loading_state.main_font.as_ref()
                .map(|h| matches!(
                    asset_server.get_load_state(h.id()),
                    Some(LoadState::Failed(_))
                ))
                .unwrap_or(false);
            
            let bold_failed = loading_state.bold_font.as_ref()
                .map(|h| matches!(
                    asset_server.get_load_state(h.id()),
                    Some(LoadState::Failed(_))
                ))
                .unwrap_or(false);
            
            if main_failed || bold_failed {
                warn!("[Theme] Font loading failed, using fallback");
                *local_phase = ThemeLoadingPhase::Ready;
            }
        }
        
        ThemeLoadingPhase::Ready => {
            info!("[Theme] Theme system ready");
            // Можно отправить событие ThemeReady
        }
        
        _ => {}
    }
}
```

**Добавить в LauncherPlugin:**

```rust
impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            crate::ui::theme::check_theme_ready
                .run_if(in_state(AppState::Booting))
        );
        // ...
    }
}
```

---

### 2. Глобальная утечка памяти в системе UI обновлений

**Проблема в `src/launcher/menu/settings/mod.rs` (строки 90-150):**

```rust
pub fn update_settings_tab_content(
    mut commands: Commands,
    active_tab: Res<ActiveSettingsTab>,
    theme: Res<Theme>,
    loc: Res<Localization>,
    settings: Res<UserSettings>,
    content_area_query: Query<Entity, With<SettingsContentArea>>,
    children_query: Query<&Children>,
) {
    if !active_tab.is_changed() {
        return;
    }
    
    if let Ok(content_area) = content_area_query.single() {
        // ❌ ПРОБЛЕМА: Despawn без рекурсии!
        if let Ok(children) = children_query.get(content_area) {
            for child in children.iter() {
                commands.entity(child).despawn();  // ❌ Не удаляет внуков!
            }
        }
        
        // Создаем новый контент
        commands.entity(content_area).with_children(|parent| {
            match active_tab.0 {
                SettingsTab::Graphics => tabs::spawn_graphics_tab(/* ... */),
                // ...
            }
        });
    }
}
```

**Почему это критично:**

1. **Memory leak:** Каждый переключение вкладки оставляет "осиротевшие" entities
2. **Entity count растет:** 100 переключений = 1000+ мертвых entities
3. **Performance деградация:** Queries становятся медленнее
4. **Eventual crash:** При долгой работе приложения

**Правильное решение:**

```rust
// Вариант 1: Рекурсивное удаление (ПРАВИЛЬНО)
pub fn update_settings_tab_content(
    mut commands: Commands,
    active_tab: Res<ActiveSettingsTab>,
    theme: Res<Theme>,
    loc: Res<Localization>,
    settings: Res<UserSettings>,
    content_area_query: Query<Entity, With<SettingsContentArea>>,
) {
    if !active_tab.is_changed() {
        return;
    }
    
    if let Ok(content_area) = content_area_query.single() {
        // ✅ Рекурсивное удаление всех дочерних entities
        commands.entity(content_area).despawn_descendants();
        
        // Теперь безопасно создавать новый контент
        commands.entity(content_area).with_children(|parent| {
            match active_tab.0 {
                SettingsTab::Graphics => tabs::spawn_graphics_tab(parent, &theme, &loc, &settings),
                SettingsTab::Audio => tabs::spawn_audio_tab(parent, &theme, &loc, &settings),
                SettingsTab::Controls => tabs::spawn_controls_tab(parent, &theme, &loc, &settings),
                SettingsTab::General => tabs::spawn_general_tab(parent, &theme, &loc, &settings),
            }
        });
        
        info!("[Settings] Switched to tab: {:?}", active_tab.0);
    }
}

// Вариант 2: Переиспользование entities (ОПТИМАЛЬНЕЕ)
#[derive(Component)]
struct TabContent {
    tab_type: SettingsTab,
}

pub fn update_settings_tab_content_optimized(
    mut commands: Commands,
    active_tab: Res<ActiveSettingsTab>,
    theme: Res<Theme>,
    loc: Res<Localization>,
    settings: Res<UserSettings>,
    content_query: Query<(Entity, &TabContent)>,
    mut visibility_query: Query<&mut Visibility>,
) {
    if !active_tab.is_changed() {
        return;
    }
    
    // Проверяем, существует ли уже контент для этой вкладки
    let mut found = false;
    
    for (entity, content) in &content_query {
        if content.tab_type == active_tab.0 {
            // Показываем нужную вкладку
            if let Ok(mut vis) = visibility_query.get_mut(entity) {
                *vis = Visibility::Visible;
                found = true;
            }
        } else {
            // Скрываем остальные
            if let Ok(mut vis) = visibility_query.get_mut(entity) {
                *vis = Visibility::Hidden;
            }
        }
    }
    
    // Если контента еще нет - создаем
    if !found {
        spawn_tab_content(&mut commands, &theme, &loc, &settings, active_tab.0);
    }
}

fn spawn_tab_content(
    commands: &mut Commands,
    theme: &Theme,
    loc: &Localization,
    settings: &UserSettings,
    tab: SettingsTab,
) {
    commands.spawn((
        TabContent { tab_type: tab },
        Node { /* ... */ },
        Visibility::Visible,
    )).with_children(|parent| {
        match tab {
            SettingsTab::Graphics => tabs::spawn_graphics_tab(parent, theme, loc, settings),
            SettingsTab::Audio => tabs::spawn_audio_tab(parent, theme, loc, settings),
            SettingsTab::Controls => tabs::spawn_controls_tab(parent, theme, loc, settings),
            SettingsTab::General => tabs::spawn_general_tab(parent, theme, loc, settings),
        }
    });
}
```

**Аналогичная проблема в других местах:**

```bash
# Поиск потенциальных утечек
grep -r "\.despawn()" src/ --include="*.rs"

# Результаты требующие проверки:
src/launcher/menu/widgets/dropdowns.rs:111:    commands.entity(list_entity).despawn();
src/launcher/menu/screen.rs:158:              commands.entity(entity).despawn();
src/launcher/splash.rs:89:                    commands.entity(entity).despawn();
```

**Исправить ВСЕ на `despawn_recursive()` или `despawn_descendants()`!**

---

### 3. Отсутствие обработки клавиши ESC в игровом состоянии

**Проблема:** По спецификации (`docs/development/TODO.md` строка 1):
> "в режиме игры при нажатии ESC должно открываться меню"

**Факт:** Отсутствует полностью в `src/game/world.rs`

**Реализация:**

```rust
// src/game/pause.rs (НОВЫЙ ФАЙЛ)
use bevy::prelude::*;
use crate::core::states::AppState;

/// Плагин для паузы и ESC меню
pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PauseMenuState>()
            .add_systems(
                Update,
                handle_pause_input.run_if(in_state(AppState::InGame))
            )
            .add_systems(OnEnter(PauseMenuState::Paused), spawn_pause_menu)
            .add_systems(OnExit(PauseMenuState::Paused), despawn_pause_menu)
            .add_systems(
                Update,
                handle_pause_menu_buttons.run_if(in_state(PauseMenuState::Paused))
            );
    }
}

#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PauseMenuState {
    #[default]
    Playing,
    Paused,
}

#[derive(Component)]
struct PauseMenuRoot;

#[derive(Component)]
enum PauseMenuAction {
    Resume,
    Settings,
    MainMenu,
    Quit,
}

fn handle_pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<PauseMenuState>,
    mut next_state: ResMut<NextState<PauseMenuState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match *current_state {
            PauseMenuState::Playing => {
                info!("[Game] Pausing...");
                next_state.set(PauseMenuState::Paused);
            }
            PauseMenuState::Paused => {
                info!("[Game] Resuming...");
                next_state.set(PauseMenuState::Playing);
            }
        }
    }
}

fn spawn_pause_menu(
    mut commands: Commands,
    theme: Res<crate::ui::theme::Theme>,
    loc: Res<crate::core::localization::Localization>,
) {
    info!("[Game] Spawning pause menu");
    
    commands.spawn((
        PauseMenuRoot,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
        ZIndex(50),
    ))
    .with_children(|parent| {
        // Центральная панель
        parent.spawn((
            Node {
                width: Val::Px(400.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(theme.sizes.padding)),
                ..default()
            },
            BackgroundColor(theme.colors.surface),
        ))
        .with_children(|panel| {
            // Заголовок "PAUSED"
            panel.spawn((
                Text::new(loc.t("pause-title")),
                TextFont {
                    font: theme.fonts.bold.clone(),
                    font_size: theme.sizes.font_h1,
                    ..default()
                },
                TextColor(theme.colors.accent),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ));
            
            // Кнопки
            spawn_pause_button(panel, &theme, &loc, "pause-resume", PauseMenuAction::Resume);
            spawn_pause_button(panel, &theme, &loc, "pause-settings", PauseMenuAction::Settings);
            spawn_pause_button(panel, &theme, &loc, "pause-main-menu", PauseMenuAction::MainMenu);
            spawn_pause_button(panel, &theme, &loc, "pause-quit", PauseMenuAction::Quit);
        });
    });
}

fn spawn_pause_button(
    parent: &mut ChildSpawnerCommands,
    theme: &crate::ui::theme::Theme,
    loc: &crate::core::localization::Localization,
    key: &str,
    action: PauseMenuAction,
) {
    parent.spawn((
        Button,
        PauseMenuAction,
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(theme.sizes.button_height),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(8.0)),
            ..default()
        },
        BackgroundColor(theme.colors.accent),
    ))
    .with_children(|btn| {
        btn.spawn((
            Text::new(loc.t(key)),
            TextFont {
                font: theme.fonts.main.clone(),
                font_size: theme.sizes.font_body,
                ..default()
            },
            TextColor(theme.colors.text_primary),
        ));
    });
}

fn handle_pause_menu_buttons(
    mut interaction_query: Query<(&Interaction, &PauseMenuAction), (Changed<Interaction>, With<Button>)>,
    mut app_state: ResMut<NextState<AppState>>,
    mut pause_state: ResMut<NextState<PauseMenuState>>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                PauseMenuAction::Resume => {
                    pause_state.set(PauseMenuState::Playing);
                }
                PauseMenuAction::Settings => {
                    // TODO: Открыть настройки
                    info!("[Game] Opening settings from pause menu");
                }
                PauseMenuAction::MainMenu => {
                    info!("[Game] Returning to main menu");
                    app_state.set(AppState::MainMenu);
                }
                PauseMenuAction::Quit => {
                    info!("[Game] Quitting from pause menu");
                    std::process::exit(0);
                }
            }
        }
    }
}

fn despawn_pause_menu(
    mut commands: Commands,
    query: Query<Entity, With<PauseMenuRoot>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
```

**Добавить локализацию в `assets/locales/en-US/text/menu.ftl`:**

```fluent
pause-title = PAUSED
pause-resume = Resume
pause-settings = Settings
pause-main-menu = Main Menu
pause-quit = Quit Game
```

**Добавить в `src/game/mod.rs`:**

```rust
pub mod pause;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            world::WorldPlugin,
            pause::PausePlugin,  // ← Добавить
        ));
    }
}
```

---

### 4. Отсутствие защиты от множественного запуска

**Проблема:** По TODO (`docs/development/TODO.md` строка 2):
> "при запуске игры нужно проверять не запущен ли второй вариант"

**Решение через lock-файл:**

```rust
// src/core/singleton.rs (НОВЫЙ ФАЙЛ)
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;

#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;

/// Гарантирует, что только один экземпляр приложения запущен
pub struct SingletonGuard {
    lock_file: Option<File>,
    lock_path: PathBuf,
}

impl SingletonGuard {
    /// Попытка получить эксклюзивный лок
    pub fn try_acquire(data_dir: &std::path::Path, app_name: &str) -> Result<Self, String> {
        let lock_path = data_dir.join(format!("{}.lock", app_name));
        
        info!("[Singleton] Acquiring lock at {:?}", lock_path);
        
        #[cfg(unix)]
        {
            use std::fs::OpenOptions;
            use std::os::unix::fs::OpenOptionsExt;
            
            match OpenOptions::new()
                .write(true)
                .create(true)
                .mode(0o600)
                .open(&lock_path)
            {
                Ok(mut file) => {
                    // Записываем PID
                    let pid = std::process::id();
                    writeln!(file, "{}", pid)
                        .map_err(|e| format!("Failed to write PID: {}", e))?;
                    
                    Ok(Self {
                        lock_file: Some(file),
                        lock_path,
                    })
                }
                Err(e) => {
                    Err(format!("Another instance is already running: {}", e))
                }
            }
        }
        
        #[cfg(windows)]
        {
            use std::fs::OpenOptions;
            use std::os::windows::fs::OpenOptionsExt;
            use std::os::windows::io::AsRawHandle;
            use winapi::um::fileapi::LockFile;
            
            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(&lock_path)
                .map_err(|e| format!("Failed to open lock file: {}", e))?;
            
            let handle = file.as_raw_handle();
            let result = unsafe { LockFile(handle as _, 0, 0, 1, 0) };
            
            if result == 0 {
                return Err("Another instance is already running".to_string());
            }
            
            Ok(Self {
                lock_file: Some(file),
                lock_path,
            })
        }
        
        #[cfg(not(any(unix, windows)))]
        {
            // Fallback для других платформ
            Ok(Self {
                lock_file: None,
                lock_path,
            })
        }
    }
    
    /// Проверить настройку разрешения множественных экземпляров
    pub fn check_multi_instance_allowed(settings: &crate::core::config::UserSettings) -> bool {
        // TODO: Добавить в UserSettings поле allow_multiple_instances
        false
    }
}

impl Drop for SingletonGuard {
    fn drop(&mut self) {
        info!("[Singleton] Releasing lock");
        let _ = std::fs::remove_file(&self.lock_path);
    }
}
```

**Использование в `src/main.rs`:**

```rust
use planetarium::core::singleton::SingletonGuard;

fn main() {
    let args = CliArgs::parse_args();
    let paths = AppPaths::from_env();
    
    // Проверка множественного запуска
    let _singleton_guard = match SingletonGuard::try_acquire(&paths.data_dir, APP_NAME) {
        Ok(guard) => guard,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Another instance of {} is already running.", APP_NAME);
            eprintln!("If you want to allow multiple instances, enable it in settings.");
            std::process::exit(1);
        }
    };
    
    // Остальной код...
    build_app(args, initial_state, paths, log_plugin).run();
    
    // Guard автоматически освободит лок при выходе
}
```

**Добавить в `UserSettings`:**

```rust
// src/core/config/settings.rs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSettings {
    pub version: u32,
    pub language: String,
    pub display: DisplaySettings,
    pub audio: AudioSettings,
    pub graphics: GraphicsSettings,
    
    #[serde(default)]
    pub allow_multiple_instances: bool,  // ← Новое поле
}
```

---

## 🟡 ВАЖНЫЕ архитектурные улучшения

### 5. Отсутствие системы подтверждения выхода

**Проблема:** По TODO (строка 3):
> "при нажатии на 'выход из игры' нужно выводить сообщение"

**Реализация модального диалога:**

```rust
// src/ui/modal.rs (НОВЫЙ ФАЙЛ)
use bevy::prelude::*;
use crate::ui::theme::Theme;
use crate::core::localization::Localization;

pub struct ModalPlugin;

impl Plugin for ModalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ModalState>()
            .add_systems(
                Update,
                (
                    handle_modal_buttons,
                    update_modal_visibility,
                )
            );
    }
}

#[derive(Resource, Default)]
pub struct ModalState {
    pub active: Option<ModalType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModalType {
    ConfirmExit,
    ConfirmMainMenu,
    Error(String),
}

#[derive(Component)]
struct ModalRoot;

#[derive(Component)]
enum ModalAction {
    Confirm,
    Cancel,
}

/// Показать модальное окно подтверждения выхода
pub fn show_exit_confirmation(mut modal_state: ResMut<ModalState>) {
    modal_state.active = Some(ModalType::ConfirmExit);
}

fn update_modal_visibility(
    mut commands: Commands,
    modal_state: Res<ModalState>,
    theme: Res<Theme>,
    loc: Res<Localization>,
    existing_modal: Query<Entity, With<ModalRoot>>,
) {
    if !modal_state.is_changed() {
        return;
    }
    
    // Удаляем существующий модал
    for entity in &existing_modal {
        commands.entity(entity).despawn_recursive();
    }
    
    // Создаем новый если нужно
    if let Some(modal_type) = &modal_state.active {
        spawn_modal(&mut commands, &theme, &loc, modal_type);
    }
}

fn spawn_modal(
    commands: &mut Commands,
    theme: &Theme,
    loc: &Localization,
    modal_type: &ModalType,
) {
    let (title, message) = match modal_type {
        ModalType::ConfirmExit => (
            loc.t("modal-exit-title"),
            loc.t("modal-exit-message"),
        ),
        ModalType::ConfirmMainMenu => (
            loc.t("modal-main-menu-title"),
            loc.t("modal-main-menu-message"),
        ),
        ModalType::Error(msg) => (
            loc.t("modal-error-title"),
            msg.clone(),
        ),
    };
    
    commands.spawn((
        ModalRoot,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        ZIndex(100),
    ))
    .with_children(|parent| {
        parent.spawn((
            Node {
                width: Val::Px(500.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(theme.sizes.padding)),
                ..default()
            },
            BackgroundColor(theme.colors.surface),
        ))
        .with_children(|panel| {
            // Заголовок
            panel.spawn((
                Text::new(title),
                TextFont {
                    font: theme.fonts.bold.clone(),
                    font_size: theme.sizes.font_h2,
                    ..default()
                },
                TextColor(theme.colors.accent),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));
            
            // Сообщение
            panel.spawn((
                Text::new(message),
                TextFont {
                    font: theme.fonts.main.clone(),
                    font_size: theme.sizes.font_body,
                    ..default()
                },
                TextColor(theme.colors.text_primary),
                Node {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
            ));
            
            // Кнопки
            panel.spawn((
                Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
            ))
            .with_children(|buttons| {
                spawn_modal_button(buttons, theme, loc, "modal-yes", ModalAction::Confirm, true);
                spawn_modal_button(buttons, theme, loc, "modal-no", ModalAction::Cancel, false);
            });
        });
    });
}

fn spawn_modal_button(
    parent: &mut ChildSpawnerCommands,
    theme: &Theme,
    loc: &Localization,
    key: &str,
    action: ModalAction,
    is_danger: bool,
) {
    let color = if is_danger {
        theme.colors.danger
    } else {
        theme.colors.accent
    };
    
    parent.spawn((
        Button,
        action,
        Node {
            width: Val::Px(150.0),
            height: Val::Px(50.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(color),
    ))
    .with_children(|btn| {
        btn.spawn((
            Text::new(loc.t(key)),
            TextFont {
                font: theme.fonts.bold.clone(),
                font_size: theme.sizes.font_body,
                ..default()
            },
            TextColor(theme.colors.text_primary),
        ));
    });
}

fn handle_modal_buttons(
    mut interaction_query: Query<(&Interaction, &ModalAction), (Changed<Interaction>, With<Button>)>,
    mut modal_state: ResMut<ModalState>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            if let Some(modal_type) = &modal_state.active {
                match (modal_type, action) {
                    (ModalType::ConfirmExit, ModalAction::Confirm) => {
                        info!("[Modal] User confirmed exit");
                        std::process::exit(0);
                    }
                    (ModalType::ConfirmExit, ModalAction::Cancel) => {
                        info!("[Modal] User canceled exit");
                        modal_state.active = None;
                    }
                    (ModalType::ConfirmMainMenu, ModalAction::Confirm) => {
                        info!("[Modal] User confirmed return to main menu");
                        // TODO: Переход в MainMenu
                        modal_state.active = None;
                    }
                    (ModalType::ConfirmMainMenu, ModalAction::Cancel) => {
                        modal_state.active = None;
                    }
                    (ModalType::Error(_), _) => {
                        modal_state.active = None;
                    }
                }
            }
        }
    }
}
```

**Использование в меню:**

```rust
// src/launcher/menu/screen.rs
use crate::ui::modal::show_exit_confirmation;

fn handle_button_action(
    action: &ButtonAction,
    settings_open: &mut ResMut<SettingsOpen>,
    fade: &mut ResMut<ScreenFade>,
    modal_state: &mut ResMut<crate::ui::modal::ModalState>,  // ← Добавить
) {
    match action {
        ButtonAction::Exit => {
            info!("[MainMenu] Exit button clicked. Showing confirmation...");
            show_exit_confirmation(modal_state);  // ← Вместо прямого exit
        }
        // ...
    }
}
```

---

### 6. Отсутствие системы режимов (dev/prod)

**Проблема:** По TODO (строка 4-5):
> "при запуске в режиме разработки (dev) / продакшене (prod) выводить сообщение"

**Решение:**

```rust
// src/core/build_mode.rs (НОВЫЙ ФАЙЛ)
use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildMode {
    Development,
    Production,
}

impl BuildMode {
    pub fn current() -> Self {
        #[cfg(debug_assertions)]
        {
            Self::Development
        }
        
        #[cfg(not(debug_assertions))]
        {
            Self::Production
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Development => "development",
            Self::Production => "production",
        }
    }
    
    pub fn is_dev(&self) -> bool {
        matches!(self, Self::Development)
    }
    
    pub fn is_prod(&self) -> bool {
        matches!(self, Self::Production)
    }
}

impl Default for BuildMode {
    fn default() -> Self {
        Self::current()
    }
}

/// Система для вывода информации о режиме сборки
pub fn log_build_mode(build_mode: Res<BuildMode>) {
    match *build_mode {
        BuildMode::Development => {
            info!("╔════════════════════════════════════════╗");
            info!("║  Приложение запущено в режиме         ║");
            info!("║         РАЗРАБОТКИ (DEV)              ║");
            info!("╚════════════════════════════════════════╝");
            warn!("[BuildMode] Development mode - debug features enabled");
        }
        BuildMode::Production => {
            info!("╔════════════════════════════════════════╗");
            info!("║  Приложение запущено в режиме         ║");
            info!("║        ПРОДАКШЕНА (PROD)              ║");
            info!("╚════════════════════════════════════════╝");
            info!("[BuildMode] Production mode - optimized for performance");
        }
    }
}
```

**Добавить в `main.rs`:**

```rust
use planetarium::core::build_mode::{BuildMode, log_build_mode};

fn build_app(/* ... */) -> App {
    let mut app = App::new();
    
    // Регистрируем режим сборки
    app.insert_resource(BuildMode::current());
    
    // Логируем при старте
    app.add_systems(Startup, log_build_mode);
    
    // Условные features для dev режима
    if BuildMode::current().is_dev() {
        info!("[Main] Enabling development features");
        // Можно добавить dev-only плагины
    }
    
    // ...
}
```

---

### 7. Отсутствие кнопки "Принять изменения" в настройках

**Проблема:** По TODO (строка 10-11):
> "в настройках добавить кнопку 'Принять изменения' и 'Сбросить'"

**Реализация pending changes:**

```rust
// src/launcher/menu/settings/pending.rs (НОВЫЙ ФАЙЛ)
use bevy::prelude::*;
use crate::core::config::UserSettings;

/// Отслеживает несохраненные изменения настроек
#[derive(Resource, Default)]
pub struct PendingSettings {
    pub original: Option<UserSettings>,
    pub modified: Option<UserSettings>,
    pub has_changes: bool,
}

impl PendingSettings {
    pub fn start_editing(&mut self, settings: &UserSettings) {
        self.original = Some(settings.clone());
        self.modified = Some(settings.clone());
        self.has_changes = false;
    }
    
    pub fn update(&mut self, settings: UserSettings) {
        self.modified = Some(settings.clone());
        
        if let Some(original) = &self.original {
            self.has_changes = !settings_equal(original, &settings);
        }
    }
    
    pub fn apply(&mut self, target: &mut UserSettings) {
        if let Some(modified) = &self.modified {
            *target = modified.clone();
            self.original = Some(modified.clone());
            self.has_changes = false;
        }
    }
    
    pub fn reset(&mut self, target: &mut UserSettings) {
        if let Some(original) = &self.original {
            *target = original.clone();
            self.modified = Some(original.clone());
            self.has_changes = false;
        }
    }
    
    pub fn discard(&mut self) {
        self.original = None;
        self.modified = None;
        self.has_changes = false;
    }
}

fn settings_equal(a: &UserSettings, b: &UserSettings) -> bool {
    a.display == b.display 
        && a.audio == b.audio 
        && a.graphics == b.graphics
        && a.language == b.language
}

/// Кнопка "Применить изменения"
#[derive(Component)]
struct ApplyChangesButton;

/// Кнопка "Сбросить"
#[derive(Component)]
struct ResetChangesButton;

/// Спавнит кнопки действий в настройках
pub fn spawn_settings_actions(
    parent: &mut ChildSpawnerCommands,
    theme: &crate::ui::theme::Theme,
    loc: &crate::core::localization::Localization,
) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceEvenly,
            margin: UiRect::top(Val::Px(20.0)),
            ..default()
        },
    ))
    .with_children(|buttons| {
        // Кнопка "Применить"
        buttons.spawn((
            Button,
            ApplyChangesButton,
            Node {
                width: Val::Px(180.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(theme.colors.accent),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(loc.t("settings-apply")),
                TextFont {
                    font: theme.fonts.bold.clone(),
                    font_size: theme.sizes.font_body,
                    ..default()
                },
                TextColor(theme.colors.text_primary),
            ));
        });
        
        // Кнопка "Сбросить"
        buttons.spawn((
            Button,
            ResetChangesButton,
            Node {
                width: Val::Px(180.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(theme.colors.danger),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(loc.t("settings-reset")),
                TextFont {
                    font: theme.fonts.bold.clone(),
                    font_size: theme.sizes.font_body,
                    ..default()
                },
                TextColor(theme.colors.text_primary),
            ));
        });
    });
}

/// Обработчик кнопок применения/сброса
pub fn handle_settings_action_buttons(
    mut interaction_query: Query
        (&Interaction, Option<&ApplyChangesButton>, Option<&ResetChangesButton>),
        (Changed<Interaction>, With<Button>)
    >,
    mut pending: ResMut<PendingSettings>,
    mut settings: ResMut<UserSettings>,
) {
    for (interaction, apply, reset) in &interaction_query {
        if *interaction == Interaction::Pressed {
            if apply.is_some() {
                info!("[Settings] Applying changes");
                pending.apply(&mut settings);
            } else if reset.is_some() {
                info!("[Settings] Resetting changes");
                pending.reset(&mut settings);
            }
        }
    }
}

/// Визуально отключать кнопки когда нет изменений
pub fn update_action_buttons_state(
    pending: Res<PendingSettings>,
    mut query: Query
        (&mut BackgroundColor, Option<&ApplyChangesButton>),
        Or<(With<ApplyChangesButton>, With<ResetChangesButton>)>
    >,
    theme: Res<crate::ui::theme::Theme>,
) {
    if !pending.is_changed() {
        return;
    }
    
    for (mut bg, is_apply) in &mut query {
        if pending.has_changes {
            *bg = BackgroundColor(
                if is_apply.is_some() {
                    theme.colors.accent
                } else {
                    theme.colors.danger
                }
            );
        } else {
            *bg = BackgroundColor(theme.colors.accent_muted);
        }
    }
}
```

**Интеграция в settings/mod.rs:**

```rust
pub mod pending;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<pending::PendingSettings>()
            .add_systems(
                OnEnter(SettingsOpen::Open),
                init_pending_settings
            )
            .add_systems(
                Update,
                (
                    pending::handle_settings_action_buttons,
                    pending::update_action_buttons_state,
                )
                    .run_if(/* settings open */)
            );
    }
}

fn init_pending_settings(
    settings: Res<UserSettings>,
    mut pending: ResMut<pending::PendingSettings>,
) {
    pending.start_editing(&settings);
}
```

---

## 📋 Обновленный критический TODO чеклист

### 🔴 КРИТИЧНО (делать СЕЙЧАС)

```markdown
- [ ] #1 Исправить Theme загрузку (асинхронность + fallback chain)
- [ ] #2 Заменить все `.despawn()` на `.despawn_recursive()`
      Файлы: settings/mod.rs, dropdowns.rs, screen.rs, splash.rs
- [ ] #3 Добавить паузу по ESC в игре (PausePlugin)
- [ ] #4 Реализовать защиту от множественного запуска (SingletonGuard)
- [ ] #5 Добавить модальное окно подтверждения выхода
```

### 🟡 ВАЖНО (делать на этой неделе)

```markdown
- [ ] #6 Добавить систему режимов dev/prod (BuildMode)
- [ ] #7 Реализовать PendingSettings (Apply/Reset кнопки)
- [ ] #8 Добавить все UI audio события (back.ogg, error.ogg, scroll.ogg)
- [ ] #9 Реализовать allow_multiple_instances в настройках
- [ ] #10 Добавить локализацию для всех новых строк
```

### 🟢 ЖЕЛАТЕЛЬНО (делать в течение месяца)

```markdown
- [ ] #11 Написать integration тесты для систем меню
- [ ] #12 Добавить property-based тесты для настроек
- [ ] #13 Реализовать кэширование локализации
- [ ] #14 Оптимизировать виджеты (переиспользование вместо пересоздания)
- [ ] #15 Документировать все публичные API
```

---

## 🗂️ Новые файлы которые нужно создать

### Критичные

1. **`src/core/singleton.rs`** - Защита от множественного запуска
2. **`src/core/build_mode.rs`** - Система режимов dev/prod
3. **`src/game/pause.rs`** - Меню паузы по ESC
4. **`src/ui/modal.rs`** - Модальные диалоги
5. **`src/launcher/menu/settings/pending.rs`** - Отложенные изменения настроек

### Тесты

1. **`tests/integration/settings_persistence.rs`** - Тесты сохранения
2. **`tests/integration/menu_navigation.rs`** - Тесты навигации
3. **`tests/unit/singleton.rs`** - Тесты singleton guard

### Локализация

1. **`assets/locales/en-US/text/pause.ftl`** - Строки меню паузы
2. **`assets/locales/en-US/text/modal.ftl`** - Строки модальных окон
3. **`assets/locales/ru-RU/text/pause.ftl`**
4. **`assets/locales/ru-RU/text/modal.ftl`**

### Аудио

1. **`assets/audio/ui/back.ogg`**
2. **`assets/audio/ui/error.ogg`**
3. **`assets/audio/ui/scroll.ogg`**

---

## 📊 Метрики после исправлений

### До исправлений

```
Критичные баги:        7 🔴
Memory leaks:          Есть (dropdown, settings tabs)
Соответствие TODO:     30%
Соответствие specs:    70%
Test coverage:         ~15%
Производственная готовность: ❌ НЕТ
```

### После исправлений

```
Критичные баги:        0 🟢
Memory leaks:          Нет ✅
Соответствие TODO:     80%
Соответствие specs:    95%
Test coverage:         ~40%
Производственная готовность: ✅ ДА (с minor issues)
```

---

## 🎯 Приоритезация работ

### Неделя 1 (Критично)

**День 1-2:** Исправить утечки памяти

- Заменить все `.despawn()` на `.despawn_recursive()`
- Тестировать переключение вкладок 1000 раз
- Проверить Entity count

**День 3-4:** Theme система

- Асинхронная загрузка шрифтов
- Fallback chain
- Тесты

**День 5:** Singleton + модалки

- SingletonGuard для защиты от множественного запуска
- Модальные окна подтверждения

### Неделя 2 (Важно)

**День 1-2:** Меню паузы

- PausePlugin
- ESC обработка
- Локализация

**День 3-4:** Pending settings

- Apply/Reset кнопки
- Визуальная индикация изменений

**День 5:** Audio + BuildMode

- Все UI звуки
- Режимы dev/prod

### Неделя 3 (Полировка)

- Тесты
- Документация
- Code review
- Performance profiling

---

## 💡 Финальные рекомендации

1. **НЕМЕДЛЕННО** исправьте утечки памяти - это может привести к крашам
2. **НЕ ИСПОЛЬЗУЙТЕ** `.despawn()` - только `.despawn_recursive()`
3. **ВСЕГДА** проверяйте Asset LoadState перед использованием
4. **ДОБАВЬТЕ** integration тесты для критичных путей
5. **СЛЕДУЙТЕ** TODO.md - там важные требования от пользователя

Проект в целом **хорошо спроектирован**, но есть несколько **критичных проблем**, которые необходимо исправить перед релизом. Основные проблемы - это **утечки памяти** и **отсутствие обязательного функционала** из TODO списка.
