//! # UI Component Markers & Types
//!
//! Marker components for buttons, sliders, dropdowns, and other interactive widgets.

use bevy::prelude::Color;

/// Marker component for primary buttons (Play, Settings, Exit).
#[derive(bevy::prelude::Component, Debug, Clone)]
pub struct PrimaryButton {
    pub label: String,
    pub action: ButtonAction,
}

/// Actions that buttons can trigger.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ButtonAction {
    Play,
    Settings,
    Exit,
    Back,
}

/// Marker for button hover state styling.
#[derive(bevy::prelude::Component, Debug)]
pub struct ButtonHoverState {
    pub base_color: Color,
    pub hover_color: Color,
}

/// Marker component for slider widgets (volume, brightness, etc.).
#[derive(bevy::prelude::Component, Debug, Clone)]
pub struct Slider {
    pub label: String,
    pub min: f32,
    pub max: f32,
    pub value: f32,
    pub setting_key: String,
}

/// Marker for interactive slider track.
#[derive(bevy::prelude::Component, Debug)]
pub struct SliderTrack(pub bevy::prelude::Entity); // Parent slider entity

/// Marker component for dropdown widgets (quality, resolution, etc.).
#[derive(bevy::prelude::Component, Debug, Clone)]
pub struct Dropdown {
    pub label: String,
    pub options: Vec<String>,
    pub selected_index: usize,
    pub setting_key: String,
    pub is_open: bool,
}
