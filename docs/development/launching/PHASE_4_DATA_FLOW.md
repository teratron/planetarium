# Phase 4 Data Flow & Architecture

## High-Level Flow Diagram

```plaintext
┌─────────────────────────────────────────────────────────────────┐
│                         Boot State                              │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ setup_theme (OnEnter)                                     │  │
│  │  ├─ Load fonts from AssetManifest                         │  │
│  │  ├─ Initialize Theme resource                             │  │
│  │  │  ├─ ThemeColors: 7-color palette                       │  │
│  │  │  ├─ ThemeFonts: main + bold handles                    │  │
│  │  │  └─ ThemeSizes: typography + spacing                   │  │
│  │  └─ Log: "[Theme] Hydrating theme assets..."              │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                          ↓ (transition)
┌─────────────────────────────────────────────────────────────────┐
│                       Splash State                              │
│ (skipped or displays splash sequence)                           │
└─────────────────────────────────────────────────────────────────┘
                          ↓ (transition)
┌─────────────────────────────────────────────────────────────────┐
│                      MainMenu State                             │
│                                                                 │
│  ┌─ spawn_main_menu (OnEnter) ───────────────────────────────┐  │
│  │  Creates:                                                 │  │
│  │  ├─ MainMenuRoot (full screen bg)                         │  │
│  │  ├─ Menu Panel (400px card)                               │  │
│  │  │  ├─ Title: "PLANETARIUM"                               │  │
│  │  │  └─ Buttons Container                                  │  │
│  │  │     ├─ PLAY (ButtonAction::Play)                       │  │
│  │  │     ├─ SETTINGS (ButtonAction::Settings)               │  │
│  │  │     └─ EXIT (ButtonAction::Exit)                       │  │
│  │  │                                                        │  │
│  │  Resources Created:                                       │  │
│  │  └─ SettingsOpen(false) → no modal yet                    │  │
│  │                                                           │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                 │
│  ┌─ Update Loop (MainMenu state) ────────────────────────────┐  │
│  │                                                           │  │
│  │ 1. button_interaction_system                              │  │
│  │    Filters: Interaction + PrimaryButton                   │  │
│  │    Effects: Color hover feedback (visual only)            │  │
│  │                                                           │  │
│  │ 2. handle_menu_button_clicks                              │  │
│  │    Filters: Changed<Interaction> + Button                 │  │
│  │    On PLAY Pressed:                                       │  │
│  │      └─ NextState → Loading                               │  │
│  │    On SETTINGS Pressed:                                   │  │
│  │      └─ SettingsOpen.0 = true                             │  │
│  │    On EXIT Pressed:                                       │  │
│  │      └─ std::process::exit(0)                             │  │
│  │    On BACK Pressed:                                       │  │
│  │      └─ SettingsOpen.0 = false                            │  │
│  │                                                           │  │
│  │ 3. spawn_settings_if_needed                               │  │
│  │    Condition: SettingsOpen.is_changed()                   │  │
│  │    If true & empty:                                       │  │
│  │      └─ spawn_settings_menu (creates SettingsRoot)        │  │
│  │    If false & exists:                                     │  │
│  │      └─ despawn SettingsRoot                              │  │
│  │                                                           │  │
│  │    Creates (when true):                                   │  │
│  │    ├─ SettingsRoot overlay (80% modal)                    │  │
│  │    ├─ Title: "Settings"                                   │  │
│  │    ├─ Tab headers: Graphics | Audio | Controls            │  │
│  │    ├─ Graphics Section:                                   │  │
│  │    │  ├─ Width: 1280 (ResolutionWidthControl)             │  │
│  │    │  ├─ Height: 720 (ResolutionHeightControl)            │  │
│  │    │  └─ Fullscreen: OFF (FullscreenToggle)               │  │
│  │    ├─ Audio Section:                                      │  │
│  │    │  ├─ Master: 0.80 (MasterVolumeControl)               │  │
│  │    │  ├─ Music: 0.70 (MusicVolumeControl)                 │  │
│  │    │  └─ SFX: 1.00 (SFXVolumeControl)                     │  │
│  │    └─ Back button (ButtonAction::Back)                    │  │
│  │                                                           │  │
│  │ 4. update_settings_ui (syncs values)                      │  │
│  │    Queries: ResolutionWidthControl, etc.                  │  │
│  │    Updates: Text.0 = UserSettings value                   │  │
│  │                                                           │  │
│  │ 5. broadcast_settings_changes (REACTIVE!)                 │  │
│  │    Condition: settings.is_changed()                       │  │
│  │    Reads: UserSettings resource                           │  │
│  │    Local State: prev: Option<UserSettings>                │  │
│  │                                                           │  │
│  │    If Display Changed:                                    │  │
│  │      ├─ window.resolution.set(width, height)              │  │
│  │      ├─ window.mode = Fullscreen | Windowed               │  │
│  │      └─ Log: "[Settings] Applied display..."              │  │
│  │                                                           │  │
│  │    If Audio Changed:                                      │  │
│  │      ├─ RuntimeAudioState.master = value                  │  │
│  │      ├─ RuntimeAudioState.music = value                   │  │
│  │      ├─ RuntimeAudioState.sfx = value                     │  │
│  │      └─ Log: "[Settings] Applied audio..."                │  │
│  │                                                           │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                 │
│  ┌─ despawn_main_menu (OnExit) ──────────────────────────────┐  │
│  │  Despawns MainMenuRoot entity (cascade)                   │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
         ↓ (PLAY)              ↓ (SETTINGS)           ↓ (EXIT)
  ┌─────────────────┐   ┌───────────────────┐   ┌──────────────┐
  │ Loading State   │   │ Settings Modal    │   │ Exit App     │
  │ (Phase 5)       │   │ (MainMenu overlay)│   │              │
  └─────────────────┘   └───────────────────┘   └──────────────┘
```

