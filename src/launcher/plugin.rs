use bevy::prelude::*;
use std::sync::Arc;

use super::{
    config::LauncherConfig, states::AppState, traits::{GameModule, LauncherHooks, StateTransitionManager, DefaultLauncherHooks}
};

/// Main plugin for the Game Launcher system
/// 
/// This plugin provides a complete launcher system with Boot, Splash, MainMenu, and Loading states.
/// It's designed to be reusable across different games and supports both 2D and 3D projects.
/// 
/// # Example
/// 
/// ```rust
/// use bevy::prelude::*;
/// use game_launcher::prelude::*;
/// 
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_plugins(LauncherPlugin::default())
///         .run();
/// }
/// ```
pub struct LauncherPlugin {
    /// Configuration for the launcher
    config: LauncherConfig,
    /// Optional hooks for customizing launcher behavior
    hooks: Arc<dyn LauncherHooks>,
    /// Whether to automatically start the launcher sequence
    auto_start: bool,
}

impl Default for LauncherPlugin {
    fn default() -> Self {
        Self {
            config: LauncherConfig::default(),
            hooks: Arc::new(DefaultLauncherHooks),
            auto_start: true,
        }
    }
}

impl LauncherPlugin {
    /// Create a new launcher plugin with default configuration
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create a launcher plugin with custom configuration
    pub fn with_config(config: LauncherConfig) -> Self {
        Self {
            config,
            hooks: Arc::new(DefaultLauncherHooks),
            auto_start: true,
        }
    }
    
    /// Set custom hooks for the launcher
    pub fn with_hooks<H: LauncherHooks>(mut self, hooks: H) -> Self {
        self.hooks = Arc::new(hooks);
        self
    }
    
    /// Set whether to automatically start the launcher sequence
    pub fn with_auto_start(mut self, auto_start: bool) -> Self {
        self.auto_start = auto_start;
        self
    }
    
    /// Create a minimal launcher (no splash, simple menu)
    pub fn minimal() -> Self {
        Self {
            config: LauncherConfig::minimal(),
            hooks: Arc::new(DefaultLauncherHooks),
            auto_start: true,
        }
    }
    
    /// Create a full-featured launcher
    pub fn full_featured() -> Self {
        Self {
            config: LauncherConfig::full_featured(),
            hooks: Arc::new(DefaultLauncherHooks),
            auto_start: true,
        }
    }
}

impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        // Validate and insert configuration
        let mut config = self.config.clone();
        config.validate();
        
        // Initialize the state system
        app.init_state::<AppState>();
        
        // Insert resources
        app.insert_resource(config)
            .insert_resource(StateTransitionManager::new())
            .insert_resource(GameModuleRegistry::new())
            .insert_resource(LauncherHooksResource(self.hooks.clone()));
        
        // Add core launcher systems
        app.add_systems(
            Update,
            validate_state_transitions,
        );
        app.add_systems(
            Update,
            handle_state_transition_requests,
        );
        app.add_systems(
            Update,
            update_loading_progress,
        );
        app.add_systems(
            Update,
            monitor_recovery_mode,
        );
        
        // Add event types
        app.add_message::<StateTransitionRequest>();
        
        // Add state-specific systems
        app.add_systems(OnEnter(AppState::Boot), setup_boot_state)
            .add_systems(Update, boot_state_system.run_if(in_state(AppState::Boot)))
            .add_systems(OnExit(AppState::Boot), cleanup_boot_state);
        
        app.add_systems(OnEnter(AppState::Splash), setup_splash_state)
            .add_systems(Update, splash_state_system.run_if(in_state(AppState::Splash)))
            .add_systems(OnExit(AppState::Splash), cleanup_splash_state);
        
        // Main menu systems will be added for any MainMenu variant
        app.add_systems(
            Update,
            setup_main_menu_on_enter,
        );
        app.add_systems(
            Update,
            main_menu_state_system,
        );
        app.add_systems(
            Update,
            cleanup_main_menu_on_exit,
        );
        
        // Loading systems will be added for any Loading variant
        app.add_systems(
            Update,
            setup_loading_on_enter,
        );
        app.add_systems(
            Update,
            loading_state_system,
        );
        app.add_systems(
            Update,
            cleanup_loading_on_exit,
        );
        
        // Settings state
        app.add_systems(OnEnter(AppState::Settings), setup_settings_state)
            .add_systems(Update, settings_state_system.run_if(in_state(AppState::Settings)))
            .add_systems(OnExit(AppState::Settings), cleanup_settings_state);
        
        info!("Game Launcher Plugin initialized");
    }
}

