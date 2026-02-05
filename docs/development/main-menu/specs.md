# Architecture Specification: Main Menu Module

## Overview

The Main Menu is the primary interactive interface for the Planetarium game. It provides access to gameplay, configuration settings, and application exit. This module must deliver a high-quality "AAA" feel through smooth transitions, reactive UI elements, and full localization support.

## Core Features

1. **Main Landing Screen**:
    - Central logo/title display.
    - Primary actions: **Play**, **Settings**, **Exit**.
    - Professional hover and click auditory feedback.
2. **Advanced Settings System**:
    - Categorized interface: **Graphics**, **Audio**, **Controls**, **General**.
    - Live synchronization with `settings.toml` and engine resources.
    - Responsive sliders, toggles, and dropdowns.
3. **Localization-First UI**:
    - All text pulled from Fluent (.ftl) files.
    - Support for "on-the-fly" language switching (EN/RU).
4. **Visual Continuity**:
    - Smooth alpha-blending transitions (fading) between menu states and settings.
    - Hover animations for interactive elements.

## Technical Architecture

### AppStates

The module operates primarily within:

- `AppState::MainMenu`: The main interactive hub.
- Sub-states (managed via internal toggles/resources):
  - `SettingsOpen`: Reactive resource to toggle the settings overlay.

### Asset Orchestration

To fix current errors, the module implements:

- **Base Path Discovery**: Robust detection of the `assets/` directory regardless of CWD.
- **Embedded Fallbacks**: Critical assets (fonts, basic button textures) included in the binary for resilience.

### Localization Schema

- **File**: `assets/locales/{lang}/text/menu.ftl`
- **Keys**:
  - `menu-title`
  - `menu-play`
  - `menu-settings`
  - `menu-exit`
  - `settings-title`
  - `settings-tab-graphics`, `settings-tab-audio`, etc.

### Configuration Sync

- The UI binds directly to the `UserSettings` resource.
- Changes in the UI trigger `SettingChangedEvent`.
- A background system watches for these events to:
    1. Update the engine state (Window, Audio).
    2. Write changes back to `settings.toml` asynchronously.

## Error Handling & Robustness

- **Missing Asset Fallback**: If a localized string or sound is missing, the system falls back to a default (English / Magenta square).
- **Navigation Safety**: Every sub-menu must have a "Back" button bound to the `Esc` key.
- **Path Resilience**: Use `std::env::current_exe()` logic to locate assets when running directly.

## UI/UX Standards

| Requirement | Implementation Detail |
| :--- | :--- |
| **Hover FX** | Slight scaling (1.0 -> 1.05) and color highlight + `hover.ogg`. |
| **Click FX** | Visual indentation and `click.ogg`. |
| **Transitions** | 300ms alpha fade between Main Menu and Settings. |
| **High DPI** | Scaling based on `Window::scale_factor()`. |

## Backlog / Future Considerations

- [ ] Gamepad/Controller navigation support.
- [ ] Animated 3D background behind the UI.
- [ ] Particle effects on button hover.
