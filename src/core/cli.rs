//! # Command Line Interface
//!
//! This module defines the arguments that can be passed to the application
//! via the command line. These arguments are useful for development,
//! debugging, and automation.

use bevy::prelude::Resource;
use clap::Parser;

/// Command line arguments for the Planetarium application.
///
/// We derive `Parser` for automatic parsing and `Resource`
/// so we can access these arguments inside Bevy systems.
#[derive(Parser, Resource, Debug, Clone)]
#[command(author, version, about, long_about = None)]
#[non_exhaustive]
pub struct CliArgs {
    /// Skip the splash screens and go straight to the main menu.
    #[arg(long, default_value_t = false)]
    pub skip_splash: bool,

    /// Start the app in a specific state (e.g., "MainMenu", "InGame").
    /// Case-insensitive.
    #[arg(long)]
    pub state: Option<String>,

    /// Enable verbose logging for debugging.
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,

    /// Custom log filter (e.g. "debug,planetarium=info"). Overrides default when provided
    /// unless the `RUST_LOG` environment variable is set.
    #[arg(long)]
    pub log_filter: Option<String>,
}

impl CliArgs {
    /// Parses the arguments from the environment.
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
