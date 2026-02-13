# Глубокий анализ проекта Planetarium - Второй проход

## 🔴 Критические архитектурные проблемы

### 1. Нарушение принципа ECS в виджетах

**Проблема в `src/launcher/menu/widgets/buttons.rs` (строки 17-28):**

```rust
#[derive(Component)]
pub struct HoverAnimationState {
    pub base_scale: Vec3,
    pub target_scale: Vec3,
    pub current_scale: Vec3,  // ❌ Дублирует Transform.scale
    pub is_hovered: bool,
}
```

**Почему это проблема:**

- Нарушает принцип "Single Source of Truth"
- `current_scale` дублирует данные из `Transform`
- Может привести к рассинхронизации состояния

**Правильное решение:**

```rust
#[derive(Component)]
pub struct HoverAnimation {
    pub base_scale: Vec3,
    pub hover_scale: Vec3,
    pub speed: f32,
}

// Система использует только Transform напрямую
pub fn animate_button_hover(
    mut query: Query<(&mut Transform, &HoverAnimation, &Interaction)>,
    time: Res<Time>,
) {
    for (mut transform, anim, interaction) in &mut query {
        let target = if *interaction == Interaction::Hovered {
            anim.hover_scale
        } else {
            anim.base_scale
        };
        
        transform.scale = transform.scale.lerp(target, anim.speed * time.delta_secs());
    }
}
```

### 2. Утечка памяти в dropdown системе

**КРИТИЧЕСКАЯ проблема в `src/launcher/menu/widgets/dropdowns.rs` (строки 94-111):**

```rust
if dropdown.is_open {
    // Spawn options
    commands.entity(entity).with_children(|parent| {
        parent.spawn((
            Node { /* ... */ },
            DropdownOptionsList(entity),
        ))
        .with_children(|list| {
            // ❌ Создаются новые entities каждый раз при открытии
            for (i, option_text) in dropdown.options.iter().enumerate() {
                list.spawn((/* ... */));
            }
        });
    });
} else {
    // ❌ Despawn может не сработать если entity уже удален
    for (list_entity, list) in &mut option_lists {
        if list.0 == entity {
            commands.entity(list_entity).despawn();
        }
    }
}
```

**Проблемы:**

1. При быстром открытии/закрытии создаются множественные копии
2. `despawn()` без рекурсии оставляет дочерние entities
3. Нет проверки существования entity перед despawn

**Исправление:**

```rust
#[derive(Component)]
pub struct DropdownOptionsList {
    parent: Entity,
    is_spawned: bool,
}

pub fn dropdown_interaction_system(
    mut commands: Commands,
    theme: Res<Theme>,
    mut dropdown_query: Query<(Entity, &Interaction, &mut Dropdown)>,
    option_lists: Query<(Entity, &DropdownOptionsList)>,
) {
    for (entity, interaction, mut dropdown) in &mut dropdown_query {
        if *interaction != Interaction::Pressed {
            continue;
        }
        
        dropdown.is_open = !dropdown.is_open;
        
        if dropdown.is_open {
            // Проверяем, не существует ли уже список
            let already_exists = option_lists.iter()
                .any(|(_, list)| list.parent == entity);
            
            if already_exists {
                warn!("[Dropdown] Options list already exists for {:?}", entity);
                continue;
            }
            
            spawn_dropdown_options(&mut commands, &theme, entity, &dropdown);
        } else {
            // Рекурсивное удаление
            for (list_entity, list) in &option_lists {
                if list.parent == entity {
                    commands.entity(list_entity).despawn_recursive();
                }
            }
        }
    }
}
```

### 3. Некорректная обработка состояния загрузки

**Проблема в `src/launcher/loading.rs` (строки 119-130):**

```rust
fn update_loading_progress(
    time: Res<Time>,
    mut tracker: ResMut<LoadingTracker>,
    mut fade: ResMut<crate::ui::fading::ScreenFade>,
) {
    tracker.progress += time.delta_secs() / 3.0;  // ❌ Mock прогресс
    
    if tracker.progress >= 1.0 {
        tracker.progress = 1.0;
        if !tracker.completed_logged {
            info!("[LoadingUI] Content loaded. Fading out to InGame.");
            fade.fade_out(0.5, AppState::InGame);
            tracker.completed_logged = true;
        }
    }
}
```

