//! # Framework Layer
//!
//! Reusable, game-agnostic infrastructure for Bevy applications.
//! This module provides common functionality such as state management,
//! splash screens, loading systems, menus, settings, audio, camera
//! controllers, and shared UI utilities.

/// Application state machine and transition helpers.
pub mod states;

/// Framework plugin that bundles all framework sub-plugins.
pub mod plugin;

/// Boot sequence — initialization and environment checks.
pub mod boot;

/// Splash screen display.
pub mod splash;

/// Asset loading orchestration and progress tracking.
pub mod loading;

/// Error state UI.
pub mod error;

/// Debug overlay and diagnostics.
pub mod diagnostics;

/// Main menu, settings, and widget systems.
pub mod menu;

pub use plugin::FrameworkPlugin;
