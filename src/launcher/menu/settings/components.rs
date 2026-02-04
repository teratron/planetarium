//! # Settings Components
//!
//! Marker components for settings controls (resolution, volume, etc.).

/// Marker for the settings panel root.
#[derive(bevy::prelude::Component)]
pub struct SettingsRoot;

/// Marker for width resolution control.
#[derive(bevy::prelude::Component)]
pub struct ResolutionWidthControl;

/// Marker for height resolution control.
#[derive(bevy::prelude::Component)]
pub struct ResolutionHeightControl;

/// Marker for fullscreen toggle control.
#[derive(bevy::prelude::Component)]
pub struct FullscreenToggle;

/// Marker for master volume control.
#[derive(bevy::prelude::Component)]
pub struct MasterVolumeControl;

/// Marker for music volume control.
#[derive(bevy::prelude::Component)]
pub struct MusicVolumeControl;

/// Marker for SFX volume control.
#[derive(bevy::prelude::Component)]
pub struct SFXVolumeControl;
