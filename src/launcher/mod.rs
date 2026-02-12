//! # Launcher Module (Re-export Shim)
//!
//! **DEPRECATED**: This module re-exports from `crate::framework`
//! for backward compatibility during the architecture migration.
//! All new code should import from `crate::framework` directly.

pub use crate::framework::boot;
pub use crate::framework::diagnostics;
pub use crate::framework::error;
pub use crate::framework::loading;
pub use crate::framework::menu;
pub use crate::framework::splash;

pub use crate::framework::FrameworkPlugin as LauncherPlugin;
