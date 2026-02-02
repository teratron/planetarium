use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Main configuration for the Game Launcher
#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct LauncherConfig {
    /// Splash screen configuration
    pub splash: SplashConfig,
    /// Main menu configuration
    pub menu: MenuConfig,
    /// Loading screen configuration
    pub loading: LoadingConfig,
    /// Integration settings with game modules
    pub integration: IntegrationConfig,
    /// Version of the launcher configuration
    pub version: u32,
}

impl Default for LauncherConfig {
    fn default() -> Self {
        Self {
            splash: SplashConfig::default(),
            menu: MenuConfig::default(),
            loading: LoadingConfig::default(),
            integration: IntegrationConfig::default(),
            version: 1,
        }
    }
}

impl LauncherConfig {
    /// Validate all configuration values and fix any invalid ones
    pub fn validate(&mut self) {
        self.splash.validate();
        self.menu.validate();
        self.loading.validate();
        self.integration.validate();
    }
    
    /// Create a minimal launcher configuration (for testing or simple games)
    pub fn minimal() -> Self {
        Self {
            splash: SplashConfig::disabled(),
            menu: MenuConfig::simple(),
            loading: LoadingConfig::fast(),
            integration: IntegrationConfig::default(),
            version: 1,
        }
    }
    
    /// Create a full-featured launcher configuration
    pub fn full_featured() -> Self {
        Self {
            splash: SplashConfig::full_featured(),
            menu: MenuConfig::full_featured(),
            loading: LoadingConfig::detailed(),
            integration: IntegrationConfig::default(),
            version: 1,
        }
    }
}

/// Configuration for the splash screen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplashConfig {
    /// Whether to show the splash screen
    pub enabled: bool,
    /// Duration to show the splash screen (in milliseconds)
    pub duration_ms: u64,
    /// List of logos to display
    pub logos: Vec<LogoConfig>,
    /// Background color for the splash screen
    pub background_color: [f32; 4], // RGBA
    /// Whether to allow skipping the splash screen
    pub allow_skip: bool,
    /// Fade in/out duration (in milliseconds)
    pub fade_duration_ms: u64,
}

impl Default for SplashConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            duration_ms: 3000, // 3 seconds
            logos: vec![LogoConfig::default()],
            background_color: [0.0, 0.0, 0.0, 1.0], // Black
            allow_skip: true,
            fade_duration_ms: 500, // 0.5 seconds
        }
    }
}

impl SplashConfig {
    /// Create a disabled splash configuration
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Default::default()
        }
    }
    
    /// Create a full-featured splash configuration
    pub fn full_featured() -> Self {
        Self {
            enabled: true,
            duration_ms: 5000, // 5 seconds
            logos: vec![
                LogoConfig::new("assets/logos/company_logo.png", 2000),
                LogoConfig::new("assets/logos/engine_logo.png", 2000),
            ],
            background_color: [0.1, 0.1, 0.1, 1.0], // Dark gray
            allow_skip: true,
            fade_duration_ms: 750,
        }
    }
    
    /// Validate splash configuration
    pub fn validate(&mut self) {
        // Ensure minimum duration
        if self.duration_ms < 500 {
            self.duration_ms = 500;
        }
        
        // Ensure fade duration is reasonable
        if self.fade_duration_ms > self.duration_ms / 2 {
            self.fade_duration_ms = self.duration_ms / 4;
        }
        
        // Validate color values
        for component in &mut self.background_color {
            *component = component.clamp(0.0, 1.0);
        }
        
        // Validate logos
        for logo in &mut self.logos {
            logo.validate();
        }
    }
    
    /// Get the total duration including all logos
    pub fn total_duration(&self) -> Duration {
        if !self.enabled {
            return Duration::from_millis(0);
        }
        
        let logo_duration: u64 = self.logos.iter().map(|logo| logo.duration_ms).sum();
        let total_ms = self.duration_ms.max(logo_duration);
        Duration::from_millis(total_ms)
    }
}

/// Configuration for a single logo in the splash screen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoConfig {
    /// Path to the logo image
    pub image_path: String,
    /// Duration to show this logo (in milliseconds)
    pub duration_ms: u64,
    /// Scale factor for the logo
    pub scale: f32,
    /// Position offset from center (x, y)
    pub offset: [f32; 2],
}

impl Default for LogoConfig {
    fn default() -> Self {
        Self {
            image_path: "assets/logos/default_logo.png".to_string(),
            duration_ms: 2000, // 2 seconds
            scale: 1.0,
            offset: [0.0, 0.0],
        }
    }
}

impl LogoConfig {
    /// Create a new logo configuration
    pub fn new(image_path: impl Into<String>, duration_ms: u64) -> Self {
        Self {
            image_path: image_path.into(),
            duration_ms,
            scale: 1.0,
            offset: [0.0, 0.0],
        }
    }
    
    /// Set the scale factor
    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }
    
    /// Set the position offset
    pub fn with_offset(mut self, x: f32, y: f32) -> Self {
        self.offset = [x, y];
        self
    }
    
    /// Validate logo configuration
    pub fn validate(&mut self) {
        // Ensure minimum duration
        if self.duration_ms < 100 {
            self.duration_ms = 100;
        }
        
        // Ensure reasonable scale
        self.scale = self.scale.clamp(0.1, 10.0);
        
        // Ensure path is not empty
        if self.image_path.trim().is_empty() {
            self.image_path = "assets/logos/default_logo.png".to_string();
        }
    }
}

/// Configuration for the main menu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuConfig {
    /// Whether the main menu is enabled
    pub enabled: bool,
    /// Theme name for the menu
    pub theme: String,
    /// Whether to show the settings button
    pub show_settings: bool,
    /// Whether to show the credits button
    pub show_credits: bool,
    /// Custom buttons to add to the menu
    pub custom_buttons: Vec<CustomButtonConfig>,
    /// Background music for the menu
    pub background_music: Option<String>,
    /// Whether to show animated background
    pub animated_background: bool,
}

