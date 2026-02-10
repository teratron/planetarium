//! UI constants for animations and timings.

pub mod animation {
    /// Default hover scale for interactive buttons.
    pub const BUTTON_HOVER_SCALE: f32 = 1.05;
    /// Lerp speed for hover transitions (higher is snappier).
    pub const BUTTON_HOVER_LERP_SPEED: f32 = 15.0;
}

pub mod timing {
    /// Settings panel fade-in duration in seconds.
    pub const SETTINGS_FADE_IN: f32 = 0.3;
    /// Settings panel fade-out duration in seconds.
    pub const SETTINGS_FADE_OUT: f32 = 0.2;
    /// Debounce duration for expensive display changes in seconds.
    pub const SETTINGS_DISPLAY_DEBOUNCE: f32 = 0.5;
}