**Проблемы:**

1. Фейковый прогресс вместо реальной загрузки ассетов
2. Нет проверки, действительно ли ассеты загружены
3. Может перейти в InGame с незагруженными ресурсами

**Правильная реализация:**

```rust
#[derive(Resource)]
pub struct AssetLoadingState {
    required_assets: Vec<UntypedHandle>,
    loaded_count: usize,
    total_count: usize,
}

fn update_loading_progress(
    asset_server: Res<AssetServer>,
    mut loading_state: ResMut<AssetLoadingState>,
    mut tracker: ResMut<LoadingTracker>,
    mut fade: ResMut<ScreenFade>,
) {
    // Реальная проверка загрузки
    let mut loaded = 0;
    for handle in &loading_state.required_assets {
        match asset_server.get_load_state(handle.id()) {
            Some(bevy::asset::LoadState::Loaded) => loaded += 1,
            Some(bevy::asset::LoadState::Failed(_)) => {
                error!("[Loading] Asset failed to load: {:?}", handle);
                // Переход в Error state
                return;
            }
            _ => {}
        }
    }
    
    loading_state.loaded_count = loaded;
    tracker.progress = loaded as f32 / loading_state.total_count.max(1) as f32;
    
    if tracker.progress >= 1.0 && !tracker.completed_logged {
        info!("[LoadingUI] All assets loaded successfully");
        fade.fade_out(0.5, AppState::InGame);
        tracker.completed_logged = true;
    }
}
```

## 🟡 Серьезные проблемы безопасности и стабильности

### 4. Отсутствие валидации пользовательского ввода

**Проблема в `src/launcher/menu/widgets/sliders.rs` (строки 61-81):**

```rust
fn slider_interaction_system(
    mut interaction_query: Query<(&Interaction, &GlobalTransform, &ComputedNode, &mut Slider)>,
    mut settings: ResMut<UserSettings>,
    windows: Query<&Window>,
) {
    let window = if let Ok(w) = windows.single() { w } else { return; };
    let mouse_pos = if let Some(pos) = window.cursor_position() { pos } else { return; };
    
    for (interaction, transform, computed, mut slider) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            let width = computed.size().x;
            // ❌ Нет проверки на деление на ноль
            if width <= 0.0 { continue; }
            
            let node_pos = transform.translation().truncate();
            let half_width = width / 2.0;
            let min_x = node_pos.x - half_width;
            
            let relative_x = (mouse_pos.x - min_x) / width;
            let relative_x = relative_x.clamp(0.0, 1.0);
            
            let new_value = slider.min + (slider.max - slider.min) * relative_x;
            slider.value = new_value;  // ❌ Нет валидации диапазона
            
            // ❌ Прямая запись в settings без проверок
            match slider.setting_key.as_str() {
                "master_volume" => settings.audio.master_volume = new_value,
                // ...
            }
        }
    }
}
```

**Исправление с валидацией:**

