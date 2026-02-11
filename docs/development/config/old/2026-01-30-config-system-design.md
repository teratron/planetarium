# Design Document: Configuration Management System

**Date:** 2026-01-30  
**Status:** Validated  
**Topic:** Reactive Dual-Layer Configuration System for Planetarium

## 1. Overview

The Configuration Management System provides a robust way to handle player settings (graphics, audio) and developer tools. It emphasizes **Reactive Sync** (changes in code/UI apply to disk, and vice versa) with a "Memory Wins" conflict resolution strategy.

## 2. Architecture

The system is built as a Bevy `Plugin` (`ConfigPlugin`) managing two primary resources:

- `GameConfig`: Player-facing settings stored in `config.toml` (TOML).
- `DevConfig`: Developer-only flags stored in `assets/dev_config.ron` (RON).

### Data Flow

- **Initialization**: Loads from system-specific app data folders using `directories` crate. Falls back to defaults if files are missing or corrupt.
- **Game → Disk Sync**: Systems monitoring `Changed<GameConfig>` trigger asynchronous disk writes.
- **Disk → Game Sync**: (Debug only) A file watcher monitors `config.toml` and updates the resource, allowing manual edits to reflect in-game instantly.
- **Conflict Strategy**: "Memory Wins". If the game state changes while a disk edit is detected, the game's state takes precedence.

## 3. Components & Data Structures

- **GraphicsConfig**: resolution, window mode, vsync.
- **AudioConfig**: master, music, sfx volumes (clamped 0.0-1.0).
- **Versioning**: Implicit `version` field in the TOML for future-proofing.

## 4. Systems (Appliers)

- `apply_graphics_settings`: Updates the `Window` entity based on config changes.
- `apply_audio_settings`: Updates global audio volume.
- `apply_dev_settings`: Toggles debug gizmos and FPS counters.

## 5. Robustness & Error Handling

- **Graceful Failure**: Corrupt files are backed up to `.bak` and reset to defaults.
- **ReadOnly Fallback**: Memory-only mode if disk is unwritable.
- **Validation**: Strict clamping and sanitization of user-edited values.
- **Migrations**: Automated upgrade path for configuration files across game versions.

## 6. Security

- Secrets (API keys, tokens) are handled via `.env` files and `dotenvy`, never touched by the `GameConfig` persistence logic.
- Developer configurations and systems are stripped in release builds using `#[cfg(debug_assertions)]`.
