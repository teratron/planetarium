use bevy::prelude::*;
use bevy::asset::AssetPath;

use super::AppState;

/// Trait that game modules must implement to integrate with the launcher
pub trait GameModule: Plugin {
    /// Returns the initial game state that the launcher should transition to
    fn initial_game_state(&self) -> AppState;
    
    /// Registers game-specific states with the app
    fn register_game_states(&self, app: &mut App);
    
    /// Returns the list of assets that need to be loaded before starting the game
    fn required_assets(&self) -> Vec<AssetPath<'static>>;
    
    /// Returns metadata describing this game module
    fn descriptor(&self) -> GameModuleDescriptor;
    
    /// Called when the launcher is ready to transition to the game
    /// This allows the game module to perform any final setup
    fn on_game_start(&self, world: &mut World) {
        // Default implementation does nothing
        let _ = world;
    }
    
    /// Called when transitioning back to launcher states (e.g., returning to main menu)
    fn on_game_exit(&self, world: &mut World) {
        // Default implementation does nothing
        let _ = world;
    }
}

/// Describes the type of game for launcher optimization
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameType {
    /// 2D games (sprites, tilemaps, etc.)
    TwoD,
    /// 3D games (meshes, scenes, etc.)
    ThreeD,
    /// Games that use both 2D and 3D elements
    Mixed,
}

/// Metadata describing a game module
#[derive(Debug, Clone)]
pub struct GameModuleDescriptor {
    /// Human-readable name of the game
    pub name: String,
    /// Version of the game module
    pub version: String,
    /// Type of game (2D, 3D, Mixed)
    pub game_type: GameType,
    /// List of required features/capabilities
    pub required_features: Vec<String>,
    /// Minimum launcher version required
    pub min_launcher_version: String,
}

impl GameModuleDescriptor {
    /// Create a new game module descriptor
    pub fn new(name: impl Into<String>, version: impl Into<String>, game_type: GameType) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            game_type,
            required_features: Vec::new(),
            min_launcher_version: "0.1.0".to_string(),
        }
    }
    
    /// Add a required feature to the descriptor
    pub fn with_feature(mut self, feature: impl Into<String>) -> Self {
        self.required_features.push(feature.into());
        self
    }
    
    /// Set the minimum launcher version required
    pub fn with_min_launcher_version(mut self, version: impl Into<String>) -> Self {
        self.min_launcher_version = version.into();
        self
    }
}

/// Trait for customizing launcher behavior through hooks
pub trait LauncherHooks: Send + Sync + 'static {
    /// Called when entering the Boot state
    fn on_boot_enter(&self) {}
    
    /// Called when exiting the Boot state
    fn on_boot_exit(&self) {}
    
    /// Called when entering the Splash state
    fn on_splash_enter(&self) {}
    
    /// Called when the splash screen timer completes
    fn on_splash_complete(&self) {}
    
    /// Called when entering the MainMenu state
    fn on_main_menu_enter(&self) {}
    
    /// Called when a menu option is selected
    fn on_menu_selection(&self, selection: &str) {
        let _ = selection;
    }
    
    /// Called when entering the Loading state
    fn on_loading_enter(&self) {}
    
    /// Called when loading progress updates
    fn on_loading_progress(&self, progress: f32) {
        let _ = progress;
    }
    
    /// Called when loading is complete
    fn on_loading_complete(&self) {}
}

/// Default implementation of LauncherHooks that does nothing
#[derive(Default)]
pub struct DefaultLauncherHooks;

impl LauncherHooks for DefaultLauncherHooks {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::launcher::states::{GameConfig, MenuConfig, MenuOption, LoadingConfig};

    #[test]
    fn test_state_transition_manager_creation() {
        let manager = StateTransitionManager::new();
        assert!(!manager.are_transitions_locked());
        assert!(!manager.is_in_recovery_mode());
        assert_eq!(manager.get_failed_transition_count(), 0);
        assert!(manager.get_recovery_state().is_some());
    }

    #[test]
    fn test_custom_recovery_state() {
        let recovery_state = AppState::main_menu();
        let manager = StateTransitionManager::with_recovery_state(recovery_state.clone());
        assert_eq!(manager.get_recovery_state(), Some(&recovery_state));
    }

