use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Extended state system with support for data in enum variants
/// This allows states to carry configuration and context information
/// 
/// Note: PartialEq and Hash are implemented to compare only discriminants,
/// not the data within variants. This allows Bevy's state system to treat
/// states with the same variant but different data as equivalent.
#[derive(Debug, Clone, States, Serialize, Deserialize)]
pub enum AppState {
    /// Initial boot state - system initialization
    Boot,
    
    /// Splash screen state - showing logos and loading critical assets
    Splash,
    
    /// Main menu state with configuration
    MainMenu(MenuConfig),
    
    /// Settings/options menu
    Settings,
    
    /// Loading state with target state and progress information
    Loading(LoadingConfig),
    
    /// In-game state with game configuration
    InGame(GameConfig),
    
    /// Paused game state
    Paused(GameConfig),
    
    /// Game over state
    GameOver(GameConfig),
}

impl Default for AppState {
    fn default() -> Self {
        Self::Boot
    }
}

impl AppState {
    /// Create a MainMenu state with default configuration
    pub fn main_menu() -> Self {
        Self::MainMenu(MenuConfig::default())
    }
    
    /// Create a MainMenu state with custom configuration
    pub fn main_menu_with_config(config: MenuConfig) -> Self {
        Self::MainMenu(config)
    }
    
    /// Create a Loading state targeting a specific state
    pub fn loading_to(target: AppState) -> Self {
        Self::Loading(LoadingConfig::new(target))
    }
    
    /// Create a Loading state with custom configuration
    pub fn loading_with_config(config: LoadingConfig) -> Self {
        Self::Loading(config)
    }
    
    /// Create an InGame state with default configuration
    pub fn in_game() -> Self {
        Self::InGame(GameConfig::default())
    }
    
    /// Create an InGame state with custom configuration
    pub fn in_game_with_config(config: GameConfig) -> Self {
        Self::InGame(config)
    }
    
    /// Create a Paused state from an InGame state
    pub fn paused_from_game(game_config: GameConfig) -> Self {
        Self::Paused(game_config)
    }
    
    /// Create a GameOver state from an InGame state
    pub fn game_over_from_game(game_config: GameConfig) -> Self {
        Self::GameOver(game_config)
    }
    
    /// Get the discriminant of the state (ignoring data)
    pub fn discriminant(&self) -> std::mem::Discriminant<Self> {
        std::mem::discriminant(self)
    }
    
    /// Check if this is a launcher state
    pub fn is_launcher_state(&self) -> bool {
        matches!(self, Self::Boot | Self::Splash | Self::MainMenu(_) | Self::Loading(_) | Self::Settings)
    }
    
    /// Check if this is a game state
    pub fn is_game_state(&self) -> bool {
        matches!(self, Self::InGame(_) | Self::Paused(_) | Self::GameOver(_))
    }
    
    /// Check if two states have the same discriminant (same variant)
    pub fn same_variant(&self, other: &Self) -> bool {
        self == other // Uses our custom PartialEq implementation
    }
    
    /// Get a string representation of the state variant (without data)
    pub fn variant_name(&self) -> &'static str {
        match self {
            Self::Boot => "Boot",
            Self::Splash => "Splash",
            Self::MainMenu(_) => "MainMenu",
            Self::Settings => "Settings",
            Self::Loading(_) => "Loading",
            Self::InGame(_) => "InGame",
            Self::Paused(_) => "Paused",
            Self::GameOver(_) => "GameOver",
        }
    }
}

// Custom PartialEq implementation that compares only discriminants
impl PartialEq for AppState {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

// Eq is automatically implemented since PartialEq is reflexive, symmetric, and transitive
impl Eq for AppState {}

// Custom Hash implementation that hashes only the discriminant
impl std::hash::Hash for AppState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}