---

## Resource & Component Hierarchy

### Resources

```plaintext
UserSettings (from boot)
├─ display: DisplaySettings
│  ├─ width: u32
│  ├─ height: u32
│  └─ fullscreen: bool
├─ audio: AudioSettings
│  ├─ master_volume: f32
│  ├─ music_volume: f32
│  └─ sfx_volume: f32
└─ language: String

Theme (from setup_theme)
├─ colors: ThemeColors
│  ├─ background: #0B0C10
│  ├─ surface: #1F2833
│  ├─ text_primary: #E0E0E0
│  ├─ text_secondary: #C5C6C7
│  ├─ accent: #66FCF1
│  ├─ accent_muted: #45A29E
│  └─ danger: #FF4C4C
├─ fonts: ThemeFonts
│  ├─ main: Handle<Font>
│  └─ bold: Handle<Font>
└─ sizes: ThemeSizes
   ├─ margin: 20px
   ├─ padding: 16px
   ├─ font_h1: 48px
   ├─ font_h2: 32px
   ├─ font_body: 18px
   └─ button_height: 50px

SettingsOpen (from menu plugin init)
└─ 0: bool (true = modal visible)

RuntimeAudioState (from menu plugin init)
├─ master: f32
├─ music: f32
└─ sfx: f32
```

### Components (Entities)

```plaintext
MainMenuRoot
├─ MainMenuRoot marker
├─ Node (full screen)
├─ BackgroundColor (background color)
└─ children:
   └─ MenuPanel
      ├─ Node (400px card)
      ├─ BackgroundColor (surface color)
      └─ children:
         ├─ Text("PLANETARIUM")
         │  ├─ TextFont (48px)
         │  └─ TextColor (accent)
         └─ ButtonsContainer
            ├─ Node (flex column)
            └─ children:
               ├─ PlayButton
               │  ├─ Button
               │  ├─ PrimaryButton { label: "PLAY", action: Play }
               │  ├─ ButtonHoverState { base, hover }
               │  └─ children: Text("PLAY")
               ├─ SettingsButton
               │  ├─ Button
               │  ├─ PrimaryButton { label: "SETTINGS", action: Settings }
               │  ├─ ButtonHoverState { base, hover }
               │  └─ children: Text("SETTINGS")
               └─ ExitButton
                  ├─ Button
                  ├─ PrimaryButton { label: "EXIT", action: Exit }
                  ├─ ButtonHoverState { base, hover }
                  └─ children: Text("EXIT")

SettingsRoot (spawned if SettingsOpen.0)
├─ SettingsRoot marker
├─ Node (80% overlay)
├─ BackgroundColor (surface color)
└─ children:
   ├─ Text("Settings")
   ├─ TabsContainer
   │  ├─ Node (flex row)
   │  └─ children:
   │     ├─ Text("Graphics")
   │     ├─ Text("Audio")
   │     └─ Text("Controls")
   ├─ ContentContainer
   │  ├─ Node (flex column)
   │  └─ children:
   │     ├─ Text("Graphics Settings")
   │     ├─ WidthRow
   │     │  ├─ Text("Width (px):")
   │     │  └─ ResolutionWidthControl + Text("1280")
   │     ├─ HeightRow
   │     │  ├─ Text("Height (px):")
   │     │  └─ ResolutionHeightControl + Text("720")
   │     ├─ FullscreenRow
   │     │  ├─ Text("Fullscreen:")
   │     │  └─ FullscreenToggle + Text("OFF")
   │     ├─ Text("Audio Settings")
   │     ├─ MasterRow
   │     │  ├─ Text("Master Volume:")
   │     │  └─ MasterVolumeControl + Text("0.80")
   │     ├─ MusicRow
   │     │  ├─ Text("Music Volume:")
   │     │  └─ MusicVolumeControl + Text("0.70")
   │     ├─ SFXRow
   │     │  ├─ Text("SFX Volume:")
   │     │  └─ SFXVolumeControl + Text("1.00")
   └─ BackButtonContainer
      └─ BackButton
         ├─ Button
         ├─ PrimaryButton { label: "Back", action: Back }
         ├─ ButtonHoverState { base, hover }
         └─ children: Text("Back")
```

