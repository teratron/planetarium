//! # Settings UI Layout Constants
//!
//! Centralized constants for settings panel dimensions and styling.

use bevy::prelude::Val;

/// Settings panel layout constants.
pub mod panel {
    use super::*;

    pub const WIDTH: Val = Val::Percent(80.0);
    pub const HEIGHT: Val = Val::Percent(80.0);
    pub const TITLE_MARGIN_TOP: Val = Val::Px(20.0);
    pub const TITLE_MARGIN_BOTTOM: Val = Val::Px(20.0);
}

/// Tabs row styling constants.
pub mod tabs {
    use super::*;

    pub const HEIGHT: Val = Val::Px(40.0);
    pub const MARGIN_TOP: Val = Val::Px(12.0);
}

/// Content area constants.
pub mod content {
    use super::*;

    pub const HEIGHT: Val = Val::Percent(70.0);
    pub const MARGIN: Val = Val::Px(12.0);
    pub const PADDING: Val = Val::Px(16.0);
}

/// Settings row (Graphics, Audio) constants.
pub mod row {
    pub const HEIGHT: f32 = 50.0;
    pub const MARGIN_BOTTOM: f32 = 16.0;
}

/// Back button area constants.
pub mod button_area {
    use bevy::prelude::Val;

    pub const HEIGHT: Val = Val::Percent(10.0);
    pub const PADDING: Val = Val::Px(12.0);
}
