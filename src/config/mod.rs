use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

// ============================================================================
// Configuration Structures
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GraphicsQuality {
    Low,
    Medium,
    High,
    Ultra,
}

impl Default for GraphicsQuality {
    fn default() -> Self {
        Self::High
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct GraphicsConfig {
    pub resolution: (u32, u32),
    pub fullscreen: bool,
    pub vsync: bool,
    pub quality: GraphicsQuality,
}

impl Default for GraphicsConfig {
    fn default() -> Self {
        Self {
            resolution: (1280, 720),
            fullscreen: false,
            vsync: true,
            quality: GraphicsQuality::default(),
        }
    }
}

impl GraphicsConfig {
    /// Validate and clamp values to safe ranges
    pub fn validate(&mut self) {
        // Clamp resolution to reasonable bounds
        self.resolution.0 = self.resolution.0.clamp(640, 7680);
        self.resolution.1 = self.resolution.1.clamp(480, 4320);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct AudioConfig {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            music_volume: 0.7,
            sfx_volume: 0.8,
        }
    }
}

impl AudioConfig {
    /// Validate and clamp volume values to 0.0-1.0 range
    pub fn validate(&mut self) {
        self.master_volume = self.master_volume.clamp(0.0, 1.0);
        self.music_volume = self.music_volume.clamp(0.0, 1.0);
        self.sfx_volume = self.sfx_volume.clamp(0.0, 1.0);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct GameConfig {
    pub version: u32,
    pub graphics: GraphicsConfig,
    pub audio: AudioConfig,
    pub language: String,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            version: 1,
            graphics: GraphicsConfig::default(),
            audio: AudioConfig::default(),
            language: "en".to_string(),
        }
    }
}

impl GameConfig {
    /// Validate all nested configs
    pub fn validate(&mut self) {
        self.graphics.validate();
        self.audio.validate();
        
        // Ensure language code is not empty
        if self.language.trim().is_empty() {
            self.language = "en".to_string();
        }
    }
}

// ============================================================================
// Path Resolution
// ============================================================================

/// Get the cross-platform path to user config file
pub fn get_user_config_path() -> PathBuf {
    use directories::ProjectDirs;
    
    if let Some(proj_dirs) = ProjectDirs::from("com", "Teratron", "Planetarium") {
        let config_dir = proj_dirs.config_dir();
        config_dir.join("config.toml")
    } else {
        // Fallback for portable versions
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("config.toml")
    }
}

// ============================================================================
// Configuration Loading & Saving
// ============================================================================

/// Load configuration from disk, falling back to defaults on error
pub fn load_config() -> GameConfig {
    let config_path = get_user_config_path();
    
    if !config_path.exists() {
        info!("Config file not found, creating default at {:?}", config_path);
        let default_config = GameConfig::default();
        let _ = save_config(&default_config);
        return default_config;
    }
    
    match fs::read_to_string(&config_path) {
        Ok(content) => {
            match toml::from_str::<GameConfig>(&content) {
                Ok(mut config) => {
                    // Migrate if needed
                    config = migrate_config(config);
                    config.validate();
                    info!("Loaded config from {:?}", config_path);
                    config
                }
                Err(e) => {
                    error!("Failed to parse config: {}. Backing up and using defaults.", e);
                    backup_corrupt_config(&config_path);
                    GameConfig::default()
                }
            }
        }
        Err(e) => {
            error!("Failed to read config file: {}. Using defaults.", e);
            GameConfig::default()
        }
    }
}

/// Save configuration to disk atomically
pub fn save_config(config: &GameConfig) -> io::Result<()> {
    let config_path = get_user_config_path();
    
    // Ensure parent directory exists
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Serialize to TOML
    let toml_content = toml::to_string_pretty(config)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    
    // Atomic save: write to .tmp then rename
    let tmp_path = config_path.with_extension("toml.tmp");
    fs::write(&tmp_path, toml_content)?;
    fs::rename(&tmp_path, &config_path)?;
    
    info!("Saved config to {:?}", config_path);
    Ok(())
}

// ============================================================================
// Migration & Error Handling
// ============================================================================

/// Migrate configuration from older versions
pub fn migrate_config(mut config: GameConfig) -> GameConfig {
    const CURRENT_VERSION: u32 = 1;
    
    if config.version < CURRENT_VERSION {
        info!("Migrating config from version {} to {}", config.version, CURRENT_VERSION);
        
        // Future migrations will go here
        // Example:
        // if config.version < 2 {
        //     // Add new field with default value
        //     config.some_new_field = default_value;
        // }
        
        config.version = CURRENT_VERSION;
    }
    
    config
}

/// Backup corrupt config file to .bak
fn backup_corrupt_config(config_path: &PathBuf) {
    let backup_path = config_path.with_extension("toml.bak");
    if let Err(e) = fs::copy(config_path, &backup_path) {
        error!("Failed to backup corrupt config: {}", e);
    } else {
        info!("Backed up corrupt config to {:?}", backup_path);
    }
}

// ============================================================================
// Developer Configuration (Debug Only)
// ============================================================================

#[cfg(debug_assertions)]
#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct DevConfig {
    pub show_fps: bool,
    pub show_gizmos: bool,
    pub hot_reload: bool,
}

#[cfg(debug_assertions)]
impl Default for DevConfig {
    fn default() -> Self {
        Self {
            show_fps: true,
            show_gizmos: false,
            hot_reload: true,
        }
    }
}

#[cfg(debug_assertions)]
pub fn load_dev_config() -> DevConfig {
    let dev_config_path = PathBuf::from("assets/config/dev_config.ron");
    
    if !dev_config_path.exists() {
        info!("Dev config not found, creating default at {:?}", dev_config_path);
        let default_config = DevConfig::default();
        let _ = save_dev_config(&default_config);
        return default_config;
    }
    
    match fs::read_to_string(&dev_config_path) {
        Ok(content) => {
            match ron::from_str::<DevConfig>(&content) {
                Ok(config) => {
                    info!("Loaded dev config from {:?}", dev_config_path);
                    config
                }
                Err(e) => {
                    error!("Failed to parse dev config: {}. Using defaults.", e);
                    DevConfig::default()
                }
            }
        }
        Err(e) => {
            error!("Failed to read dev config: {}. Using defaults.", e);
            DevConfig::default()
        }
    }
}

#[cfg(debug_assertions)]
pub fn save_dev_config(config: &DevConfig) -> io::Result<()> {
    let dev_config_path = PathBuf::from("assets/config/dev_config.ron");
    
    if let Some(parent) = dev_config_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    let ron_content = ron::ser::to_string_pretty(config, ron::ser::PrettyConfig::default())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    
    fs::write(&dev_config_path, ron_content)?;
    info!("Saved dev config to {:?}", dev_config_path);
    Ok(())
}


// ============================================================================
// Bevy Plugin
// ============================================================================

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        // Load config at startup
        let config = load_config();
        app.insert_resource(config);
        
