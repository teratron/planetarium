//! # Asset Manifest System
//!
//! Handles the parsing of `assets.ron` and provides a centralized
//! index of all mandatory game assets.

use crate::config::metadata::ASSET_MANIFEST_FILENAME;
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

/// Runtime cache for frequently used asset handles.
#[derive(Resource, Debug, Default)]
#[non_exhaustive]
pub struct AssetCache {
    fonts: HashMap<String, Handle<Font>>,
    audio: HashMap<String, Handle<AudioSource>>,
}

impl AssetCache {
    /// Get or load a font by manifest key, falling back to the provided path.
    pub fn get_or_load_font(
        &mut self,
        key: &str,
        fallback_path: &str,
        asset_server: &AssetServer,
        manifest: &AssetManifest,
    ) -> Handle<Font> {
        if let Some(handle) = self.fonts.get(key) {
            return handle.clone();
        }

        let path = manifest
            .font(key)
            .cloned()
            .unwrap_or_else(|| fallback_path.to_string());
        let handle: Handle<Font> = asset_server.load(path);
        self.fonts.insert(key.to_string(), handle.clone());
        handle
    }

    /// Get or load an audio asset by manifest key.
    pub fn get_or_load_audio(
        &mut self,
        key: &str,
        asset_server: &AssetServer,
        manifest: &AssetManifest,
    ) -> Option<Handle<AudioSource>> {
        if let Some(handle) = self.audio.get(key) {
            return Some(handle.clone());
        }

        let path = manifest.audio(key)?.to_string();
        let handle: Handle<AudioSource> = asset_server.load(path);
        self.audio.insert(key.to_string(), handle.clone());
        Some(handle)
    }
}

/// System to load the asset manifest from disk.
pub fn setup_asset_manifest(
    mut commands: Commands,
    mut next_state: ResMut<NextState<crate::framework::states::AppState>>,
    mut error_state: ResMut<crate::framework::states::ErrorState>,
    paths: Res<crate::config::AppPaths>,
) {
    let manifest_path = paths.assets_dir.join(ASSET_MANIFEST_FILENAME);
    info!("[Assets] Loading manifest from {:?}", manifest_path);

    let manifest = match fs::read_to_string(&manifest_path) {
        Ok(content) => match ron::from_str::<AssetManifest>(&content) {
            Ok(m) => {
                info!("[Assets] Manifest loaded successfully.");
                m
            }
            Err(e) => {
                let err_msg = format!("Failed to parse {}: {}", ASSET_MANIFEST_FILENAME, e);
                error!("[Assets] {}", err_msg);
                error_state.message = err_msg;
                next_state.set(crate::framework::states::AppState::Error);
                AssetManifest::default()
            }
        },
        Err(e) => {
            let err_msg = format!("Failed to read {}: {}", ASSET_MANIFEST_FILENAME, e);
            error!("[Assets] {}", err_msg);
            error_state.message = err_msg;
            next_state.set(crate::framework::states::AppState::Error);
            AssetManifest::default()
        }
    };

    commands.insert_resource(manifest);
    commands.insert_resource(AssetCache::default());
}
