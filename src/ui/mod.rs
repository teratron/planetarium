//! # UI Module (Re-export Shim)
//!
//! **DEPRECATED**: This module re-exports from `crate::framework::ui`
//! for backward compatibility during the architecture migration.
//! All new code should import from `crate::framework::ui` directly.

pub use crate::framework::ui::fading;
pub use crate::framework::ui::theme;
