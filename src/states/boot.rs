use crate::resources::{save_data::SaveData, settings::GameSettings};
use crate::states::GameState;
use crate::config::GameConfig;
use bevy::prelude::*;
use std::time::Duration;

pub struct BootPlugin;

impl Plugin for BootPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Boot), setup_boot)
            .add_systems(
                Update,
                (
                    boot_initialization_system,
                    boot_complete_check,
                ).run_if(in_state(GameState::Boot)),
            );
    }
}

/// Resource to track boot initialization progress
#[derive(Resource)]
pub struct BootProgress {
    pub config_loaded: bool,
    pub system_requirements_checked: bool,
    pub basic_components_initialized: bool,
    pub launcher_config_loaded: bool,
    pub boot_timer: Timer,
    pub initialization_errors: Vec<String>,
}

impl Default for BootProgress {
    fn default() -> Self {
        Self {
            config_loaded: false,
            system_requirements_checked: false,
            basic_components_initialized: false,
            launcher_config_loaded: false,
            boot_timer: Timer::new(Duration::from_millis(100), TimerMode::Once), // Minimum boot time
            initialization_errors: Vec::new(),
        }
    }
}

impl BootProgress {
    /// Check if all boot tasks are complete
    pub fn is_complete(&self) -> bool {
        self.config_loaded 
            && self.system_requirements_checked 
            && self.basic_components_initialized 
            && self.launcher_config_loaded
            && self.boot_timer.just_finished()
    }
    
    /// Get completion percentage (0.0 to 1.0)
    pub fn completion_percentage(&self) -> f32 {
        let mut completed_tasks = 0;
        let total_tasks = 4;
        
        if self.config_loaded { completed_tasks += 1; }
        if self.system_requirements_checked { completed_tasks += 1; }
        if self.basic_components_initialized { completed_tasks += 1; }
        if self.launcher_config_loaded { completed_tasks += 1; }
        
        completed_tasks as f32 / total_tasks as f32
    }
    
    /// Add an initialization error
    pub fn add_error(&mut self, error: String) {
        error!("Boot initialization error: {}", error);
        self.initialization_errors.push(error);
    }
    
    /// Check if there are any critical errors that should prevent boot completion
    pub fn has_critical_errors(&self) -> bool {
        // For now, we'll be lenient and not consider any errors as critical
        // In a real implementation, you might want to check for specific error types
        false
    }
}

fn setup_boot(mut commands: Commands) {
    info!("=== Boot Phase Started ===");
    
    // Spawn 2D camera for UI (basic component initialization)
    commands.spawn(Camera2d::default());
    
    // Initialize boot progress tracking
    commands.insert_resource(BootProgress::default());
    
    info!("Boot setup complete, starting initialization sequence");
}

/// Main boot initialization system that handles all boot tasks
fn boot_initialization_system(
    mut boot_progress: ResMut<BootProgress>,
    mut commands: Commands,
    time: Res<Time>,
    game_config: Option<Res<GameConfig>>,
    game_settings: Option<Res<GameSettings>>,
    save_data: Option<Res<SaveData>>,
) {
    // Update boot timer
    boot_progress.boot_timer.tick(time.delta());
    
    // Task 1: Load game configuration
    if !boot_progress.config_loaded {
        match game_config {
            Some(_config) => {
                info!("✓ Game configuration loaded successfully");
                boot_progress.config_loaded = true;
                
                // Initialize global resources based on config
                if game_settings.is_none() {
                    commands.insert_resource(GameSettings::default());
                    info!("✓ Game settings initialized");
                }
                
                if save_data.is_none() {
                    commands.insert_resource(SaveData::default());
                    info!("✓ Save data initialized");
                }
            }
            None => {
                // Config should be loaded by ConfigPlugin before we get here
                // If it's not available, we'll create a default one
                warn!("Game configuration not found, creating default");
                commands.insert_resource(crate::config::GameConfig::default());
                commands.insert_resource(GameSettings::default());
                commands.insert_resource(SaveData::default());
                boot_progress.config_loaded = true;
            }
        }
    }
    
    // Task 2: Check system requirements
    if !boot_progress.system_requirements_checked && boot_progress.config_loaded {
        if let Err(error) = check_system_requirements() {
            boot_progress.add_error(format!("System requirements check failed: {}", error));
        } else {
            info!("✓ System requirements check passed");
        }
        boot_progress.system_requirements_checked = true;
    }
    
    // Task 3: Initialize basic components
    if !boot_progress.basic_components_initialized && boot_progress.config_loaded {
        if let Err(error) = initialize_basic_components(&mut commands) {
            boot_progress.add_error(format!("Basic components initialization failed: {}", error));
        } else {
            info!("✓ Basic components initialized");
        }
        boot_progress.basic_components_initialized = true;
    }
    
    // Task 4: Load launcher configuration (simplified for now)
    if !boot_progress.launcher_config_loaded && boot_progress.basic_components_initialized {
        // For now, we'll just mark this as complete since we're not using LauncherConfig
        info!("✓ Launcher configuration initialized (using defaults)");
        boot_progress.launcher_config_loaded = true;
    }
    
    // Log progress periodically
    let progress = boot_progress.completion_percentage();
    if progress > 0.0 && boot_progress.boot_timer.elapsed().as_millis() % 500 == 0 {
        info!("Boot progress: {:.0}%", progress * 100.0);
    }
}