/// Configuration for the main menu state
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MenuConfig {
    /// Currently selected menu option
    pub selected_option: MenuOption,
    /// Whether the settings panel is open
    pub settings_open: bool,
    /// Whether to show the credits button
    pub show_credits: bool,
    /// Custom theme for this menu instance
    pub theme: Option<String>,
}

impl Default for MenuConfig {
    fn default() -> Self {
        Self {
            selected_option: MenuOption::NewGame,
            settings_open: false,
            show_credits: true,
            theme: None,
        }
    }
}

/// Available menu options
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MenuOption {
    NewGame,
    LoadGame,
    Settings,
    Credits,
    Quit,
    Custom(String),
}

impl MenuOption {
    /// Get the display text for this menu option
    pub fn display_text(&self) -> &str {
        match self {
            Self::NewGame => "New Game",
            Self::LoadGame => "Load Game",
            Self::Settings => "Settings",
            Self::Credits => "Credits",
            Self::Quit => "Quit",
            Self::Custom(text) => text,
        }
    }
}

/// Configuration for the loading state
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LoadingConfig {
    /// The target state type to transition to after loading
    pub target_state_type: TargetStateType,
    /// Current loading progress (0.0 to 1.0)
    pub progress: u8, // Using u8 (0-100) instead of f32 for Eq/Hash
    /// Whether to show loading tips
    pub show_tips: bool,
    /// Whether to show detailed progress information
    pub show_detailed_progress: bool,
    /// Minimum loading time in milliseconds (for UX purposes)
    pub minimum_duration_ms: u64,
}

/// Simplified target state type for loading
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TargetStateType {
    InGame,
    MainMenu,
    Settings,
}

impl LoadingConfig {
    /// Create a new loading configuration targeting a specific state
    pub fn new(target_state: AppState) -> Self {
        let target_state_type = match target_state {
            AppState::InGame(_) => TargetStateType::InGame,
            AppState::MainMenu(_) => TargetStateType::MainMenu,
            AppState::Settings => TargetStateType::Settings,
            _ => TargetStateType::MainMenu, // Default fallback
        };
        
        Self {
            target_state_type,
            progress: 0,
            show_tips: true,
            show_detailed_progress: false,
            minimum_duration_ms: 1000, // 1 second minimum
        }
    }
    
    /// Get the target state from the configuration
    pub fn target_state(&self) -> AppState {
        match self.target_state_type {
            TargetStateType::InGame => AppState::in_game(),
            TargetStateType::MainMenu => AppState::main_menu(),
            TargetStateType::Settings => AppState::Settings,
        }
    }
    
    /// Set the loading progress (0-100)
    pub fn with_progress(mut self, progress: u8) -> Self {
        self.progress = progress.min(100);
        self
    }
    
    /// Get the progress as a float (0.0 to 1.0)
    pub fn progress_f32(&self) -> f32 {
        self.progress as f32 / 100.0
    }
    
    /// Set the progress from a float (0.0 to 1.0)
    pub fn set_progress_f32(&mut self, progress: f32) {
        self.progress = (progress.clamp(0.0, 1.0) * 100.0) as u8;
    }
    
    /// Check if loading is complete
    pub fn is_complete(&self) -> bool {
        self.progress >= 100
    }
}

/// Configuration for game states
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GameConfig {
    /// Current level or scene identifier
    pub level_id: Option<String>,
    /// Game difficulty setting
    pub difficulty: GameDifficulty,
    /// Whether the game is in debug mode
    pub debug_mode: bool,
    /// Custom game parameters as a sorted vector for Hash compatibility
    pub custom_params: Vec<(String, String)>,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            level_id: None,
            difficulty: GameDifficulty::Normal,
            debug_mode: cfg!(debug_assertions),
            custom_params: Vec::new(),
        }
    }
}

impl GameConfig {
    /// Create a new game configuration for a specific level
    pub fn for_level(level_id: impl Into<String>) -> Self {
        Self {
            level_id: Some(level_id.into()),
            ..Default::default()
        }
    }
    
