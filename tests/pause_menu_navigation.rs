use bevy::prelude::*;
use planetarium::framework::assets::{AssetCache, AssetManifest};
use planetarium::framework::menu::pause::components::{PauseMenuButton, PauseMenuButtonAction};
use planetarium::framework::menu::pause::input::handle_escape_input;
use planetarium::framework::menu::pause::state::{
    PauseMenuActionEvent, PauseMenuMode, PauseMenuState,
};
use planetarium::framework::menu::pause::systems::{
    apply_pause_menu_actions, handle_pause_menu_button_clicks,
};
use planetarium::framework::settings::RuntimeAudioState;
use planetarium::framework::settings::SettingsOpen;
use planetarium::framework::states::AppState;

fn setup_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::state::app::StatesPlugin));
    app.insert_state(AppState::InGame);
    app.insert_resource(ButtonInput::<KeyCode>::default());
    app.init_resource::<SettingsOpen>();
    app.init_resource::<PauseMenuState>();
    app.insert_resource(AssetManifest::default());
    app.insert_resource(AssetCache::default());
    app.insert_resource(RuntimeAudioState::default());
    app.add_message::<PauseMenuActionEvent>();
    app.add_systems(
        Update,
        (
            handle_escape_input,
            handle_pause_menu_button_clicks,
            apply_pause_menu_actions,
        )
            .chain(),
    );

    app
}

#[test]
fn esc_opens_pause_menu() {
    let mut app = setup_app();
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::Escape);

    app.update();

    let pause = app.world().resource::<PauseMenuState>();
    assert_eq!(pause.mode, PauseMenuMode::Menu);
}

#[test]
fn esc_in_settings_returns_to_pause_menu() {
    let mut app = setup_app();
    app.world_mut().resource_mut::<PauseMenuState>().mode = PauseMenuMode::Settings;
    app.world_mut().resource_mut::<SettingsOpen>().0 = true;
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::Escape);

    app.update();

    let pause = app.world().resource::<PauseMenuState>();
    let settings_open = app.world().resource::<SettingsOpen>();
    assert_eq!(pause.mode, PauseMenuMode::Menu);
    assert!(!settings_open.0);
}

#[test]
fn pause_button_can_transition_to_main_menu() {
    let mut app = setup_app();
    app.world_mut().resource_mut::<PauseMenuState>().mode = PauseMenuMode::Menu;

    app.world_mut().spawn((
        Button,
        Interaction::Pressed,
        PauseMenuButton {
            action: PauseMenuButtonAction::ExitToMainMenu,
        },
    ));

    app.update();
    app.update();

    let state = app.world().resource::<State<AppState>>();
    assert_eq!(*state.get(), AppState::MainMenu);

    let pause = app.world().resource::<PauseMenuState>();
    assert_eq!(pause.mode, PauseMenuMode::Closed);
}
