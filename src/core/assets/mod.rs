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
pub fn setup_asset_manifest(mut commands: Commands) {
    let manifest_path = "assets/assets.toml";
    info!("[Assets] Loading manifest from {}", manifest_path);

    let manifest = match fs::read_to_string(manifest_path) {
        Ok(content) => match toml::from_str::<AssetManifest>(&content) {
            Ok(m) => {
                info!("[Assets] Manifest loaded successfully.");
                m
            }
            Err(e) => {
                error!("[Assets] Failed to parse assets.toml: {}", e);
                AssetManifest::default()
            }
        },
        Err(e) => {
            error!("[Assets] Failed to read assets.toml: {}", e);
            AssetManifest::default()
        }
    };

    commands.insert_resource(manifest);
}