    #[test]
    fn test_valid_transitions() {
        let manager = StateTransitionManager::new();
        
        // Test basic launcher flow
        assert!(manager.is_valid_transition(&AppState::Boot, &AppState::Splash));
        assert!(manager.is_valid_transition(&AppState::Boot, &AppState::main_menu()));
        assert!(manager.is_valid_transition(&AppState::Splash, &AppState::main_menu()));
        assert!(manager.is_valid_transition(&AppState::main_menu(), &AppState::Settings));
        
        // Test loading transitions
        let loading_state = AppState::loading_to(AppState::in_game());
        assert!(manager.is_valid_transition(&AppState::main_menu(), &loading_state));
        assert!(manager.is_valid_transition(&loading_state, &AppState::in_game()));
        
        // Test game state transitions
        assert!(manager.is_valid_transition(&AppState::in_game(), &AppState::main_menu()));
        assert!(manager.is_valid_transition(&AppState::in_game(), &AppState::paused_from_game(GameConfig::default())));
        
        // Test restart transitions
        assert!(manager.is_valid_transition(&AppState::in_game(), &AppState::Boot));
        assert!(manager.is_valid_transition(&AppState::main_menu(), &AppState::Boot));
    }

    #[test]
    fn test_invalid_transitions() {
        let manager = StateTransitionManager::new();
        
        // Test invalid transitions
        assert!(!manager.is_valid_transition(&AppState::Splash, &AppState::in_game()));
        assert!(!manager.is_valid_transition(&AppState::Settings, &AppState::Splash));
        assert!(!manager.is_valid_transition(&AppState::in_game(), &AppState::Splash));
    }

    #[test]
    fn test_same_variant_transitions() {
        let manager = StateTransitionManager::new();
        
        let menu1 = AppState::main_menu();
        let menu2 = AppState::main_menu_with_config(MenuConfig {
            selected_option: MenuOption::LoadGame,
            settings_open: true,
            show_credits: false,
            theme: Some("dark".to_string()),
        });
        
        // Should allow transitions between same variants with different data
        assert!(manager.is_valid_transition(&menu1, &menu2));
        assert!(manager.is_valid_transition(&menu2, &menu1));
    }

    #[test]
    fn test_failed_transition_recovery() {
        let mut manager = StateTransitionManager::new();
        let from_state = AppState::main_menu();
        let invalid_to_state = AppState::Splash; // Invalid transition
        
        // First few failed transitions should not trigger recovery
        assert!(manager.record_failed_transition(&from_state, &invalid_to_state).is_none());
        assert!(manager.record_failed_transition(&from_state, &invalid_to_state).is_none());
        assert_eq!(manager.get_failed_transition_count(), 2);
        assert!(!manager.is_in_recovery_mode());
        
        // Third failed transition should trigger recovery
        let recovery_state = manager.record_failed_transition(&from_state, &invalid_to_state);
        assert!(recovery_state.is_some());
        assert!(manager.is_in_recovery_mode());
        assert_eq!(manager.get_failed_transition_count(), 0); // Reset after recovery
    }

    #[test]
    fn test_recovery_mode_transitions() {
        let mut manager = StateTransitionManager::new();
        
        // Trigger recovery mode
        let from_state = AppState::main_menu();
        let invalid_to_state = AppState::Splash;
        for _ in 0..3 {
            manager.record_failed_transition(&from_state, &invalid_to_state);
        }
        assert!(manager.is_in_recovery_mode());
        
        // In recovery mode, only safe transitions should be allowed
        assert!(manager.is_valid_transition(&AppState::main_menu(), &AppState::Boot));
        assert!(manager.is_valid_transition(&AppState::Boot, &AppState::main_menu()));
        
        // Unsafe transitions should be blocked
        assert!(!manager.is_valid_transition(&AppState::main_menu(), &AppState::in_game()));
    }

    #[test]
    fn test_state_validation() {
        let manager = StateTransitionManager::new();
        
        // Valid states
        assert!(manager.validate_state(&AppState::Boot).is_ok());
        assert!(manager.validate_state(&AppState::main_menu()).is_ok());
        
        // Invalid loading state
        let mut invalid_loading = LoadingConfig::new(AppState::main_menu());
        invalid_loading.progress = 150; // Invalid progress > 100
        let invalid_state = AppState::Loading(invalid_loading);
        assert!(manager.validate_state(&invalid_state).is_err());
        
        // Invalid menu state
        let invalid_menu = AppState::MainMenu(MenuConfig {
            selected_option: MenuOption::NewGame,
            settings_open: false,
            show_credits: true,
            theme: Some("".to_string()), // Empty theme string is invalid
        });
        assert!(manager.validate_state(&invalid_menu).is_err());
    }

