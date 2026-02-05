//! # Asset Manifest System
//!
//! Handles the parsing of `assets.toml` and provides a centralized
//! index of all mandatory game assets.

use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

/// Centralized manifest of all game assets.
#[derive(Resource, Deserialize, Debug, Clone, Default)]
#[non_exhaustive]
pub struct AssetManifest {
    pub fonts: HashMap<String, String>,
    pub audio: HashMap<String, String>,
    pub branding: HashMap<String, String>,
}

impl AssetManifest {
    /// Helper to get a font path by key.
    pub fn font(&self, key: &str) -> Option<&String> {
        self.fonts.get(key)
    }

    /// Helper to get an audio path by key.
    pub fn audio(&self, key: &str) -> Option<&String> {
        self.audio.get(key)
    }

    /// Helper to get a branding asset path by key.
    pub fn branding(&self, key: &str) -> Option<&String> {
        self.branding.get(key)
    }
}

/// System to load the asset manifest from disk.
pub fn setup_asset_manifest(
    mut commands: Commands,
    mut next_state: ResMut<NextState<crate::core::states::AppState>>,
    mut error_state: ResMut<crate::core::states::ErrorState>,
    paths: Res<crate::core::config::AppPaths>,
) {
    let manifest_path = paths.assets_dir.join("assets.toml");
    info!("[Assets] Loading manifest from {:?}", manifest_path);

    let manifest = match fs::read_to_string(&manifest_path) {
        Ok(content) => match toml::from_str::<AssetManifest>(&content) {
            Ok(m) => {
                info!("[Assets] Manifest loaded successfully.");
                m
            }
            Err(e) => {
                let err_msg = format!("Failed to parse assets.toml: {}", e);
                error!("[Assets] {}", err_msg);
                error_state.message = err_msg;
                next_state.set(crate::core::states::AppState::Error);
                AssetManifest::default()
            }
        },
        Err(e) => {
            let err_msg = format!("Failed to read assets.toml: {}", e);
            error!("[Assets] {}", err_msg);
            error_state.message = err_msg;
            next_state.set(crate::core::states::AppState::Error);
            AssetManifest::default()
        }
    };

    commands.insert_resource(manifest);
}
