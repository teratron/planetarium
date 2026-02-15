//! # UI Widgets
//!
//! Generic, reusable UI widgets for the main menu and settings screens.
//! Built on Bevy's native UI system with custom styling and interaction.

use crate::settings::Quality;

pub mod base;
pub mod buttons;
pub mod components;
pub mod constants;
pub mod dropdowns;
pub mod sliders;

pub use base::Widget;
pub use buttons::animate_button_hover;
pub use buttons::button_interaction_system;
pub use buttons::spawn_primary_button;
pub use buttons::{PrimaryButtonSpec, PrimaryButtonWidget};
pub use components::{
    ButtonAction, ButtonHoverState, Dropdown, DropdownOption, DropdownOptionsList, DropdownText,
    PrimaryButton, Slider, SliderFill, SliderTrack,
};
pub use constants::button as button_constants;
pub use dropdowns::dropdown_interaction_system;
pub use dropdowns::dropdown_option_interaction_system;
pub use dropdowns::{spawn_dropdown, DropdownSpec, DropdownWidget};
pub use sliders::slider_interaction_system;
pub use sliders::update_slider_visuals;
pub use sliders::{spawn_slider, SliderSpec, SliderWidget, SliderWidgetSpec};

/// Helper to convert dropdown index into a `Quality` value.
pub fn quality_from_index(index: usize) -> Quality {
    match index {
        0 => Quality::Low,
        1 => Quality::Medium,
        2 => Quality::High,
        3 => Quality::Ultra,
        _ => Quality::Medium,
    }
}

/// Parse resolution string like "1920x1080" into (width, height).
pub fn parse_resolution(res: &str) -> Option<(u32, u32)> {
    let parts: Vec<&str> = res.split('x').collect();
    if parts.len() == 2 {
        let w = parts[0].parse().ok()?;
        let h = parts[1].parse().ok()?;
        Some((w, h))
    } else {
        None
    }
}
