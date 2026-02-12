//! # Core Module (Re-export Shim)
//!
//! **DEPRECATED**: This module re-exports from new canonical locations
//! for backward compatibility during the architecture migration.
//! All new code should import from `crate::config`, `crate::utils`,
//! or `crate::framework` directly.

/// Re-exports from `crate::framework::states`.
pub use crate::framework::states;

/// Re-exports from `crate::config`.
pub use crate::config;

/// Re-exports from `crate::config::cli`.
pub use crate::config::cli;

/// Re-exports from `crate::utils::single_instance`.
pub use crate::utils::single_instance;

/// Re-exports from `crate::framework::localization`.
pub use crate::framework::localization;

/// Re-exports from `crate::framework::assets`.
pub use crate::framework::assets;
