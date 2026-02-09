//! # Settings Components
//!
//! Marker components for settings controls (resolution, volume, etc.).

use bevy::prelude::{Component, Timer};

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

/// Marker for a settings tab button.
#[derive(Component)]
pub struct SettingsTabButton(pub super::SettingsTab);

/// Marker for the container holding tab content.
#[derive(Component)]
pub struct SettingsContentArea;

/// Marker for the Graphics category panel.
#[derive(Component)]
pub struct GraphicsSettingsPanel;

/// Marker for the Audio category panel.
#[derive(Component)]
pub struct AudioSettingsPanel;

/// Marker for the Controls category panel.
#[derive(Component)]
pub struct ControlsSettingsPanel;

/// Marker for the General category panel.
#[derive(Component)]
pub struct GeneralSettingsPanel;

/// Component to handle fade transitions for the settings menu.
#[derive(Component)]
pub struct SettingsFade {
    pub timer: Timer,
    pub direction: FadeDirection,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FadeDirection {
    In,
    Out,
}
