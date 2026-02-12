//! # Widget Constants
//!
//! Layout constants and metrics for UI widgets.

use bevy::prelude::Val;

/// Button widget styling constants.
pub mod button {
    use super::*;

    pub const HEIGHT: Val = Val::Px(50.0);
    pub const WIDTH: Val = Val::Percent(100.0);
    pub const PADDING: Val = Val::Px(10.0);
    pub const MARGIN: Val = Val::Px(8.0);
    pub const BORDER_RADIUS: f32 = 4.0;
}

/// Slider widget styling constants.
pub mod slider {
    use super::*;

    pub const HEIGHT: f32 = 40.0;
    pub const TRACK_HEIGHT: f32 = 8.0;
    pub const WIDTH: Val = Val::Percent(100.0);
    pub const PADDING: f32 = 10.0;
}

/// Dropdown widget styling constants.
pub mod dropdown {
    use super::*;

    pub const HEIGHT: f32 = 40.0;
    pub const WIDTH: Val = Val::Percent(100.0);
    pub const PADDING: f32 = 10.0;
    pub const PANEL_HEIGHT: f32 = 150.0; // Max visible items
}