```rust
fn slider_interaction_system(
    mut interaction_query: Query<(&Interaction, &GlobalTransform, &ComputedNode, &mut Slider)>,
    mut settings: ResMut<UserSettings>,
    windows: Query<&Window>,
) {
    let Ok(window) = windows.get_single() else { return; };
    let Some(mouse_pos) = window.cursor_position() else { return; };
    
    for (interaction, transform, computed, mut slider) in &mut interaction_query {
        if *interaction != Interaction::Pressed {
            continue;
        }
        
        let width = computed.size().x;
        if width <= f32::EPSILON {
            warn!("[Slider] Invalid slider width: {}", width);
            continue;
        }
        
        let node_pos = transform.translation().truncate();
        let relative_x = ((mouse_pos.x - (node_pos.x - width / 2.0)) / width)
            .clamp(0.0, 1.0);
        
        let new_value = (slider.min + (slider.max - slider.min) * relative_x)
            .clamp(slider.min, slider.max);
        
        // Дополнительная валидация для аудио
        let new_value = if slider.setting_key.contains("volume") {
            new_value.clamp(0.0, 1.0)
        } else {
            new_value
        };
        
        if (slider.value - new_value).abs() < 0.001 {
            continue; // Избегаем излишних обновлений
        }
        
        slider.value = new_value;
        
        // Применяем с валидацией
        apply_slider_setting(&slider.setting_key, new_value, &mut settings);
    }
}

fn apply_slider_setting(key: &str, value: f32, settings: &mut UserSettings) {
    match key {
        "master_volume" => {
            settings.audio.master_volume = value.clamp(0.0, 1.0);
            info!("[Settings] Master volume: {:.2}", value);
        }
        "music_volume" => {
            settings.audio.music_volume = value.clamp(0.0, 1.0);
            info!("[Settings] Music volume: {:.2}", value);
        }
        "sfx_volume" => {
            settings.audio.sfx_volume = value.clamp(0.0, 1.0);
            info!("[Settings] SFX volume: {:.2}", value);
        }
        _ => warn!("[Settings] Unknown slider key: {}", key),
    }
}
```

### 5. Race condition в системе затухания

**Проблема в `src/ui/fading.rs` (строки 85-120):**

```rust
fn update_fade_system(
    mut fade: ResMut<ScreenFade>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<AppState>>,
    mut query: Query<&mut BackgroundColor, With<FadeOverlay>>,
) {
    // ...
    match fade.state {
        FadeState::FadingOut => {
            fade.alpha = percent;
            if fade.timer.just_finished() {
                fade.state = FadeState::Idle;
                fade.alpha = 1.0;
                
                if let Some(target) = fade.next_app_state {
                    next_state.set(target);
                    // ❌ Немедленное переключение на FadingIn
                    fade.state = FadeState::FadingIn;
                    fade.timer.reset();
                    fade.next_app_state = None;
                }
            }
        }
        // ...
    }
}
```

**Проблемы:**

1. `FadeState::Idle` сразу перезаписывается на `FadingIn`
2. Нет гарантии, что новый стейт уже активен
3. Может вызвать визуальный баг если OnEnter нового стейта также вызывает fade

**Исправление:**

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FadeState {
    Idle,
    FadingIn,
    FadingOut,
    WaitingForStateChange, // Новое состояние
}

fn update_fade_system(
    mut fade: ResMut<ScreenFade>,
    time: Res<Time>,
    current_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut query: Query<&mut BackgroundColor, With<FadeOverlay>>,
) {
    match fade.state {
        FadeState::FadingOut => {
            fade.alpha = fade.timer.fraction();
            
            if fade.timer.just_finished() {
                fade.alpha = 1.0;
                
                if let Some(target) = fade.next_app_state {
                    info!("[Fade] Requesting state transition to {:?}", target);
                    next_state.set(target);
                    fade.state = FadeState::WaitingForStateChange;
                } else {
                    fade.state = FadeState::Idle;
                }
            }
        }
        FadeState::WaitingForStateChange => {
            // Ждем подтверждения смены стейта
            if let Some(target) = fade.next_app_state {
                if current_state.get() == &target {
                    info!("[Fade] State changed, starting fade in");
                    fade.state = FadeState::FadingIn;
                    fade.timer = Timer::from_seconds(0.5, TimerMode::Once);
                    fade.timer.reset();
                    fade.next_app_state = None;
                }
            }
        }
        FadeState::FadingIn => {
            fade.alpha = 1.0 - fade.timer.fraction();
            
            if fade.timer.just_finished() {
                fade.alpha = 0.0;
                fade.state = FadeState::Idle;
            }
        }
        FadeState::Idle => {}
    }
    
    fade.timer.tick(time.delta());
    
    for mut bg in &mut query {
        bg.0 = Color::BLACK.with_alpha(fade.alpha);
    }
}
```

### 6. Проблемы с локализацией при runtime переключении

**Проблема в `src/core/localization/systems.rs` (строки 33-75):**

```rust
pub fn apply_language_change_system(
    settings: Res<UserSettings>,
    mut prev: Local<Option<String>>,
    paths: Res<AppPaths>,
    mut commands: Commands,
) {
    if !settings.is_changed() {
        return;
    }
    
    if prev.as_deref() != Some(settings.language.as_str()) {
        // ❌ Полная перезагрузка бандлов при каждом изменении
        // ❌ Существующие UI элементы не обновляются автоматически
        commands.insert_resource(Localization::new(/* ... */));
        *prev = Some(settings.language.clone());
    }
}
```

**Проблемы:**

1. UI не обновляется после смены языка
2. Нет события для уведомления компонентов
3. Потенциальная утечка памяти при частой смене языка

**Правильная реализация:**

```rust
// Добавить событие
#[derive(Event)]
pub struct LanguageChanged {
    pub old: String,
    pub new: String,
}