        // Add reactive systems
        app.add_systems(Update, (
            apply_graphics_settings,
            apply_audio_settings,
            save_config_on_change,
        ));
        
        // Debug-only features
        #[cfg(debug_assertions)]
        {
            let dev_config = load_dev_config();
            app.insert_resource(dev_config);
            app.add_systems(Update, (
                apply_dev_settings,
                hot_reload_config,
            ));
        }
    }
}

// ============================================================================
// Reactive Systems
// ============================================================================

/// Apply graphics settings when config changes
fn apply_graphics_settings(
    config: Res<GameConfig>,
    mut windows: Query<&mut Window>,
) {
    if !config.is_changed() {
        return;
    }
    
    for mut window in windows.iter_mut() {
        let (width, height) = config.graphics.resolution;
        window.resolution.set(width as f32, height as f32);
        
        window.mode = if config.graphics.fullscreen {
            bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary)
        } else {
            bevy::window::WindowMode::Windowed
        };
        
        window.present_mode = if config.graphics.vsync {
            bevy::window::PresentMode::AutoVsync
        } else {
            bevy::window::PresentMode::AutoNoVsync
        };
    }
    
    info!("Applied graphics settings: {:?}", config.graphics);
}

/// Apply audio settings when config changes
fn apply_audio_settings(
    config: Res<GameConfig>,
    // TODO: Add audio sink queries when audio system is implemented
) {
    if !config.is_changed() {
        return;
    }
    
    // Placeholder for audio volume application
    // In a real implementation, you would update AudioSink volumes here
    info!("Applied audio settings: master={}, music={}, sfx={}", 
        config.audio.master_volume,
        config.audio.music_volume,
        config.audio.sfx_volume
    );
}

