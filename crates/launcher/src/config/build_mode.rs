//! # Build Mode System
//!
//! Provides build-dependent configuration and environment detection.

use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BuildMode {
    #[default]
    Development,
    Production,
}

impl BuildMode {
    /// Detects current build mode based on compile-time flags.
    pub fn current() -> Self {
        if cfg!(debug_assertions) {
            Self::Development
        } else {
            Self::Production
        }
    }

    pub fn is_dev(&self) -> bool {
        matches!(self, Self::Development)
    }

    pub fn is_prod(&self) -> bool {
        matches!(self, Self::Production)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Development => "Development",
            Self::Production => "Production",
        }
    }
}

/// System to log build mode information on startup.
pub fn log_build_mode(build_mode: Res<BuildMode>) {
    match *build_mode {
        BuildMode::Development => {
            info!("╔════════════════════════════════════════╗");
            info!("║  Application running in               ║");
            info!("║         DEVELOPMENT MODE (DEV)        ║");
            info!("╚════════════════════════════════════════╝");
            warn!("[BuildMode] Dev features and debug symbols are ENABLED.");
        }
        BuildMode::Production => {
            info!("╔════════════════════════════════════════╗");
            info!("║  Application running in               ║");
            info!("║         PRODUCTION MODE (PROD)        ║");
            info!("╚════════════════════════════════════════╝");
            info!("[BuildMode] Optimized for performance and stability.");
        }
    }
}
