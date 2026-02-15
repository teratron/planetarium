use crate::loading::assets::AssetManifest;
use crate::states::AppState;
use bevy::prelude::*;
pub use theme::ThemeLoadingState;
use theme::{Theme, ThemeColors, ThemeFonts, ThemeLoadingPhase, ThemeSizes};

/// Embedded fallback font for critical error states.
const FALLBACK_FONT_BYTES: &[u8] = include_bytes!("../../../../assets/fonts/FiraSans-Regular.ttf");

/// System to load theme assets (fonts) using paths from the AssetManifest.
pub fn setup_theme(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    manifest: Res<AssetManifest>,
    mut theme: ResMut<Theme>,
    mut fonts: ResMut<Assets<Font>>,
) {
    info!("[Theme] Initializing theme system...");

    // 1. Create embedded fallback (synchronously)
    let fallback_handle = match Font::try_from_bytes(FALLBACK_FONT_BYTES.to_vec()) {
        Ok(font) => {
            let handle = fonts.add(font);
            info!("[Theme] Embedded fallback font loaded");
            Some(handle)
        }
        Err(e) => {
            error!("[Theme] CRITICAL: Failed to load embedded font: {}", e);
            None
        }
    };

    let fallback = fallback_handle.unwrap_or_default();

    // 2. Start async loading from disk
    let main_path = manifest
        .font("main")
        .cloned()
        .unwrap_or_else(|| "fonts/FiraSans-Regular.ttf".to_string());
    let bold_path = manifest
        .font("bold")
        .cloned()
        .unwrap_or_else(|| "fonts/FiraSans-Regular.ttf".to_string());

    let main_handle = asset_server.load(main_path.clone());
    let bold_handle = asset_server.load(bold_path.clone());

    info!(
        "[Theme] Requesting font assets: main={}, bold={}",
        main_path, bold_path
    );

    // 3. Initialize theme with fallback values
    theme.colors = ThemeColors::default();
    theme.sizes = ThemeSizes::default();
    theme.fonts = ThemeFonts {
        main: main_handle.clone(),
        bold: bold_handle.clone(),
        fallback,
    };

    commands.insert_resource(ThemeLoadingState {
        main_font: Some(main_handle),
        bold_font: Some(bold_handle),
        is_ready: false,
    });
}

/// System to monitor theme loading progress.
pub fn check_theme_ready(
    asset_server: Res<AssetServer>,
    mut loading_state: ResMut<ThemeLoadingState>,
    mut local_phase: Local<ThemeLoadingPhase>,
    _next_state: ResMut<NextState<AppState>>,
) {
    if loading_state.is_ready {
        return;
    }

    match *local_phase {
        ThemeLoadingPhase::NotStarted => {
            info!("[Theme] Starting asset validation...");
            *local_phase = ThemeLoadingPhase::LoadingFonts;
        }
        ThemeLoadingPhase::LoadingFonts => {
            use bevy::asset::LoadState;

            let main_state = loading_state
                .main_font
                .as_ref()
                .and_then(|h| asset_server.get_load_state(h.id()));
            let bold_state = loading_state
                .bold_font
                .as_ref()
                .and_then(|h| asset_server.get_load_state(h.id()));

            let main_ready = matches!(main_state, Some(LoadState::Loaded));
            let bold_ready = matches!(bold_state, Some(LoadState::Loaded));

            if main_ready && bold_ready {
                info!("[Theme] All fonts loaded successfully");
                loading_state.is_ready = true;
                *local_phase = ThemeLoadingPhase::Ready;
                // In a real flow, we might advance state here if this was the last blocker
            }

            // Check for failures
            if matches!(main_state, Some(LoadState::Failed(_))) {
                warn!("[Theme] Main font failed to load, UI might be degraded");
                // We could set to ready anyway to allow usage of fallback
            }
            if matches!(bold_state, Some(LoadState::Failed(_))) {
                warn!("[Theme] Bold font failed to load");
            }
        }
        ThemeLoadingPhase::Ready => {
            // Can trigger follow-up state transitions here
        }
    }
}