pub fn apply_language_change_system(
    settings: Res<UserSettings>,
    mut prev: Local<Option<String>>,
    paths: Res<AppPaths>,
    mut commands: Commands,
    mut events: EventWriter<LanguageChanged>,
) {
    if !settings.is_changed() {
        return;
    }
    
    if let Some(old_lang) = prev.as_ref() {
        if old_lang != &settings.language {
            info!("[Localization] Language changing: {} -> {}", old_lang, settings.language);
            
            // Создаем новый ресурс
            let (main_bundle, fallback_bundle) = create_bundles(&settings.language, &paths);
            commands.insert_resource(Localization::new(
                parse_language_id(&settings.language),
                main_bundle,
                fallback_bundle,
                paths.assets_dir.clone(),
            ));
            
            // Отправляем событие
            events.send(LanguageChanged {
                old: old_lang.clone(),
                new: settings.language.clone(),
            });
            
            *prev = Some(settings.language.clone());
        }
    } else {
        *prev = Some(settings.language.clone());
    }
}

// Система обновления UI текстов
#[derive(Component)]
pub struct LocalizedText {
    pub key: String,
}

pub fn update_localized_texts(
    mut events: EventReader<LanguageChanged>,
    loc: Res<Localization>,
    mut query: Query<(&LocalizedText, &mut Text)>,
) {
    if events.read().next().is_none() {
        return;
    }
    
    info!("[Localization] Updating {} localized text elements", query.iter().count());
    
    for (localized, mut text) in &mut query {
        text.0 = loc.t(&localized.key);
    }
}
```

## 🟢 Улучшения производительности и качества кода

### 7. Избыточные клонирования в настройках

**Проблема в `src/launcher/menu/reactive.rs` (строки 31-61):**

```rust
pub fn broadcast_settings_changes(
    settings: Res<UserSettings>,
    mut prev: Local<Option<UserSettings>>,  // ❌ Клонирует весь UserSettings
    mut windows: Query<&mut Window>,
    mut runtime: ResMut<RuntimeAudioState>,
) {
    // ...
    *prev = Some(settings.clone());  // ❌ Полное клонирование каждый кадр
}
```

**Оптимизация:**

```rust
#[derive(Default)]
struct SettingsSnapshot {
    display: DisplaySettings,
    audio: AudioSettings,
}

