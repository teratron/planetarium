//! # Theme Typography & Sizes
//!
//! Centralized font handles and UI metrics.

use bevy::prelude::{Font, Handle, Val};

/// Typed handles for standard fonts.
#[derive(Debug, Clone, Default)]
pub struct ThemeFonts {
    pub main: Handle<Font>,
    pub bold: Handle<Font>,
    pub fallback: Handle<Font>,
}

/// Standard UI metrics (margins, padding, font sizes).
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ThemeSizes {
    pub margin: Val,
    pub padding: Val,
    pub font_h1: f32,
    pub font_h2: f32,
    pub font_body: f32,
    pub button_height: Val,
}

impl Default for ThemeSizes {
    fn default() -> Self {
        Self {
            margin: Val::Px(20.0),
            padding: Val::Px(24.0),
            font_h1: 64.0,
            font_h2: 32.0,
            font_body: 18.0,
            button_height: Val::Px(60.0),
        }
    }
}
