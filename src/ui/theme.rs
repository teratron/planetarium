use bevy::prelude::Color;

pub mod colors {
    use super::*;
    
    pub const TEXT_PRIMARY: Color = Color::srgb(1.0, 1.0, 1.0);
    pub const TEXT_SECONDARY: Color = Color::srgb(0.7, 0.7, 0.7);
    
    pub const BUTTON_NORMAL: Color = Color::srgb(0.2, 0.2, 0.3);
    pub const BUTTON_HOVER: Color = Color::srgb(0.3, 0.3, 0.45);
    pub const BUTTON_PRESSED: Color = Color::srgb(0.15, 0.15, 0.25);
    pub const BUTTON_BORDER: Color = Color::srgb(0.4, 0.4, 0.6);
    
    pub const MENU_BACKGROUND: Color = Color::srgb(0.1, 0.1, 0.15);
}

pub mod fonts {
    pub const TITLE_SIZE: f32 = 72.0;
    pub const BUTTON_TEXT_SIZE: f32 = 24.0;
    pub const HEADER_SIZE: f32 = 32.0;
}
