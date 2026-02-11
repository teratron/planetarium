//! Core functionality, shared states, and global resources.

/// High-level application state definitions.
pub mod states;

/// Command line argument parsing.
pub mod cli;

/// Application configuration and path resolution.
pub mod config;

/// Startup single-instance lock protection.
pub mod single_instance;

/// Multi-language support using Project Fluent.
pub mod localization;

/// Asset manifest and management.
pub mod assets;