/// Save config to disk when it changes (Memory Wins strategy)
fn save_config_on_change(
    config: Res<GameConfig>,
) {
    if config.is_changed() && !config.is_added() {
        if let Err(e) = save_config(&*config) {
            error!("Failed to save config: {}", e);
        }
    }
}

// ============================================================================
// Debug-Only Systems
// ============================================================================

#[cfg(debug_assertions)]
fn apply_dev_settings(
    dev_config: Res<DevConfig>,
    mut gizmo_config: ResMut<GizmoConfigStore>,
) {
    if !dev_config.is_changed() {
        return;
    }
    
    // Toggle gizmos visibility
    for (_, config, _) in gizmo_config.iter_mut() {
        config.enabled = dev_config.show_gizmos;
    }
    
    info!("Applied dev settings: {:?}", *dev_config);
}

#[cfg(debug_assertions)]
fn hot_reload_config(
    mut config: ResMut<GameConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::F5) {
        let new_config = load_config();
        *config = new_config;
        info!("Config hot-reloaded!");
    }
}


// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_validation_clamps_volumes() {
        let mut audio = AudioConfig {
            master_volume: 1.5,
            music_volume: -0.2,
            sfx_volume: 0.5,
        };
        
        audio.validate();
        
        assert_eq!(audio.master_volume, 1.0);
        assert_eq!(audio.music_volume, 0.0);
        assert_eq!(audio.sfx_volume, 0.5);
    }

    #[test]
    fn test_graphics_validation_clamps_resolution() {
        let mut graphics = GraphicsConfig {
            resolution: (320, 240),
            fullscreen: false,
            vsync: true,
            quality: GraphicsQuality::Low,
        };
        
        graphics.validate();
        
        assert_eq!(graphics.resolution, (640, 480));
    }

    #[test]
    fn test_graphics_validation_clamps_high_resolution() {
        let mut graphics = GraphicsConfig {
            resolution: (10000, 10000),
            fullscreen: false,
            vsync: true,
            quality: GraphicsQuality::Ultra,
        };
        
        graphics.validate();
        
        assert_eq!(graphics.resolution, (7680, 4320));
    }

    #[test]
    fn test_config_migration_updates_version() {
        let old_config = GameConfig {
            version: 0,
            ..Default::default()
        };
        
        let migrated = migrate_config(old_config);
        
        assert_eq!(migrated.version, 1);
    }

    #[test]
    fn test_config_validation_fixes_empty_language() {
        let mut config = GameConfig {
            language: "   ".to_string(),
            ..Default::default()
        };
        
        config.validate();
        
        assert_eq!(config.language, "en");
    }

    #[test]
    fn test_default_config_is_valid() {
        let mut config = GameConfig::default();
        config.validate();
        
        // Should not panic or change values
        assert_eq!(config.version, 1);
        assert_eq!(config.graphics.resolution, (1280, 720));
        assert_eq!(config.audio.master_volume, 1.0);
    }

    #[test]
    fn test_config_serialization_roundtrip() {
        let original = GameConfig::default();
        let toml_str = toml::to_string(&original).unwrap();
        let deserialized: GameConfig = toml::from_str(&toml_str).unwrap();
        
        assert_eq!(original.version, deserialized.version);
        assert_eq!(original.graphics.resolution, deserialized.graphics.resolution);
        assert_eq!(original.language, deserialized.language);
    }
}