---

## System Execution Order (MainMenu State)

```plaintext
OnEnter(MainMenu):
  └─ spawn_main_menu
     Creates: MainMenuRoot + menu UI

Update (per frame):
  ├─ 1. button_interaction_system
  │     └─ Updates hover colors (visual feedback)
  │
  ├─ 2. handle_menu_button_clicks
  │     └─ Processes button presses (state changes)
  │
  ├─ 3. spawn_settings_if_needed
  │     └─ Conditionally spawns/despawns settings modal
  │
  ├─ 4. update_settings_ui
  │     └─ Syncs display values to UserSettings
  │
  └─ 5. broadcast_settings_changes
        └─ Applies settings to Window + RuntimeAudioState

OnExit(MainMenu):
  └─ despawn_main_menu
     Removes: MainMenuRoot (cascade)
```

---

## Data Change Flow (Reactive Path)

```plaintext
User modifies UserSettings (e.g., toggles fullscreen)
  ↓
is_changed() flag set on UserSettings resource
  ↓
broadcast_settings_changes system runs in Update
  ↓
Compares prev vs current display/audio settings
  ↓
Change detected:
  ├─ DISPLAY CHANGE:
  │  ├─ window.resolution.set(width, height)
  │  ├─ window.mode = fullscreen ? Fullscreen(...) : Windowed
  │  └─ info!("[Settings] Applied display...")
  │
  └─ AUDIO CHANGE:
     ├─ RuntimeAudioState.master = value
     ├─ RuntimeAudioState.music = value
     ├─ RuntimeAudioState.sfx = value
     └─ info!("[Settings] Applied audio...")
  ↓
update_settings_ui runs in parallel
  ├─ Reads UserSettings
  ├─ Updates Text components to display new values
  └─ UI reflects changes on next render
```

---

## State Transitions

```plaintext
MainMenu: PLAY button
  ├─ Detection: Interaction::Pressed + ButtonAction::Play
  ├─ Effect: NextState::set(AppState::Loading)
  └─ Next: Loading state

MainMenu: SETTINGS button
  ├─ Detection: Interaction::Pressed + ButtonAction::Settings
  ├─ Effect: SettingsOpen.0 = true
  ├─ Result: spawn_settings_if_needed detects change
  ├─ Action: spawn_settings_menu creates overlay
  └─ UI: Modal appears over menu

Settings: BACK button
  ├─ Detection: Interaction::Pressed + ButtonAction::Back
  ├─ Effect: SettingsOpen.0 = false
  ├─ Result: spawn_settings_if_needed detects change
  ├─ Action: despawn SettingsRoot
  └─ UI: Modal disappears

MainMenu: EXIT button
  ├─ Detection: Interaction::Pressed + ButtonAction::Exit
  ├─ Effect: std::process::exit(0)
  └─ Result: Application terminates gracefully
```

---

## Integration Points with Other Phases

### Phase 1 (Boot) → Phase 4

- `setup_theme` loads from AssetManifest (Phase 2)
- `UserSettings` loaded by boot config system (Phase 1)

### Phase 4 → Phase 5 (Loading)

- Theme system ready for loading screen UI
- Widget patterns reusable for progress displays
- Reactive pattern available for state transitions

### Phase 4 ← Game Module

- RuntimeAudioState consumed by audio system
- Theme colors usable throughout game UI
- Menu pattern reusable for pause screens

---

## Key Invariants

✅ Theme is always available in MainMenu state
✅ UserSettings resource always exists (loaded in Boot)
✅ RuntimeAudioState initialized on MenuPlugin build
✅ SettingsOpen resource toggles modal visibility
✅ Setting changes always trigger is_changed() flag
✅ broadcast_settings_changes runs every frame (Update loop)
✅ Window and RuntimeAudioState always in sync with UserSettings
✅ UI display values synced by update_settings_ui