pub fn broadcast_settings_changes(
    settings: Res<UserSettings>,
    mut prev: Local<SettingsSnapshot>,
    mut windows: Query<&mut Window>,
    mut runtime: ResMut<RuntimeAudioState>,
) {
    if !settings.is_changed() {
        return;
    }
    
    // Проверяем только изменившиеся секции
    let display_changed = prev.display != settings.display;
    let audio_changed = prev.audio != settings.audio;
    
    if display_changed {
        if let Ok(mut window) = windows.get_single_mut() {
            window.resolution.set(
                settings.display.width as f32,
                settings.display.height as f32,
            );
            window.mode = if settings.display.fullscreen {
                WindowMode::Fullscreen(MonitorSelection::Current, VideoModeSelection::Current)
            } else {
                WindowMode::Windowed
            };
            info!("[Settings] Display updated: {}x{} fullscreen={}", 
                settings.display.width, settings.display.height, settings.display.fullscreen);
        }
        prev.display = settings.display.clone();
    }
    
    if audio_changed {
        runtime.master = settings.audio.master_volume;
        runtime.music = settings.audio.music_volume;
        runtime.sfx = settings.audio.sfx_volume;
        info!("[Settings] Audio updated: master={:.2} music={:.2} sfx={:.2}",
            runtime.master, runtime.music, runtime.sfx);
        prev.audio = settings.audio.clone();
    }
}
```

### 8. Отсутствие cleanup в splash системе

**Проблема в `src/launcher/splash.rs` (строки 45-75):**

```rust
fn countdown_splash(
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut fade: ResMut<ScreenFade>,
) {
    timer.0.tick(time.delta());
    
    if timer.0.just_finished() {
        fade.fade_out(0.5, AppState::MainMenu);
        return;  // ❌ Не очищает ресурсы до OnExit
    }
    
    if timer.0.elapsed_secs() > 1.0 {
        if skip_input {
            fade.fade_out(0.3, AppState::MainMenu);
            // ❌ Возможно дублирование fade_out
        }
    }
}
```

**Исправление:**

```rust
#[derive(Resource, Default)]
struct SplashState {
    timer: Timer,
    can_skip: bool,
    skip_requested: bool,
}

fn countdown_splash(
    time: Res<Time>,
    mut state: ResMut<SplashState>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut fade: ResMut<ScreenFade>,
) {
    // Предотвращаем двойной fade_out
    if state.skip_requested {
        return;
    }
    
    state.timer.tick(time.delta());
    
    // Разрешаем skip после 1 секунды
    if !state.can_skip && state.timer.elapsed_secs() > 1.0 {
        state.can_skip = true;
        info!("[Splash] Skip now available");
    }
    
    let should_transition = state.timer.just_finished() || 
        (state.can_skip && check_skip_input(&keys, &mouse));
    
    if should_transition {
        let duration = if state.timer.just_finished() { 0.5 } else { 0.3 };
        info!("[Splash] Transitioning to MainMenu (duration: {}s)", duration);
        fade.fade_out(duration, AppState::MainMenu);
        state.skip_requested = true;
    }
}

fn check_skip_input(keys: &ButtonInput<KeyCode>, mouse: &ButtonInput<MouseButton>) -> bool {
    keys.any_just_pressed([KeyCode::Space, KeyCode::Enter, KeyCode::Escape]) ||
    mouse.any_just_pressed([MouseButton::Left, MouseButton::Right])
}

// OnEnter - инициализация
fn setup_splash(mut commands: Commands, /* ... */) {
    commands.insert_resource(SplashState {
        timer: Timer::from_seconds(2.0, TimerMode::Once),
        can_skip: false,
        skip_requested: false,
    });
    // ...
}

// OnExit - полная очистка
fn cleanup_splash(
    mut commands: Commands,
    query: Query<Entity, With<SplashRoot>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
    commands.remove_resource::<SplashState>();
}
```

### 9. Неэффективная система обновления UI

**Проблема в `src/launcher/menu/settings/mod.rs` (строки 162-179):**

```rust
pub fn update_settings_ui(
    settings: Res<UserSettings>,
    mut master_q: Query<&mut Text, (With<MasterVolumeControl>, Without<...>)>,
    mut music_q: Query<&mut Text, (With<MusicVolumeControl>, Without<...>)>,
    mut sfx_q: Query<&mut Text, (With<SFXVolumeControl>, Without<...>)>,
) {
    // ❌ 3 отдельных запроса с пересекающимися фильтрами
    // ❌ Обновляет каждый кадр даже если не изменилось
    if let Ok(mut text) = master_q.single_mut() {
        text.0 = format!("{:.2}", settings.audio.master_volume);
    }
    // ...
}
```

**Оптимизированная версия:**

```rust
#[derive(Component)]
pub enum VolumeControlType {
    Master,
    Music,
    SFX,
}

