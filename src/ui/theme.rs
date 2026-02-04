//! # User Interface Theme
//!
//! Centralizes design tokens (colors, fonts, sizes) for the application.

use crate::core::assets::AssetManifest;
use bevy::asset::AssetServer;
use bevy::prelude::*;

/// Plugin managing the UI theme.
pub struct ThemePlugin;

impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        // We only initialize the resource here with default/empty values if needed,
        // but the actual hydration happens in the Booting state.
        app.init_resource::<Theme>();
    }
}

/// Global theme resource containing colors, fonts, and metrics.
#[derive(Resource, Debug, Clone)]
pub struct Theme {
    pub colors: ThemeColors,
    pub fonts: ThemeFonts,
    pub sizes: ThemeSizes,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            colors: ThemeColors::default(),
            fonts: ThemeFonts::default(),
            sizes: ThemeSizes::default(),
        }
    }
}

/// Standardized color palette.
/// "Deep Space & Neon" aesthetic.
#[derive(Debug, Clone)]
pub struct ThemeColors {
    /// Very dark background (Void).
    pub background: Color,
    /// Slightly lighter background for panels/cards.
    pub surface: Color,
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

/// Typed handles for standard fonts.
#[derive(Debug, Clone, Default)]
pub struct ThemeFonts {
    pub main: Handle<Font>,
    pub bold: Handle<Font>,
}

/// Standard UI metrics.
#[derive(Debug, Clone)]
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
            padding: Val::Px(16.0),
            font_h1: 48.0,
            font_h2: 32.0,
            font_body: 18.0,
            button_height: Val::Px(50.0),
        }
    }
}

/// System to load theme assets (fonts) using paths from the AssetManifest.
pub fn setup_theme(
    asset_server: Res<AssetServer>,
    manifest: Res<AssetManifest>,
    mut theme: ResMut<Theme>,
) {
    info!("[Theme] Hydrating theme assets...");

    // Load fonts with fallback paths
    let main_path = manifest
        .font("main")
        .map(|s| s.clone())
        .unwrap_or_else(|| "fonts/Inter-Regular.ttf".to_string());
    let bold_path = manifest
        .font("bold")
        .map(|s| s.clone())
        .unwrap_or_else(|| "fonts/Inter-Bold.ttf".to_string());

    theme.fonts.main = asset_server.load(main_path);
    theme.fonts.bold = asset_server.load(bold_path);

    // Force initialization of colors and sizes if not already set (re-applying defaults is cheap)
    theme.colors = ThemeColors::default();
    theme.sizes = ThemeSizes::default();
}