    #[test]
    fn test_state_recovery() {
        let manager = StateTransitionManager::new();
        
        // Test recovery of invalid states
        let mut invalid_loading = LoadingConfig::new(AppState::main_menu());
        invalid_loading.progress = 150;
        let invalid_state = AppState::Loading(invalid_loading);
        
        let recovered = manager.recover_invalid_state(&invalid_state);
        assert!(manager.validate_state(&recovered).is_ok());
        assert!(matches!(recovered, AppState::Loading(_)));
    }

    #[test]
    fn test_transition_history() {
        let mut manager = StateTransitionManager::new();
        
        // Record some transitions
        manager.record_transition(AppState::Boot, AppState::Splash);
        manager.record_transition(AppState::Splash, AppState::main_menu());
        
        // Check history
        assert_eq!(manager.peek_previous_state(), Some(&AppState::Splash));
        assert_eq!(manager.get_previous_state(), Some(AppState::Splash));
        assert_eq!(manager.get_previous_state(), Some(AppState::Boot));
        assert_eq!(manager.get_previous_state(), None);
    }

    #[test]
    fn test_transition_locking() {
        let mut manager = StateTransitionManager::new();
        
        assert!(!manager.are_transitions_locked());
        
        manager.lock_transitions();
        assert!(manager.are_transitions_locked());
        
        manager.unlock_transitions();
        assert!(!manager.are_transitions_locked());
    }

    #[test]
    fn test_last_known_good_state() {
        let mut manager = StateTransitionManager::new();
        
        // Record transition to stable state
        let stable_state = AppState::main_menu();
        manager.record_transition(AppState::Boot, stable_state.clone());
        
        assert_eq!(manager.get_last_known_good_state(), Some(&stable_state));
        
        // Manually set last known good state
        let game_state = AppState::in_game();
        manager.set_last_known_good_state(game_state.clone());
        assert_eq!(manager.get_last_known_good_state(), Some(&game_state));
    }
}

/// Resource for managing state transitions and validation
#[derive(Resource)]
pub struct StateTransitionManager {
    /// Stack of previous states for back navigation
    state_history: Vec<AppState>,
    /// Maximum size of the state history stack
    max_history_size: usize,
    /// Whether transitions are currently locked (e.g., during loading)
    transitions_locked: bool,
    /// Current recovery state for error handling
    recovery_state: Option<AppState>,
    /// Number of consecutive failed transitions
    failed_transition_count: u32,
    /// Maximum allowed failed transitions before forcing recovery
    max_failed_transitions: u32,
    /// Whether we're currently in recovery mode
    in_recovery_mode: bool,
    /// Last known good state for recovery purposes
    last_known_good_state: Option<AppState>,
}

impl Default for StateTransitionManager {
    fn default() -> Self {
        Self {
            state_history: Vec::new(),
            max_history_size: 10,
            transitions_locked: false,
            recovery_state: Some(AppState::Boot), // Default recovery state
            failed_transition_count: 0,
            max_failed_transitions: 3,
            in_recovery_mode: false,
            last_known_good_state: None,
        }
    }
}

impl StateTransitionManager {
    /// Create a new state transition manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create a new state transition manager with custom recovery state
    pub fn with_recovery_state(recovery_state: AppState) -> Self {
        Self {
            recovery_state: Some(recovery_state),
            ..Default::default()
        }
    }
    
    /// Check if a state transition is valid
    pub fn is_valid_transition(&self, from: &AppState, to: &AppState) -> bool {
        use AppState::*;
        
        // If we're in recovery mode, allow transitions to recovery state or safe states
        if self.in_recovery_mode {
            return self.is_safe_recovery_transition(from, to);
        }
        
        match (from, to) {
            // Any state can transition to Boot (for restart) or recovery state - check this first
            (_, Boot) => true,
            (_, state) if Some(state) == self.recovery_state.as_ref() => true,
            
            // Allow transitions to the same state (for state updates with different data)
            (from_state, to_state) if from_state.same_variant(to_state) => true,
            
            // Boot can transition to Splash or MainMenu
            (Boot, Splash) | (Boot, MainMenu(_)) => true,
            
            // Splash can transition to MainMenu
            (Splash, MainMenu(_)) => true,
            
            // MainMenu can transition to Loading or Settings
            (MainMenu(_), Loading(_)) | (MainMenu(_), Settings) => true,
            
            // Loading can transition to any game state or back to MainMenu on error
            (Loading(_), state) if self.is_game_state(state) => true,
            (Loading(_), MainMenu(_)) => true, // Allow fallback to menu on loading error
            
            // Settings can return to MainMenu
            (Settings, MainMenu(_)) => true,
            
            // Game states can transition to each other or back to MainMenu
            (from_state, to_state) if self.is_game_state(from_state) => {
                self.is_game_state(to_state) || matches!(to_state, MainMenu(_) | Loading(_))
            }
            
            _ => false,
        }
    }
    