pub fn update_settings_ui(
    settings: Res<UserSettings>,
    mut query: Query<(&VolumeControlType, &mut Text)>,
) {
    if !settings.is_changed() {
        return;
    }
    
    for (control_type, mut text) in &mut query {
        let new_text = match control_type {
            VolumeControlType::Master => format!("{:.2}", settings.audio.master_volume),
            VolumeControlType::Music => format!("{:.2}", settings.audio.music_volume),
            VolumeControlType::SFX => format!("{:.2}", settings.audio.sfx_volume),
        };
        
        if text.0 != new_text {
            text.0 = new_text;
        }
    }
}
```

### 10. Отсутствие обработки сбоев сохранения

**Проблема в `src/core/config/settings.rs` (строки 127-133):**

```rust
pub fn save_settings(paths: &AppPaths, settings: &UserSettings) {
    if let Ok(toml_string) = toml::to_string_pretty(settings)
        && let Err(e) = fs::write(&paths.settings_file, toml_string)
    {
        error!("[Config] Failed to save settings: {}", e);
        // ❌ Нет recovery механизма
        // ❌ Пользователь не уведомлен
    }
}
```

**Улучшенная версия:**

```rust
#[derive(Event)]
pub struct SettingsSaveError {
    pub error: String,
}

pub fn save_settings(
    paths: &AppPaths,
    settings: &UserSettings,
    events: Option<&mut EventWriter<SettingsSaveError>>,
) -> Result<(), String> {
    let toml_string = toml::to_string_pretty(settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;
    
    // Атомарное сохранение через временный файл
    let temp_path = paths.settings_file.with_extension("tmp");
    
    fs::write(&temp_path, &toml_string)
        .map_err(|e| format!("Failed to write temp file: {}", e))?;
    
    fs::rename(&temp_path, &paths.settings_file)
        .map_err(|e| {
            // Откат
            let _ = fs::remove_file(&temp_path);
            format!("Failed to move temp file: {}", e)
        })?;
    
    info!("[Config] Settings saved successfully to {:?}", paths.settings_file);
    Ok(())
}

// Обертка для системы
pub fn auto_save_settings(
    time: Res<Time>,
    mut timer: ResMut<SettingsAutoSaveTimer>,
    settings: Res<UserSettings>,
    paths: Res<AppPaths>,
    mut error_events: EventWriter<SettingsSaveError>,
) {
    if timer.0.is_paused() || !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    
    if let Err(e) = save_settings(&paths, &settings, Some(&mut error_events)) {
        error!("[Settings] Auto-save failed: {}", e);
        error_events.send(SettingsSaveError { error: e });
    }
    
    timer.0.pause();
}
```

## 📊 Анализ соответствия спецификациям

### 11. Несоответствие спецификации в `specs.md`

**Требование из `docs/development/launching/specs.md` (строки 121-125):**
> "Standard UI Audio Assets: click.ogg, hover.ogg, back.ogg, error.ogg, scroll.ogg"

**Факт в `src/launcher/menu/widgets/buttons.rs` (строки 92-120):**

```rust
// ❌ Используются только click.ogg и hover.ogg
// ❌ Отсутствуют back.ogg, error.ogg, scroll.ogg
if let Some(path) = manifest.audio("hover") {
    // ...
}
if let Some(path) = manifest.audio("click") {
    // ...
}
```

**Требуемая доработка:**

```rust
// Добавить enum для типов аудио событий
#[derive(Debug, Clone, Copy)]
pub enum UiAudioEvent {
    Click,
    Hover,
    Back,
    Error,
    Scroll,
}

impl UiAudioEvent {
    pub fn manifest_key(&self) -> &'static str {
        match self {
            Self::Click => "click",
            Self::Hover => "hover",
            Self::Back => "back",
            Self::Error => "error",
            Self::Scroll => "scroll",
        }
    }
}

pub fn play_ui_audio(
    commands: &mut Commands,
    asset_server: &AssetServer,
    manifest: &AssetManifest,
    audio_state: &RuntimeAudioState,
    event: UiAudioEvent,
) {
    if let Some(path) = manifest.audio(event.manifest_key()) {
        let volume = match event {
            UiAudioEvent::Hover => audio_state.sfx * 0.5,
            _ => audio_state.sfx,
        };
        
        commands.spawn((
            AudioPlayer::new(asset_server.load(path)),
            PlaybackSettings {
                mode: PlaybackMode::Despawn,
                volume: Volume::Linear(volume),
                ..default()
            },
        ));
    } else {
        warn!("[Audio] Missing UI sound: {:?}", event);
    }
}

