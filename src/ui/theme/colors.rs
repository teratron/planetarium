//! # Theme Color Palette
//!
//! Standardized color palette with "Deep Space & Neon" aesthetic.

use bevy::prelude::Color;

/// Standardized color palette.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ThemeColors {
    /// Very dark background (Void).
    pub background: Color,
    /// Slightly lighter background for panels/cards.
    pub surface: Color,
    /// Lighter surface for hover states.
    pub surface_light: Color,
    /// Primary text color (High contrast).
    pub text_primary: Color,
    /// Secondary text color (Muted).
    pub text_secondary: Color,
    /// Main accent color (Interactive elements).
    pub accent: Color,
    /// Muted accent (Disabled/Inactive).
    pub accent_muted: Color,
    /// Destructive/Warning color.
    pub danger: Color,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            // #0B0C10
            background: Color::srgb_u8(11, 12, 16),
            // #1F2833
            surface: Color::srgb_u8(31, 40, 51),
            // #2A3542 - Lighter surface for hover
            surface_light: Color::srgb_u8(42, 53, 66),
            // #E0E0E0 - Bright white/grey
            text_primary: Color::srgb_u8(224, 224, 224),
            // #C5C6C7
            text_secondary: Color::srgb_u8(197, 198, 199),
            // #66FCF1 - Neon Cyan
            accent: Color::srgb_u8(102, 252, 241),
            // #45A29E
            accent_muted: Color::srgb_u8(69, 162, 158),
            // #FF4C4C
            danger: Color::srgb_u8(255, 76, 76),
        }
    }
}