impl Default for MenuConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            theme: "default".to_string(),
            show_settings: true,
            show_credits: true,
            custom_buttons: Vec::new(),
            background_music: None,
            animated_background: false,
        }
    }
}

impl MenuConfig {
    /// Create a simple menu configuration
    pub fn simple() -> Self {
        Self {
            enabled: true,
            theme: "minimal".to_string(),
            show_settings: false,
            show_credits: false,
            custom_buttons: Vec::new(),
            background_music: None,
            animated_background: false,
        }
    }
    
    /// Create a full-featured menu configuration
    pub fn full_featured() -> Self {
        Self {
            enabled: true,
            theme: "deluxe".to_string(),
            show_settings: true,
            show_credits: true,
            custom_buttons: vec![
                CustomButtonConfig::new("Achievements", "show_achievements"),
                CustomButtonConfig::new("Leaderboard", "show_leaderboard"),
            ],
            background_music: Some("assets/music/menu_theme.ogg".to_string()),
            animated_background: true,
        }
    }
    
    /// Validate menu configuration
    pub fn validate(&mut self) {
        // Ensure theme name is not empty
        if self.theme.trim().is_empty() {
            self.theme = "default".to_string();
        }
        
        // Validate custom buttons
        for button in &mut self.custom_buttons {
            button.validate();
        }
    }
}

/// Configuration for a custom menu button
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomButtonConfig {
    /// Display text for the button
    pub text: String,
    /// Action identifier when the button is clicked
    pub action: String,
    /// Whether the button is enabled
    pub enabled: bool,
    /// Optional icon path
    pub icon: Option<String>,
}

impl CustomButtonConfig {
    /// Create a new custom button configuration
    pub fn new(text: impl Into<String>, action: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            action: action.into(),
            enabled: true,
            icon: None,
        }
    }
    
    /// Set the icon for the button
    pub fn with_icon(mut self, icon_path: impl Into<String>) -> Self {
        self.icon = Some(icon_path.into());
        self
    }
    
    /// Set whether the button is enabled
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    
    /// Validate button configuration
    pub fn validate(&mut self) {
        // Ensure text is not empty
        if self.text.trim().is_empty() {
            self.text = "Button".to_string();
        }
        
        // Ensure action is not empty
        if self.action.trim().is_empty() {
            self.action = "default_action".to_string();
        }
    }
}

/// Configuration for the loading screen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadingConfig {
    /// Whether to show loading progress bar
    pub show_progress: bool,
    /// Whether to show loading tips
    pub show_tips: bool,
    /// Background music during loading
    pub background_music: Option<String>,
    /// Minimum loading duration (in milliseconds) for UX purposes
    pub minimum_duration_ms: u64,
    /// Whether to show detailed loading information
    pub show_detailed_info: bool,
    /// List of loading tips to display
    pub tips: Vec<String>,
}

impl Default for LoadingConfig {
    fn default() -> Self {
        Self {
            show_progress: true,
            show_tips: true,
            background_music: None,
            minimum_duration_ms: 1000, // 1 second
            show_detailed_info: false,
            tips: vec![
                "Loading game assets...".to_string(),
                "Preparing your adventure...".to_string(),
                "Almost ready!".to_string(),
            ],
        }
    }
}

impl LoadingConfig {
    /// Create a fast loading configuration (minimal UI)
    pub fn fast() -> Self {
        Self {
            show_progress: false,
            show_tips: false,
            background_music: None,
            minimum_duration_ms: 0,
            show_detailed_info: false,
            tips: Vec::new(),
        }
    }
    
    /// Create a detailed loading configuration
    pub fn detailed() -> Self {
        Self {
            show_progress: true,
            show_tips: true,
            background_music: Some("assets/music/loading_ambient.ogg".to_string()),
            minimum_duration_ms: 2000, // 2 seconds
            show_detailed_info: true,
            tips: vec![
                "Loading textures and models...".to_string(),
                "Initializing game systems...".to_string(),
                "Loading audio assets...".to_string(),
                "Preparing user interface...".to_string(),
                "Finalizing setup...".to_string(),
            ],
        }
    }
    
    /// Validate loading configuration
    pub fn validate(&mut self) {
        // Ensure minimum duration is reasonable
        if self.minimum_duration_ms > 10000 {
            self.minimum_duration_ms = 10000; // Max 10 seconds
        }
        
        // Ensure we have at least one tip if tips are enabled
        if self.show_tips && self.tips.is_empty() {
            self.tips.push("Loading...".to_string());
        }
        
        // Remove empty tips
        self.tips.retain(|tip| !tip.trim().is_empty());
    }
}

/// Configuration for integration with game modules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    /// Whether to auto-detect game modules
    pub auto_detect_modules: bool,
    /// Whether to validate module compatibility
    pub validate_compatibility: bool,
    /// Maximum time to wait for module initialization (in milliseconds)
    pub module_init_timeout_ms: u64,
    /// Whether to show module loading progress
    pub show_module_progress: bool,
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            auto_detect_modules: true,
            validate_compatibility: true,
            module_init_timeout_ms: 5000, // 5 seconds
            show_module_progress: false,
        }
    }
}

impl IntegrationConfig {
    /// Validate integration configuration
    pub fn validate(&mut self) {
        // Ensure timeout is reasonable
        if self.module_init_timeout_ms < 1000 {
            self.module_init_timeout_ms = 1000; // Minimum 1 second
        }
        if self.module_init_timeout_ms > 30000 {
            self.module_init_timeout_ms = 30000; // Maximum 30 seconds
        }
    }
}