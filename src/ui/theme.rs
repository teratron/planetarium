//! # User Interface Theme
//!
//! Centralizes design tokens (colors, fonts, sizes) for the application.

use crate::core::assets::AssetManifest;
use bevy::asset::AssetServer;
use bevy::prelude::*;

pub mod colors;
pub mod metrics;

pub use colors::ThemeColors;
pub use metrics::{ThemeFonts, ThemeSizes};

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
#[derive(Resource, Debug, Clone, Default)]
#[non_exhaustive]
pub struct Theme {
    pub colors: ThemeColors,
    pub fonts: ThemeFonts,
    pub sizes: ThemeSizes,
}

/// Embedded fallback font for critical error states.
const FALLBACK_FONT_BYTES: &[u8] = include_bytes!("../../assets/fonts/FiraSans-Regular.ttf");

/// System to load theme assets (fonts) using paths from the AssetManifest.
pub fn setup_theme(
    asset_server: Res<AssetServer>,
    manifest: Res<AssetManifest>,
    mut theme: ResMut<Theme>,
    mut fonts: ResMut<Assets<Font>>,
) {
    info!("[Theme] Hydrating theme assets...");

    // 1. Register the embedded fallback font first
    // This ensures we ALWAYS have a valid font available.
    theme.fonts.fallback = fonts.add(Font::try_from_bytes(FALLBACK_FONT_BYTES.to_vec()).unwrap());

    // 2. Load primary fonts from disk
    let main_path = manifest
        .font("main")
        .cloned()
        .unwrap_or_else(|| "fonts/Inter-Regular.ttf".to_string());
    let bold_path = manifest
        .font("bold")
        .cloned()
        .unwrap_or_else(|| "fonts/Inter-Bold.ttf".to_string());

    theme.fonts.main = asset_server.load(main_path);
    theme.fonts.bold = asset_server.load(bold_path);

    // Force initialization of colors and sizes if not already set (re-applying defaults is cheap)
    theme.colors = ThemeColors::default();
    theme.sizes = ThemeSizes::default();
}