/// Resource for storing registered game modules
#[derive(Resource)]
pub struct GameModuleRegistry {
    modules: Vec<Box<dyn GameModule>>,
    active_module: Option<usize>,
}

impl GameModuleRegistry {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            active_module: None,
        }
    }
    
    /// Register a game module
    pub fn register_module(&mut self, module: Box<dyn GameModule>) {
        info!("Registering game module: {}", module.descriptor().name);
        self.modules.push(module);
    }
    
    /// Get the active game module
    pub fn active_module(&self) -> Option<&dyn GameModule> {
        self.active_module
            .and_then(|index| self.modules.get(index))
            .map(|module| module.as_ref())
    }
    
    /// Set the active game module by index
    pub fn set_active_module(&mut self, index: usize) -> bool {
        if index < self.modules.len() {
            self.active_module = Some(index);
            true
        } else {
            false
        }
    }
    
    /// Get all registered modules
    pub fn modules(&self) -> &[Box<dyn GameModule>] {
        &self.modules
    }
}

/// Resource wrapper for launcher hooks
#[derive(Resource)]
pub struct LauncherHooksResource(pub Arc<dyn LauncherHooks>);

/// Event for requesting state transitions
#[derive(Debug, Clone, Message)]
pub struct StateTransitionRequest {
    pub target_state: AppState,
    pub force: bool,
}

impl StateTransitionRequest {
    pub fn new(target_state: AppState) -> Self {
        Self {
            target_state,
            force: false,
        }
    }
    
    pub fn forced(target_state: AppState) -> Self {
        Self {
            target_state,
            force: true,
        }
    }
}

// ============================================================================
// Core Systems
// ============================================================================

/// System to validate state transitions before they occur
fn validate_state_transitions(
    current_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut transition_manager: ResMut<StateTransitionManager>,
    mut transition_requests: MessageReader<StateTransitionRequest>,
) {
    for request in transition_requests.read() {
        let current = current_state.get();
        let target = &request.target_state;
        
        // Validate the target state configuration
        if let Err(validation_error) = transition_manager.validate_state(target) {
            error!("Invalid state configuration: {} - attempting recovery", validation_error);
            let recovered_state = transition_manager.recover_invalid_state(target);
            
            if transition_manager.is_valid_transition(current, &recovered_state) {
                transition_manager.record_transition(current.clone(), recovered_state.clone());
                info!("State recovered and transitioned: {:?} -> {:?}", current, recovered_state);
                next_state.set(recovered_state);
            } else {
                // If recovery state is also invalid, trigger recovery mode
                if let Some(recovery_state) = transition_manager.record_failed_transition(current, target) {
                    error!("Triggered recovery mode, transitioning to: {:?}", recovery_state);
                    next_state.set(recovery_state);
                }
            }
            continue;
        }
        
        if request.force || transition_manager.is_valid_transition(current, target) {
            if !transition_manager.are_transitions_locked() || request.force {
                transition_manager.record_transition(current.clone(), target.clone());
                next_state.set(target.clone());
                info!("State transition: {:?} -> {:?}", current, target);
                
                // Log recovery mode status
                if transition_manager.is_in_recovery_mode() {
                    info!("Transition completed in recovery mode");
                }
            } else {
                warn!("State transition blocked (transitions locked): {:?} -> {:?}", current, target);
                // Record as failed transition if not forced
                if let Some(recovery_state) = transition_manager.record_failed_transition(current, target) {
                    error!("Triggered recovery mode due to locked transitions, transitioning to: {:?}", recovery_state);
                    next_state.set(recovery_state);
                }
            }
        } else {
            warn!("Invalid state transition: {:?} -> {:?}", current, target);
            // Record failed transition and potentially trigger recovery
            if let Some(recovery_state) = transition_manager.record_failed_transition(current, target) {
                error!("Triggered recovery mode due to invalid transition, transitioning to: {:?}", recovery_state);
                next_state.set(recovery_state);
            }
        }
    }
}

