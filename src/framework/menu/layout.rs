//! # Menu Layout Configuration
//!
//! Centralized constants for menu UI dimensions and durations.

use bevy::prelude::Val;

/// Panel width for menu content container.
pub const PANEL_WIDTH: Val = Val::Px(500.0);

/// Panel height - auto to fit content.
pub const PANEL_HEIGHT: Val = Val::Auto;

/// Space between title and buttons.
pub const TITLE_MARGIN_BOTTOM: Val = Val::Px(60.0);

/// Fade duration when transitioning to Loading state.
pub const FADE_DURATION_LOADING: f32 = 0.8;
