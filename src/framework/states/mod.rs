//! # Application States
//!
//! Re-exports the primary application state machine and related resources
//! used to drive the high-level application flow.

pub mod state;
pub mod transition;

pub use state::{AppState, ErrorState};
