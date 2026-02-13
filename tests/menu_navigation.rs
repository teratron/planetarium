use bevy::prelude::*;
use planetarium::framework::menu::screen::handle_menu_button_clicks;
use planetarium::framework::menu::settings::SettingsOpen;
use planetarium::framework::menu::widgets::{ButtonAction, PrimaryButton};
use planetarium::framework::states::AppState;
use planetarium::framework::ui::fading::{FadeState, FadingPlugin, ScreenFade};

#[test]
fn play_button_triggers_loading_transition() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::state::app::StatesPlugin));
    app.init_state::<AppState>();
    app.add_plugins(FadingPlugin);
    app.init_resource::<SettingsOpen>();
    app.add_systems(Update, handle_menu_button_clicks);

    app.world_mut().spawn((
        Button,
        Interaction::Pressed,
        PrimaryButton {
            label: "Play".to_string(),
            action: ButtonAction::Play,
        },
    ));

    app.update();

    {
        let fade = app.world().resource::<ScreenFade>();
        assert_eq!(fade.state, FadeState::FadingOut);
        assert_eq!(fade.next_app_state, Some(AppState::Loading));
    }
}

#[test]
fn settings_button_opens_settings_panel() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::state::app::StatesPlugin));
    app.init_state::<AppState>();
    app.insert_resource(ScreenFade::default());
    app.init_resource::<SettingsOpen>();
    app.add_systems(Update, handle_menu_button_clicks);

    app.world_mut().spawn((
        Button,
        Interaction::Pressed,
        PrimaryButton {
            label: "Settings".to_string(),
            action: ButtonAction::Settings,
        },
    ));

    app.update();

    let settings_open = app.world().resource::<SettingsOpen>();
    assert!(settings_open.0);
}