/// System to handle state transition requests
fn handle_state_transition_requests(
    mut transition_requests: MessageWriter<StateTransitionRequest>,
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<AppState>>,
) {
    // Example: Press Escape to go back to main menu from game states
    if keyboard.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            AppState::InGame(_) | AppState::Paused(_) | AppState::GameOver(_) => {
                transition_requests.write(StateTransitionRequest::new(AppState::main_menu()));
            }
            _ => {}
        }
    }
    
    // Example: Press F1 to restart (go to Boot)
    if keyboard.just_pressed(KeyCode::F1) {
        transition_requests.write(StateTransitionRequest::forced(AppState::Boot));
    }
}

/// System to update loading progress
fn update_loading_progress(
    current_state: Res<State<AppState>>,
    time: Res<Time>,
    // TODO: Add asset loading progress tracking
) {
    if let AppState::Loading(_config) = current_state.get() {
        // TODO: Implement actual loading progress tracking
        // This would integrate with Bevy's asset loading system
        let _delta = time.delta();
        // Update loading progress based on actual asset loading
    }
}

// ============================================================================
// Boot State Systems
// ============================================================================

fn setup_boot_state(
    commands: Commands,
    hooks: Res<LauncherHooksResource>,
    mut transition_manager: ResMut<StateTransitionManager>,
) {
    info!("Entering Boot state");
    
    // Clear any previous state history
    transition_manager.clear_history();
    
    // Call hook
    hooks.0.on_boot_enter();
    
    // TODO: Add boot-specific setup (system checks, config loading, etc.)
    let _ = commands;
}

fn boot_state_system(
    mut transition_requests: MessageWriter<StateTransitionRequest>,
    config: Res<LauncherConfig>,
    // TODO: Add system for checking boot completion
) {
    // For now, immediately transition to next state
    // In a real implementation, this would wait for system initialization
    
    if config.splash.enabled {
        transition_requests.write(StateTransitionRequest::new(AppState::Splash));
    } else {
        transition_requests.write(StateTransitionRequest::new(AppState::main_menu()));
    }
}

fn cleanup_boot_state(
    commands: Commands,
    hooks: Res<LauncherHooksResource>,
) {
    info!("Exiting Boot state");
    hooks.0.on_boot_exit();
    let _ = commands;
}

// ============================================================================
// Splash State Systems
// ============================================================================

#[derive(Component)]
struct SplashTimer(Timer);

fn setup_splash_state(
    mut commands: Commands,
    hooks: Res<LauncherHooksResource>,
    config: Res<LauncherConfig>,
) {
    info!("Entering Splash state");
    
    // Create splash timer
    let duration = config.splash.total_duration();
    commands.spawn(SplashTimer(Timer::new(duration, TimerMode::Once)));
    
    // Call hook
    hooks.0.on_splash_enter();
    
    // TODO: Add splash screen UI setup
}

fn splash_state_system(
    mut transition_requests: MessageWriter<StateTransitionRequest>,
    mut splash_timers: Query<&mut SplashTimer>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    config: Res<LauncherConfig>,
    hooks: Res<LauncherHooksResource>,
    commands: Commands,
) {
    // Check for skip input
    if config.splash.allow_skip && (keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Enter)) {
        transition_requests.write(StateTransitionRequest::new(AppState::main_menu()));
        return;
    }
    
    // Update timer
    for mut timer in splash_timers.iter_mut() {
        timer.0.tick(time.delta());
        
        if timer.0.just_finished() {
            hooks.0.on_splash_complete();
            transition_requests.write(StateTransitionRequest::new(AppState::main_menu()));
        }
    }
    
    let _ = commands;
}

fn cleanup_splash_state(
    mut commands: Commands,
    splash_timers: Query<Entity, With<SplashTimer>>,
) {
    info!("Exiting Splash state");
    
    // Clean up splash timer
    for entity in splash_timers.iter() {
        commands.entity(entity).despawn();
    }
}

// ============================================================================
// Main Menu State Systems
// ============================================================================

fn setup_main_menu_on_enter(
    current_state: Res<State<AppState>>,
    commands: Commands,
    hooks: Res<LauncherHooksResource>,
) {
    if let AppState::MainMenu(_config) = current_state.get() {
        if current_state.is_changed() {
            info!("Entering MainMenu state");
            hooks.0.on_main_menu_enter();
            // TODO: Setup main menu UI
            let _ = commands;
        }
    }
}

