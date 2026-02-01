use bevy::prelude::*;

pub mod boot;
pub mod gameplay;
pub mod loading;
pub mod main_menu;
pub mod settings;
pub mod splash;
pub mod paused;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Boot,
    Splash,
    MainMenu,
    Settings,
    Loading,
    Gameplay,
    Paused,
    Credits,
}

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            boot::BootPlugin,
            splash::SplashPlugin,
            main_menu::MainMenuPlugin,
            settings::SettingsPlugin,
            loading::LoadingPlugin,
            gameplay::GameplayPlugin,
            paused::PausedPlugin,
        ));
    }
}
