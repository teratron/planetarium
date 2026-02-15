//! Settings Tab Implementations
//!
//! Contains specialized UI panels for each settings category:
//! Graphics, Audio, Controls, and General.

pub mod audio;
pub mod controls;
pub mod general;
pub mod graphics;

pub use audio::spawn_audio_tab;
pub use controls::spawn_controls_tab;
pub use general::spawn_general_tab;
pub use graphics::spawn_graphics_tab;
