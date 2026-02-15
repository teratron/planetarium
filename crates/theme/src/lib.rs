//! # User Interface Theme
//!
//! Centralizes design tokens (colors, fonts, sizes) for the application.

use bevy::prelude::*;

pub mod colors;
pub mod constants;
pub mod metrics;

pub use colors::ThemeColors;
pub use metrics::{ThemeFonts, ThemeSizes};

/// Plugin managing the UI theme.
pub struct ThemePlugin;

impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        // We only initialize the resource here with default/empty values if needed,
        // but the actual hydration happens in the Booting state (via launcher).
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

/// Loading state for theme assets.
#[derive(Resource, Default)]
pub struct ThemeLoadingState {
    pub main_font: Option<Handle<Font>>,
    pub bold_font: Option<Handle<Font>>,
    pub is_ready: bool,
}

/// Phases of theme loading.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeLoadingPhase {
    #[default]
    NotStarted,
    LoadingFonts,
    Ready,
}
