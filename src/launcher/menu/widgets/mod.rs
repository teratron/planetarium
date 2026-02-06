//! # UI Widgets
//!
//! Generic, reusable UI widgets for the main menu and settings screens.
//! Built on Bevy's native UI system with custom styling and interaction.

use crate::core::config::settings::Quality;

pub mod buttons;
pub mod components;
pub mod constants;
pub mod dropdowns;
pub mod sliders;

pub use buttons::button_interaction_system;
pub use buttons::spawn_primary_button;
pub use components::{
    ButtonAction, ButtonHoverState, Dropdown, DropdownOption, DropdownOptionsList, DropdownText,
    PrimaryButton, Slider, SliderFill, SliderTrack,
};
pub use constants::button as button_constants;
pub use dropdowns::dropdown_interaction_system;
pub use dropdowns::dropdown_option_interaction_system;
pub use dropdowns::{DropdownSpec, spawn_dropdown};
pub use sliders::slider_interaction_system;
pub use sliders::update_slider_visuals;
pub use sliders::{SliderSpec, spawn_slider};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quality_mapping() {
        assert_eq!(quality_from_index(0), Quality::Low);
        assert_eq!(quality_from_index(1), Quality::Medium);
        assert_eq!(quality_from_index(2), Quality::High);
        assert_eq!(quality_from_index(3), Quality::Ultra);
        assert_eq!(quality_from_index(999), Quality::Medium);
    }
}