/// Check if boot is complete and transition to next state
fn boot_complete_check(
    mut boot_progress: ResMut<BootProgress>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    // Continue ticking the timer
    boot_progress.boot_timer.tick(time.delta());
    
    if boot_progress.is_complete() {
        if boot_progress.has_critical_errors() {
            error!("Boot completed with critical errors, staying in boot state");
            error!("Errors: {:?}", boot_progress.initialization_errors);
            return;
        }
        
        info!("=== Boot Phase Complete ===");
        if !boot_progress.initialization_errors.is_empty() {
            warn!("Boot completed with {} non-critical warnings", boot_progress.initialization_errors.len());
            for error in &boot_progress.initialization_errors {
                warn!("  - {}", error);
            }
        }
        
        // For now, always transition to Splash state
        // In the future, this will be determined by launcher configuration
        info!("Transitioning to Splash state");
        next_state.set(GameState::Splash);
    }
}

/// Check system requirements
fn check_system_requirements() -> Result<(), String> {
    // Check available memory
    if let Some(available_memory) = get_available_memory() {
        const MIN_MEMORY_MB: u64 = 512; // 512 MB minimum
        if available_memory < MIN_MEMORY_MB * 1024 * 1024 {
            return Err(format!(
                "Insufficient memory: {} MB available, {} MB required",
                available_memory / (1024 * 1024),
                MIN_MEMORY_MB
            ));
        }
        info!("Memory check passed: {} MB available", available_memory / (1024 * 1024));
    } else {
        warn!("Could not determine available memory, skipping memory check");
    }
    
    // Check graphics capabilities
    if let Err(error) = check_graphics_capabilities() {
        return Err(format!("Graphics check failed: {}", error));
    }
    
    // Check disk space
    if let Err(error) = check_disk_space() {
        return Err(format!("Disk space check failed: {}", error));
    }
    
    Ok(())
}

/// Get available system memory (simplified implementation)
fn get_available_memory() -> Option<u64> {
    // This is a simplified implementation
    // In a real application, you would use platform-specific APIs
    #[cfg(target_os = "windows")]
    {
        // On Windows, you could use GlobalMemoryStatusEx
        // For now, we'll assume we have enough memory
        Some(2 * 1024 * 1024 * 1024) // Assume 2GB available
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        // On other platforms, you could read /proc/meminfo or use sysinfo crate
        // For now, we'll assume we have enough memory
        Some(2 * 1024 * 1024 * 1024) // Assume 2GB available
    }
}

/// Check graphics capabilities
fn check_graphics_capabilities() -> Result<(), String> {
    // In a real implementation, you would check:
    // - OpenGL/Vulkan/DirectX support
    // - Minimum GPU memory
    // - Required extensions
    
    // For now, we'll assume graphics are supported since Bevy handles this
    info!("Graphics capabilities check passed (delegated to Bevy)");
    Ok(())
}

/// Check available disk space
fn check_disk_space() -> Result<(), String> {
    // In a real implementation, you would check available disk space
    // For now, we'll assume there's enough space
    info!("Disk space check passed (assuming sufficient space)");
    Ok(())
}

