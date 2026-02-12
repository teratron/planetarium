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

pub use plugin::FrameworkPlugin;
