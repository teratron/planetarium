//! # UI Component Markers & Types
//!
//! Marker components for buttons, sliders, dropdowns, and other interactive widgets.

use crate::config::settings::SettingKey;
use bevy::prelude::{Color, Component, Entity};

/// Marker component for primary buttons (Play, Settings, Exit).
#[derive(Component, Debug, Clone)]
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
#[derive(Component, Debug)]
pub struct ButtonHoverState {
    pub base_color: Color,
    pub hover_color: Color,
}

/// Marker component for slider widgets (volume, brightness, etc.).
#[derive(Component, Debug, Clone)]
pub struct Slider {
    pub label: String,
    pub min: f32,
    pub max: f32,
    pub value: f32,
    pub setting_key: SettingKey,
}

/// Marker for interactive slider track.
#[derive(Component, Debug)]
pub struct SliderTrack(pub Entity); // Parent slider entity

/// Marker for slider fill area (visual feedback).
#[derive(Component, Debug)]
pub struct SliderFill(pub Entity); // Parent slider entity

/// Marker component for dropdown widgets (quality, resolution, etc.).
#[derive(Component, Debug, Clone)]
pub struct Dropdown {
    pub label: String,
    pub options: Vec<String>,
    /// Optional user-facing display values for options. If present, these are
    /// used for visual labels while `options` contains internal values (e.g., locale IDs).
    pub display_values: Option<Vec<String>>,
    pub selected_index: usize,
    pub setting_key: SettingKey,
    pub is_open: bool,
}

/// Marker for an option button within a dropdown list.
#[derive(Component, Debug, Clone)]
pub struct DropdownOption {
    pub parent_dropdown: Entity,
    pub index: usize,
}

/// Marker for the container of dropdown options.
#[derive(Component, Debug)]
pub struct DropdownOptionsList(pub Entity); // Parent dropdown entity

/// Marker for the text label inside the dropdown button.
#[derive(Component, Debug)]
pub struct DropdownText;