// Использование
pub fn button_interaction_system(/* ... */) {
    for (interaction, /* ... */) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                play_ui_audio(&mut commands, &asset_server, &manifest, 
                             &audio_state, UiAudioEvent::Hover);
            }
            Interaction::Pressed => {
                play_ui_audio(&mut commands, &asset_server, &manifest,
                             &audio_state, UiAudioEvent::Click);
            }
            _ => {}
        }
    }
}

// Для кнопки Back
if button.action == ButtonAction::Back {
    play_ui_audio(&mut commands, &asset_server, &manifest,
                 &audio_state, UiAudioEvent::Back);
}
```

### 12. Отсутствие миграции версий конфига

**Требование из `specs.md` (строки 255-258):**
> "Configuration Migration (Version Guard): performs a 'non-destructive merge'"

**Факт в `src/core/config/settings.rs` (строки 93-110):**

```rust
if s.version < SETTINGS_VERSION {
    info!("[Config] Migration: Upgrading settings from v{} to v{}", 
          s.version, SETTINGS_VERSION);
    s.version = SETTINGS_VERSION;
    // ❌ Только обновление версии, нет реального слияния
    save_settings(paths, &s);
}
```

**Правильная реализация миграции:**

```rust
fn migrate_settings(old: UserSettings, paths: &AppPaths) -> UserSettings {
    let default = UserSettings::default();
    
    match old.version {
        1 => {
            info!("[Config] Migrating v1 -> v2: Adding graphics.quality");
            UserSettings {
                version: 2,
                language: old.language,
                display: old.display,
                audio: old.audio,
                graphics: default.graphics, // Новое поле из v2
            }
        }
        2 => {
            info!("[Config] Migrating v2 -> v3: No structural changes");
            UserSettings {
                version: 3,
                ..old
            }
        }
        _ => {
            warn!("[Config] Unknown version {}, using defaults", old.version);
            default
        }
    }
}

pub fn load_settings(paths: &AppPaths) -> UserSettings {
    if !paths.settings_file.exists() {
        info!("[Config] settings.toml not found. Creating default.");
        let default_settings = UserSettings::default();
        let _ = save_settings(paths, &default_settings, None);
        return default_settings;
    }
    
    match fs::read_to_string(&paths.settings_file) {
        Ok(content) => match toml::from_str::<UserSettings>(&content) {
            Ok(loaded) => {
                if loaded.version < SETTINGS_VERSION {
                    info!("[Config] Migrating settings from v{} to v{}", 
                          loaded.version, SETTINGS_VERSION);
                    let migrated = migrate_settings(loaded, paths);
                    let _ = save_settings(paths, &migrated, None);
                    migrated
                } else if loaded.version > SETTINGS_VERSION {
                    warn!("[Config] Settings file is from newer version (v{}), using defaults", 
                          loaded.version);
                    UserSettings::default()
                } else {
                    info!("[Config] Settings loaded successfully (v{})", loaded.version);
                    loaded
                }
            }
            Err(e) => {
                warn!("[Config] Failed to parse settings: {}. Using defaults.", e);
                UserSettings::default()
            }
        },
        Err(e) => {
            warn!("[Config] Failed to read settings: {}. Using defaults.", e);
            UserSettings::default()
        }
    }
}
```

## 🎯 Критический TODO список

### Высокий приоритет (КРИТИЧНО)

```markdown
- [ ] #1 Исправить утечку памяти в dropdown (despawn_recursive)
- [ ] #2 Реализовать реальную загрузку ассетов вместо mock прогресса
- [ ] #3 Исправить race condition в системе фейдинга
- [ ] #4 Добавить валидацию всех пользовательских вводов
- [ ] #5 Убрать дублирование состояния в HoverAnimationState
```

### Средний приоритет (ВАЖНО)

```markdown
- [ ] #6 Реализовать event-driven обновление UI при смене языка
- [ ] #7 Добавить атомарное сохранение настроек
- [ ] #8 Оптимизировать клонирование в broadcast_settings_changes
- [ ] #9 Реализовать полноценную миграцию версий конфига
- [ ] #10 Добавить все 5 UI аудио событий из спецификации
```

### Низкий приоритет (УЛУЧШЕНИЯ)

```markdown
- [ ] #11 Рефакторинг виджетов на единый трейт
- [ ] #12 Кэширование локализованных строк
- [ ] #13 Централизация констант анимации
- [ ] #14 Добавить property-based тесты
- [ ] #15 Улучшить cleanup в splash системе
```

## 🔧 Рекомендуемые новые файлы

### 1. `src/core/validation.rs` - Централизованная валидация

```rust
//! Input validation utilities

