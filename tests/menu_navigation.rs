use bevy::prelude::*;
use fluent_bundle::bundle::FluentBundle;
use intl_memoizer::concurrent::IntlLangMemoizer;
use planetarium::framework::localization::Localization;
use planetarium::framework::menu::main::systems::handle_menu_button_clicks;
use planetarium::framework::settings::SettingsOpen;
use planetarium::framework::states::AppState;
use planetarium::framework::ui::fading::{FadeState, FadingPlugin, ScreenFade};
use planetarium::framework::ui::modal::ModalState;
use planetarium::framework::ui::widgets::{ButtonAction, PrimaryButton};
use std::str::FromStr;
use unic_langid::LanguageIdentifier;

fn create_dummy_localization() -> Localization {
    let lang = LanguageIdentifier::from_str("en-US").unwrap();

    // Create main bundle using new_concurrent
    let main_bundle =
        FluentBundle::<fluent_bundle::FluentResource, IntlLangMemoizer>::new_concurrent(vec![
            lang.clone(),
        ]);

    // Create fallback bundle using new_concurrent
    let fallback_bundle =
        FluentBundle::<fluent_bundle::FluentResource, IntlLangMemoizer>::new_concurrent(vec![
            lang.clone(),
        ]);

    // The Localization::new signature: locale, main_bundle, fallback_bundle, assets_dir
    Localization::new(
        lang,
        main_bundle,
        fallback_bundle,
        std::path::PathBuf::new(),
    )
}

#[test]
fn play_button_triggers_loading_transition() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::state::app::StatesPlugin));
    app.init_state::<AppState>();
    app.add_plugins(FadingPlugin);
    app.init_resource::<SettingsOpen>();
    app.init_resource::<ModalState>();
    app.insert_resource(create_dummy_localization());
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
    app.init_resource::<ModalState>();
    app.insert_resource(create_dummy_localization());
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