    /// Set the difficulty level
    pub fn with_difficulty(mut self, difficulty: GameDifficulty) -> Self {
        self.difficulty = difficulty;
        self
    }
    
    /// Enable or disable debug mode
    pub fn with_debug_mode(mut self, debug_mode: bool) -> Self {
        self.debug_mode = debug_mode;
        self
    }
    
    /// Add a custom parameter
    pub fn with_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let key = key.into();
        let value = value.into();
        
        // Remove existing parameter with the same key
        self.custom_params.retain(|(k, _)| k != &key);
        
        // Add new parameter and keep sorted for consistent hashing
        self.custom_params.push((key, value));
        self.custom_params.sort_by(|a, b| a.0.cmp(&b.0));
        
        self
    }
    
    /// Get a custom parameter value
    pub fn get_param(&self, key: &str) -> Option<&str> {
        self.custom_params
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }
    
    /// Get all custom parameters as a HashMap for convenience
    pub fn custom_params_map(&self) -> std::collections::HashMap<String, String> {
        self.custom_params.iter().cloned().collect()
    }
}

/// Game difficulty levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameDifficulty {
    Easy,
    Normal,
    Hard,
    Expert,
    Custom(String),
}

impl GameDifficulty {
    /// Get the display name for this difficulty
    pub fn display_name(&self) -> &str {
        match self {
            Self::Easy => "Easy",
            Self::Normal => "Normal",
            Self::Hard => "Hard",
            Self::Expert => "Expert",
            Self::Custom(name) => name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_discriminant_only_equality() {
        // Test that states with same discriminant but different data are equal
        let menu1 = AppState::MainMenu(MenuConfig {
            selected_option: MenuOption::NewGame,
            settings_open: false,
            show_credits: true,
            theme: None,
        });
        
        let menu2 = AppState::MainMenu(MenuConfig {
            selected_option: MenuOption::LoadGame, // Different data
            settings_open: true,                   // Different data
            show_credits: false,                   // Different data
            theme: Some("dark".to_string()),       // Different data
        });
        
        // With custom PartialEq, these should now be equal
        assert_eq!(menu1, menu2, "States with same discriminant should be equal");
        
        // Test with HashMap - should now have size 1 due to custom Hash
        let mut map = HashMap::new();
        map.insert(menu1.clone(), "first");
        map.insert(menu2.clone(), "second"); // Should overwrite the first entry
        
        assert_eq!(map.len(), 1, "HashMap should have size 1 with discriminant-only hashing");
        assert_eq!(map.get(&menu1), Some(&"second"), "Should find the overwritten value");
        assert_eq!(map.get(&menu2), Some(&"second"), "Should find the same value for both keys");
    }
    
    #[test]
    fn test_different_discriminants_not_equal() {
        let menu = AppState::main_menu();
        let boot = AppState::Boot;
        let splash = AppState::Splash;
        let settings = AppState::Settings;
        
        // Different discriminants should not be equal
        assert_ne!(menu, boot);
        assert_ne!(menu, splash);
        assert_ne!(menu, settings);
        assert_ne!(boot, splash);
        assert_ne!(boot, settings);
        assert_ne!(splash, settings);
    }
    
    #[test]
    fn test_game_states_equality() {
        let game1 = AppState::InGame(GameConfig {
            level_id: Some("level1".to_string()),
            difficulty: GameDifficulty::Easy,
            debug_mode: false,
            custom_params: vec![("param1".to_string(), "value1".to_string())],
        });
        
        let game2 = AppState::InGame(GameConfig {
            level_id: Some("level2".to_string()), // Different data
            difficulty: GameDifficulty::Hard,     // Different data
            debug_mode: true,                     // Different data
            custom_params: vec![("param2".to_string(), "value2".to_string())], // Different data
        });
        
        // Should be equal due to same discriminant
        assert_eq!(game1, game2, "Game states with same discriminant should be equal");
        
        // But different from other states
        let paused = AppState::Paused(GameConfig::default());
        assert_ne!(game1, paused, "InGame and Paused should not be equal");
    }
    
    #[test]
    fn test_loading_states_equality() {
        let loading1 = AppState::Loading(LoadingConfig {
            target_state_type: TargetStateType::InGame,
            progress: 50,
            show_tips: true,
            show_detailed_progress: false,
            minimum_duration_ms: 1000,
        });
        
        let loading2 = AppState::Loading(LoadingConfig {
            target_state_type: TargetStateType::MainMenu, // Different data
            progress: 90,                                  // Different data
            show_tips: false,                              // Different data
            show_detailed_progress: true,                  // Different data
            minimum_duration_ms: 2000,                     // Different data
        });
        
        // Should be equal due to same discriminant
        assert_eq!(loading1, loading2, "Loading states with same discriminant should be equal");
    }
    
    #[test]
    fn test_current_equality_behavior() {
        // Test new behavior - states with same discriminant should be equal
        let menu1 = AppState::MainMenu(MenuConfig {
            selected_option: MenuOption::NewGame,
            settings_open: false,
            show_credits: true,
            theme: None,
        });
        
        let menu2 = AppState::MainMenu(MenuConfig {
            selected_option: MenuOption::LoadGame, // Different data
            settings_open: true,                   // Different data
            show_credits: false,                   // Different data
            theme: Some("dark".to_string()),       // Different data
        });
        
        println!("menu1 == menu2: {}", menu1 == menu2);
        println!("menu1.discriminant() == menu2.discriminant(): {}", 
                 menu1.discriminant() == menu2.discriminant());
        
        // Test with HashMap - should now have size 1 due to custom Hash
        let mut map = HashMap::new();
        map.insert(menu1.clone(), "first");
        map.insert(menu2.clone(), "second");
        
        println!("HashMap size: {}", map.len());
        println!("Can find menu1: {}", map.contains_key(&menu1));
        println!("Can find menu2: {}", map.contains_key(&menu2));
        
        // With custom implementations, both should be true and equal
        assert_eq!(menu1, menu2);
        assert_eq!(map.len(), 1);
    }
    
    #[test]
    fn test_helper_methods() {
        let menu1 = AppState::main_menu();
        let menu2 = AppState::main_menu_with_config(MenuConfig {
            selected_option: MenuOption::LoadGame,
            settings_open: true,
            show_credits: false,
            theme: Some("custom".to_string()),
        });
        let boot = AppState::Boot;
        
        // Test same_variant method
        assert!(menu1.same_variant(&menu2), "Same variants should return true");
        assert!(!menu1.same_variant(&boot), "Different variants should return false");
        
        // Test variant_name method
        assert_eq!(menu1.variant_name(), "MainMenu");
        assert_eq!(menu2.variant_name(), "MainMenu");
        assert_eq!(boot.variant_name(), "Boot");
        assert_eq!(AppState::Splash.variant_name(), "Splash");
        assert_eq!(AppState::Settings.variant_name(), "Settings");
        assert_eq!(AppState::in_game().variant_name(), "InGame");
        assert_eq!(AppState::paused_from_game(GameConfig::default()).variant_name(), "Paused");
        assert_eq!(AppState::game_over_from_game(GameConfig::default()).variant_name(), "GameOver");
        assert_eq!(AppState::loading_to(AppState::in_game()).variant_name(), "Loading");
    }
    
    #[test]
    fn test_discriminant_method() {
        let menu1 = AppState::main_menu();
        let menu2 = AppState::main_menu_with_config(MenuConfig {
            selected_option: MenuOption::LoadGame,
            settings_open: true,
            show_credits: false,
            theme: Some("custom".to_string()),
        });
        
        // The discriminant method should return the same value for both
        assert_eq!(menu1.discriminant(), menu2.discriminant());
        
        // With custom PartialEq, the states themselves should now be equal
        assert_eq!(menu1, menu2, "States with same discriminant should be equal");
    }
}