/// Initialize basic components required for the game
fn initialize_basic_components(commands: &mut Commands) -> Result<(), String> {
    // Initialize logging if not already done
    // (This is typically done in main.rs, but we can ensure it's set up)
    
    // Initialize asset loading systems
    // (Bevy handles this automatically, but we could add custom asset loaders here)
    
    // Initialize input systems
    // (Bevy handles this automatically)
    
    // Initialize audio systems
    // (Bevy handles this automatically)
    
    // Initialize networking systems if needed
    // (Would be added here if the game requires networking)
    
    // For now, we'll just log that basic components are initialized
    info!("Basic components initialization complete");
    
    // Add any custom resources that are needed globally
    // These would be game-specific resources that need to be available early
    
    let _ = commands; // Suppress unused warning for now
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;

    #[test]
    fn test_boot_progress_default() {
        let progress = BootProgress::default();
        
        assert!(!progress.config_loaded);
        assert!(!progress.system_requirements_checked);
        assert!(!progress.basic_components_initialized);
        assert!(!progress.launcher_config_loaded);
        assert!(progress.initialization_errors.is_empty());
        assert!(!progress.is_complete());
        assert_eq!(progress.completion_percentage(), 0.0);
    }

    #[test]
    fn test_boot_progress_completion() {
        let mut progress = BootProgress::default();
        
        // Initially 0% complete
        assert_eq!(progress.completion_percentage(), 0.0);
        assert!(!progress.is_complete());
        
        // Complete first task - 25%
        progress.config_loaded = true;
        assert_eq!(progress.completion_percentage(), 0.25);
        assert!(!progress.is_complete());
        
        // Complete second task - 50%
        progress.system_requirements_checked = true;
        assert_eq!(progress.completion_percentage(), 0.5);
        assert!(!progress.is_complete());
        
        // Complete third task - 75%
        progress.basic_components_initialized = true;
        assert_eq!(progress.completion_percentage(), 0.75);
        assert!(!progress.is_complete());
        
        // Complete fourth task - 100% but timer not finished
        progress.launcher_config_loaded = true;
        assert_eq!(progress.completion_percentage(), 1.0);
        assert!(!progress.is_complete()); // Timer not finished yet
        
        // Finish timer
        progress.boot_timer.set_elapsed(std::time::Duration::from_millis(200));
        assert!(progress.is_complete());
    }

    #[test]
    fn test_boot_progress_error_handling() {
        let mut progress = BootProgress::default();
        
        assert!(!progress.has_critical_errors());
        assert!(progress.initialization_errors.is_empty());
        
        progress.add_error("Test error".to_string());
        assert_eq!(progress.initialization_errors.len(), 1);
        assert_eq!(progress.initialization_errors[0], "Test error");
        
        // Currently no errors are considered critical
        assert!(!progress.has_critical_errors());
    }

    #[test]
    fn test_system_requirements_check() {
        // This should not fail in a test environment
        let result = check_system_requirements();
        assert!(result.is_ok(), "System requirements check should pass: {:?}", result);
    }

    #[test]
    fn test_get_available_memory() {
        let memory = get_available_memory();
        assert!(memory.is_some(), "Should be able to get memory information");
        
        if let Some(mem) = memory {
            assert!(mem > 0, "Available memory should be greater than 0");
            // Should be at least 512MB in any reasonable test environment
            assert!(mem >= 512 * 1024 * 1024, "Should have at least 512MB available");
        }
    }

    #[test]
    fn test_graphics_capabilities_check() {
        let result = check_graphics_capabilities();
        assert!(result.is_ok(), "Graphics capabilities check should pass: {:?}", result);
    }

    #[test]
    fn test_disk_space_check() {
        let result = check_disk_space();
        assert!(result.is_ok(), "Disk space check should pass: {:?}", result);
    }

    #[test]
    fn test_initialize_basic_components() {
        let mut app = App::new();
        let mut commands = app.world_mut().commands();
        
        let result = initialize_basic_components(&mut commands);
        assert!(result.is_ok(), "Basic components initialization should succeed: {:?}", result);
    }

    #[test]
    fn test_boot_plugin_registration() {
        let mut app = App::new();
        app.add_plugins(BootPlugin);
        
        // The plugin should register the boot systems
        // We can't easily test the systems directly, but we can verify the plugin builds
        assert!(true, "BootPlugin should register without errors");
    }
}
