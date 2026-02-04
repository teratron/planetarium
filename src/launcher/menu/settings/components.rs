//! # Settings Components
//!
//! Marker components for settings controls (resolution, volume, etc.).

use bevy::prelude::Component;

/// Marker for the settings panel root.
#[derive(Component)]
pub struct SettingsRoot;

/// Marker for width resolution control.
#[derive(Component)]
pub struct ResolutionWidthControl;

/// Marker for height resolution control.
#[derive(Component)]
pub struct ResolutionHeightControl;

/// Marker for fullscreen toggle control.
#[derive(Component)]
pub struct FullscreenToggle;

/// Marker for master volume control.
#[derive(Component)]
pub struct MasterVolumeControl;

/// Marker for music volume control.
#[derive(Component)]
pub struct MusicVolumeControl;

/// Marker for SFX volume control.
#[derive(Component)]
pub struct SFXVolumeControl;
