//! # Application States (Re-export Shim)
//!
//! **DEPRECATED**: This module re-exports types from `crate::framework::states`
//! for backward compatibility during the architecture migration.
//! All new code should import from `crate::framework::states` directly.

pub use crate::framework::states::{AppState, ErrorState};
