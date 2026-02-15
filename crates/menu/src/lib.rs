use bevy::prelude::*;

pub mod modal;
// Redundant modal code removed. Using modal module instead.

pub mod events;
pub mod main_menu;
// pub mod modal; // Removed duplicate
pub mod pause;
pub mod settings;
pub mod widgets;

use transitions::FadingPlugin;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<events::UiAudioEvent>()
            .add_message::<settings::SettingsSaveError>()
            .add_plugins((
                FadingPlugin::<main_menu::MainState>::default(),
                FadingPlugin::<pause::PauseState>::default(),
                FadingPlugin::<settings::SettingsTab>::default(),
                main_menu::MainPlugin,
                pause::PausePlugin,
                settings::SettingsPlugin,
                modal::ModalPlugin,
            ))
            .init_resource::<settings::SettingsOpen>()
            .init_resource::<settings::ActiveSettingsTab>()
            .init_resource::<settings::SettingsChangeTracker>()
            .init_resource::<settings::SettingsAutoSaveTimer>()
            .init_resource::<settings::PendingSettings>()
            .add_message::<settings::SettingsSaveError>()
            .init_resource::<settings::RuntimeAudioState>()
            .add_systems(
                Update,
                (
                    settings::spawn_settings_if_needed,
                    settings::animate_settings_fade,
                    settings::update_settings_ui,
                    settings::handle_settings_tab_clicks,
                    settings::update_settings_tab_content,
                    settings::broadcast_settings_changes,
                    settings::broadcast_theme_changes,
                )
                    .into_configs()
                    .run_if(settings_active_or_background),
            )
            .add_systems(
                Update,
                settings::schedule_settings_save.run_if(settings_active_or_background),
            )
            .add_systems(
                Update,
                settings::auto_save_settings.run_if(settings_active_or_background),
            )
            .add_systems(
                Update,
                settings::pending_systems::handle_settings_action_buttons
                    .run_if(settings_active_or_background),
            )
            .add_systems(
                Update,
                settings::pending_systems::initialize_pending_settings
                    .run_if(settings_active_or_background),
            )
            .add_systems(
                Update,
                (
                    widgets::button_interaction_system,
                    widgets::slider_interaction_system,
                    widgets::update_slider_visuals,
                    widgets::dropdown_interaction_system,
                    widgets::dropdown_option_interaction_system,
                )
                    .into_configs(),
            )
            .add_systems(
                Update,
                (
                    main_menu::spawn_main_menu,
                    main_menu::handle_menu_button_clicks,
                )
                    .into_configs()
                    .run_if(in_state(launcher::states::AppState::MainMenu)),
            )
            .add_systems(
                OnExit(launcher::states::AppState::MainMenu),
                main_menu::despawn_main_menu,
            );
    }
}

fn settings_active_or_background(
    settings_open: Res<settings::SettingsOpen>,
    auto_save: Res<settings::SettingsAutoSaveTimer>,
) -> bool {
    settings_open.0 || settings::settings_auto_save_active(auto_save)
}