    /// Check if a transition is safe during recovery mode
    fn is_safe_recovery_transition(&self, from: &AppState, to: &AppState) -> bool {
        use AppState::*;
        
        match to {
            // Always allow transitions to recovery state
            state if Some(state) == self.recovery_state.as_ref() => true,
            // Allow transitions to safe launcher states
            Boot | MainMenu(_) => true,
            // Allow transitions from recovery state to any launcher state
            Splash | Settings if matches!(from, Boot | MainMenu(_)) => true,
            // Allow loading only if going to safe states
            Loading(config) => matches!(config.target_state_type, crate::launcher::states::TargetStateType::MainMenu),
            _ => false,
        }
    }
    
    /// Check if a state is a game state (not a launcher state)
    pub fn is_game_state(&self, state: &AppState) -> bool {
        matches!(state, AppState::InGame(_) | AppState::Paused(_) | AppState::GameOver(_))
    }
    
    /// Check if a state is a launcher state
    pub fn is_launcher_state(&self, state: &AppState) -> bool {
        matches!(
            state,
            AppState::Boot | AppState::Splash | AppState::MainMenu(_) | AppState::Loading(_) | AppState::Settings
        )
    }
    
    /// Record a successful state transition in history
    pub fn record_transition(&mut self, from: AppState, to: AppState) {
        if self.is_valid_transition(&from, &to) {
            // Reset failed transition count on successful transition
            self.failed_transition_count = 0;
            
            // Update last known good state if this is a stable state
            if self.is_stable_state(&to) {
                self.last_known_good_state = Some(to.clone());
            }
            
            // Exit recovery mode if we successfully transitioned to a safe state
            if self.in_recovery_mode && self.is_launcher_state(&to) {
                self.in_recovery_mode = false;
                info!("Exited recovery mode, transitioned to safe state: {:?}", to);
            }
            
            self.state_history.push(from);
            
            // Limit history size
            if self.state_history.len() > self.max_history_size {
                self.state_history.remove(0);
            }
        }
    }
    
    /// Record a failed state transition and potentially trigger recovery
    pub fn record_failed_transition(&mut self, from: &AppState, to: &AppState) -> Option<AppState> {
        self.failed_transition_count += 1;
        warn!(
            "Failed state transition #{}: {:?} -> {:?}", 
            self.failed_transition_count, from, to
        );
        
        // If we've exceeded the maximum failed transitions, trigger recovery
        if self.failed_transition_count >= self.max_failed_transitions {
            self.trigger_recovery_mode(from)
        } else {
            None
        }
    }
    
    /// Trigger recovery mode and return the recovery state
    fn trigger_recovery_mode(&mut self, current_state: &AppState) -> Option<AppState> {
        self.in_recovery_mode = true;
        
        warn!(
            "Triggering recovery mode after {} failed transitions from state: {:?}", 
            self.failed_transition_count, current_state
        );
        
        // Determine the best recovery state
        let recovery_target = self.determine_recovery_state(current_state);
        
        // Reset failed transition count
        self.failed_transition_count = 0;
        
        Some(recovery_target)
    }
    
    /// Determine the best recovery state based on current situation
    fn determine_recovery_state(&self, current_state: &AppState) -> AppState {
        // Priority order for recovery:
        // 1. Configured recovery state
        // 2. Last known good state (if it's a launcher state)
        // 3. MainMenu as fallback
        // 4. Boot as last resort
        
        if let Some(recovery_state) = &self.recovery_state {
            return recovery_state.clone();
        }
        
        if let Some(last_good) = &self.last_known_good_state {
            if self.is_launcher_state(last_good) {
                return last_good.clone();
            }
        }
        
        // If current state is a game state, go to main menu
        if self.is_game_state(current_state) {
            return AppState::main_menu();
        }
        
        // Default to Boot for complete restart
        AppState::Boot
    }
    