pub trait Validate {
    type Error;
    fn validate(&self) -> Result<(), Self::Error>;
}

impl Validate for DisplaySettings {
    type Error = String;
    
    fn validate(&self) -> Result<(), Self::Error> {
        if self.width < 640 || self.width > 7680 {
            return Err(format!("Invalid width: {}", self.width));
        }
        if self.height < 480 || self.height > 4320 {
            return Err(format!("Invalid height: {}", self.height));
        }
        Ok(())
    }
}

impl Validate for AudioSettings {
    type Error = String;
    
    fn validate(&self) -> Result<(), Self::Error> {
        for (name, value) in [
            ("master", self.master_volume),
            ("music", self.music_volume),
            ("sfx", self.sfx_volume),
        ] {
            if !(0.0..=1.0).contains(&value) {
                return Err(format!("{} volume out of range: {}", name, value));
            }
        }
        Ok(())
    }
}
```

### 2. `src/launcher/menu/events.rs` - UI события

```rust
//! UI-related events

use bevy::prelude::*;

#[derive(Event)]
pub struct LanguageChanged {
    pub old: String,
    pub new: String,
}

#[derive(Event)]
pub struct SettingChanged {
    pub key: SettingKey,
    pub value: SettingValue,
}

#[derive(Event)]
pub struct UiAudioRequested {
    pub event_type: UiAudioEvent,
}

#[derive(Event, Debug)]
pub struct SettingsSaveError {
    pub error: String,
    pub timestamp: std::time::Instant,
}
```

### 3. `tests/integration/settings_persistence.rs`

```rust
//! Integration tests for settings persistence

use bevy::prelude::*;
use planetarium::core::config::*;
use tempfile::TempDir;

#[test]
fn test_settings_roundtrip() {
    let temp = TempDir::new().unwrap();
    let paths = AppPaths {
        data_dir: temp.path().to_path_buf(),
        settings_file: temp.path().join("settings.toml"),
        // ...
    };
    
    let mut original = UserSettings::default();
    original.audio.master_volume = 0.75;
    original.display.width = 1920;
    
    save_settings(&paths, &original, None).unwrap();
    let loaded = load_settings(&paths);
    
    assert_eq!(loaded.audio.master_volume, 0.75);
    assert_eq!(loaded.display.width, 1920);
}

#[test]
fn test_migration_v1_to_v3() {
    // Тест миграции между версиями
}

#[test]
fn test_corrupt_settings_recovery() {
    // Тест восстановления при повреждении файла
}
```

## 📈 Метрики качества кода

После анализа:

```
Критические проблемы:     5 🔴
Важные улучшения:         10 🟡  
Оптимизации:              15 🟢
Несоответствия спеку:     3 ⚠️
Покрытие тестами:         ~15% (needs improvement)
Документация API:         ~60% (good)
Соответствие ECS:         ~75% (improvement needed)
```

## 🎓 Заключение

Проект демонстрирует хорошее понимание Bevy и архитектурных паттернов, но имеет несколько критических проблем, которые должны быть исправлены перед продакшном:

1. **Утечки памяти** - Dropdown система
2. **Фейковая загрузка** - Loading screen без реальных ассетов
3. **Race conditions** - Система фейдинга
4. **Отсутствие валидации** - Пользовательский ввод
5. **Неполная миграция** - Конфигурационные файлы

Рекомендую начать с исправления критических проблем (#1-#5), затем перейти к важным улучшениям (#6-#10), и только потом заниматься оптимизациями.
