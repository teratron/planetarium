//! # Application States
//!
//! Re-exports the primary application state machine and related resources
//! used to drive the high-level application flow.

pub mod app_state;
pub mod transition;

pub use app_state::{AppState, ErrorState};