    /// Check if a state is considered stable (good for recovery purposes)
    fn is_stable_state(&self, state: &AppState) -> bool {
        matches!(
            state,
            AppState::Boot | AppState::MainMenu(_) | AppState::InGame(_)
        )
    }
    
    /// Get the previous state for back navigation
    pub fn get_previous_state(&mut self) -> Option<AppState> {
        self.state_history.pop()
    }
    
    /// Get the previous state without removing it from history
    pub fn peek_previous_state(&self) -> Option<&AppState> {
        self.state_history.last()
    }
    
    /// Lock state transitions (e.g., during loading)
    pub fn lock_transitions(&mut self) {
        self.transitions_locked = true;
        info!("State transitions locked");
    }
    
    /// Unlock state transitions
    pub fn unlock_transitions(&mut self) {
        self.transitions_locked = false;
        info!("State transitions unlocked");
    }
    
    /// Check if transitions are currently locked
    pub fn are_transitions_locked(&self) -> bool {
        self.transitions_locked
    }
    
    /// Check if we're currently in recovery mode
    pub fn is_in_recovery_mode(&self) -> bool {
        self.in_recovery_mode
    }
    
    /// Force exit from recovery mode (use with caution)
    pub fn force_exit_recovery_mode(&mut self) {
        self.in_recovery_mode = false;
        self.failed_transition_count = 0;
        info!("Forced exit from recovery mode");
    }
    
    /// Set a custom recovery state
    pub fn set_recovery_state(&mut self, state: AppState) {
        self.recovery_state = Some(state);
    }
    
    /// Clear the recovery state (will use default recovery logic)
    pub fn clear_recovery_state(&mut self) {
        self.recovery_state = None;
    }
    
    /// Get the current recovery state
    pub fn get_recovery_state(&self) -> Option<&AppState> {
        self.recovery_state.as_ref()
    }
    
    /// Clear the state history
    pub fn clear_history(&mut self) {
        self.state_history.clear();
    }
    
    /// Get the number of failed transitions
    pub fn get_failed_transition_count(&self) -> u32 {
        self.failed_transition_count
    }
    
    /// Reset the failed transition count
    pub fn reset_failed_transition_count(&mut self) {
        self.failed_transition_count = 0;
    }
    
    /// Get the last known good state
    pub fn get_last_known_good_state(&self) -> Option<&AppState> {
        self.last_known_good_state.as_ref()
    }
    
    /// Manually set the last known good state
    pub fn set_last_known_good_state(&mut self, state: AppState) {
        self.last_known_good_state = Some(state);
    }
    
    /// Validate the current state configuration
    pub fn validate_state(&self, state: &AppState) -> Result<(), String> {
        match state {
            AppState::Loading(config) => {
                // Validate loading configuration
                if config.progress > 100 {
                    return Err("Loading progress cannot exceed 100%".to_string());
                }
                Ok(())
            }
            AppState::MainMenu(config) => {
                // Validate menu configuration
                if config.theme.as_ref().map_or(false, |t| t.is_empty()) {
                    return Err("Menu theme cannot be empty string".to_string());
                }
                Ok(())
            }
            AppState::InGame(config) | AppState::Paused(config) | AppState::GameOver(config) => {
                // Validate game configuration
                if let Some(level_id) = &config.level_id {
                    if level_id.is_empty() {
                        return Err("Level ID cannot be empty string".to_string());
                    }
                }
                Ok(())
            }
            _ => Ok(()), // Other states don't need validation
        }
    }
    
    /// Attempt to recover from an invalid state by creating a valid variant
    pub fn recover_invalid_state(&self, invalid_state: &AppState) -> AppState {
        match invalid_state {
            AppState::MainMenu(_) => AppState::main_menu(), // Use default config
            AppState::Loading(_) => AppState::loading_to(AppState::main_menu()), // Safe loading target
            AppState::InGame(_) => AppState::in_game(), // Use default game config
            AppState::Paused(_) => AppState::paused_from_game(crate::launcher::states::GameConfig::default()),
            AppState::GameOver(_) => AppState::game_over_from_game(crate::launcher::states::GameConfig::default()),
            // Simple states don't need recovery
            state => state.clone(),
        }
    }
}