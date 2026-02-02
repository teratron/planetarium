pub mod config;
pub mod plugin;
pub mod states;
pub mod traits;

pub use config::{LauncherConfig, SplashConfig, IntegrationConfig};
pub use plugin::LauncherPlugin;
pub use states::{AppState, GameConfig, GameDifficulty, MenuOption};
pub use traits::{
    GameModule, GameModuleDescriptor, GameType, LauncherHooks, StateTransitionManager,
};

/// Re-export commonly used types for convenience
pub mod prelude {
    pub use super::{
        AppState, GameModule, GameModuleDescriptor, GameType, LauncherConfig, LauncherPlugin,
        StateTransitionManager,
    };
}