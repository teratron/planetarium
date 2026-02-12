//! In-game pause menu plugin.

use crate::framework::states::AppState;
use bevy::prelude::*;

pub mod components;
pub mod input;
pub mod settings_bridge;
pub mod state;
pub mod systems;
pub mod ui;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<state::PauseMenuState>()
            .add_message::<state::PauseMenuActionEvent>()
            .add_systems(
                Update,
                input::handle_escape_input.run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                (
                    systems::handle_pause_menu_button_clicks,
                    systems::handle_pause_settings_back_button,
                    systems::apply_pause_menu_actions,
                    ui::sync_pause_menu_ui,
                )
                    .chain()
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                (
                    settings_bridge::spawn_settings_if_needed_bridge,
                    settings_bridge::handle_settings_tab_clicks_bridge,
                    settings_bridge::update_settings_tab_content_bridge,
                    settings_bridge::animate_settings_fade_bridge,
                )
                    .chain()
                    .run_if(in_state(AppState::InGame))
                    .run_if(state::pause_settings_active),
            )
            .add_systems(
                Update,
                settings_bridge::update_settings_ui_bridge
                    .run_if(resource_changed::<crate::config::UserSettings>)
                    .run_if(in_state(AppState::InGame))
                    .run_if(state::pause_settings_active),
            )
            .add_systems(
                Update,
                (
                    super::widgets::slider_interaction_system,
                    super::widgets::dropdown_interaction_system,
                    super::widgets::dropdown_option_interaction_system,
                    super::widgets::update_slider_visuals,
                )
                    .run_if(in_state(AppState::InGame))
                    .run_if(state::pause_settings_active),
            )
            .add_systems(
                OnExit(AppState::InGame),
                (ui::cleanup_pause_menu, systems::reset_pause_state_on_exit),
            );
    }
}
