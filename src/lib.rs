//! # Planetarium Core Library
//!
//! Two-layer architecture: `framework` (reusable infrastructure)
//! and `game` (game-specific logic). Legacy shim modules (`core`,
//! `launcher`, `ui`) re-export from their canonical locations.

pub mod config;
pub mod framework;
pub mod game;
pub mod utils;

// Re-export shims for backward compatibility (to be removed in Phase 7)
pub mod core;
pub mod launcher;
pub mod ui;