fn main_menu_state_system(
    current_state: Res<State<AppState>>,
    mut transition_requests: MessageWriter<StateTransitionRequest>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if let AppState::MainMenu(_config) = current_state.get() {
        // Example menu navigation
        if keyboard.just_pressed(KeyCode::Enter) {
            // Start new game
            let game_config = super::states::GameConfig::default();
            let loading_config = super::states::LoadingConfig::new(AppState::in_game_with_config(game_config));
            transition_requests.write(StateTransitionRequest::new(AppState::loading_with_config(loading_config)));
        }
        
        if keyboard.just_pressed(KeyCode::KeyS) {
            // Go to settings
            transition_requests.write(StateTransitionRequest::new(AppState::Settings));
        }
    }
}

fn cleanup_main_menu_on_exit(
    current_state: Res<State<AppState>>,
    commands: Commands,
) {
    // Only run cleanup when actually exiting MainMenu state
    if !matches!(current_state.get(), AppState::MainMenu(_)) && current_state.is_changed() {
        info!("Exiting MainMenu state");
        // TODO: Cleanup main menu UI
        let _ = commands;
    }
}

// ============================================================================
// Loading State Systems
// ============================================================================

fn setup_loading_on_enter(
    current_state: Res<State<AppState>>,
    commands: Commands,
    hooks: Res<LauncherHooksResource>,
) {
    if let AppState::Loading(_config) = current_state.get() {
        if current_state.is_changed() {
            info!("Entering Loading state");
            hooks.0.on_loading_enter();
            // TODO: Setup loading UI and start asset loading
            let _ = commands;
        }
    }
}

fn loading_state_system(
    current_state: Res<State<AppState>>,
    mut transition_requests: MessageWriter<StateTransitionRequest>,
    time: Res<Time>,
    hooks: Res<LauncherHooksResource>,
    commands: Commands,
) {
    if let AppState::Loading(config) = current_state.get() {
        // TODO: Implement actual loading progress tracking
        // For now, simulate loading completion after minimum duration
        
        // Simulate loading completion
        let progress = 1.0; // This would come from actual asset loading
        hooks.0.on_loading_progress(progress);
        
        if progress >= 1.0 {
            hooks.0.on_loading_complete();
            transition_requests.write(StateTransitionRequest::new(config.target_state()));
        }
        
        let _ = (time, commands); // Suppress unused warning
    }
}

fn cleanup_loading_on_exit(
    current_state: Res<State<AppState>>,
    commands: Commands,
) {
    if !matches!(current_state.get(), AppState::Loading(_)) && current_state.is_changed() {
        info!("Exiting Loading state");
        // TODO: Cleanup loading UI
        let _ = commands;
    }
}

// ============================================================================
// Settings State Systems
// ============================================================================

fn setup_settings_state(
    commands: Commands,
) {
    info!("Entering Settings state");
    // TODO: Setup settings UI
    let _ = commands;
}

fn settings_state_system(
    mut transition_requests: MessageWriter<StateTransitionRequest>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // Example: Press Escape to return to main menu
    if keyboard.just_pressed(KeyCode::Escape) {
        transition_requests.write(StateTransitionRequest::new(AppState::main_menu()));
    }
}

fn cleanup_settings_state(
    commands: Commands,
) {
    info!("Exiting Settings state");
    // TODO: Cleanup settings UI
    let _ = commands;
}

// ============================================================================
// Recovery and Monitoring Systems
// ============================================================================

/// System to monitor recovery mode and provide diagnostics
fn monitor_recovery_mode(
    transition_manager: Res<StateTransitionManager>,
    current_state: Res<State<AppState>>,
) {
    // Only log recovery mode status changes
    if transition_manager.is_changed() && transition_manager.is_in_recovery_mode() {
        let failed_count = transition_manager.get_failed_transition_count();
        let recovery_state = transition_manager.get_recovery_state();
        let last_good_state = transition_manager.get_last_known_good_state();
        
        info!(
            "Recovery mode active - Current: {:?}, Failed transitions: {}, Recovery target: {:?}, Last good: {:?}",
            current_state.get(),
            failed_count,
            recovery_state,
            last_good_state
        );
    }
}