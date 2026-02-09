//! Stage 3: Main Menu system.
//! Handles UI layout, interaction, and settings.

use crate::core::states::AppState;
use bevy::prelude::*;

pub mod layout;
pub mod reactive;
pub mod screen;
pub mod settings;
pub mod widgets;

use reactive::{
    RuntimeAudioState, SettingsAutoSaveTimer, auto_save_settings, broadcast_settings_changes,
    broadcast_theme_changes, schedule_settings_save,
};
use screen::{despawn_main_menu, handle_menu_button_clicks, spawn_main_menu};
use settings::{
    ActiveSettingsTab, SettingsOpen, handle_settings_tab_clicks, spawn_settings_if_needed,
    update_settings_tab_content, update_settings_ui,
};
use widgets::{animate_button_hover, button_interaction_system};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        info!("[MenuPlugin] Initializing...");
        // Initialize settings resources
        app.init_resource::<SettingsOpen>();
        app.init_resource::<ActiveSettingsTab>();

        // Setup systems for MainMenu state
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu);

        // Update systems while in MainMenu
        app.add_systems(
            Update,
            (button_interaction_system, animate_button_hover).run_if(in_state(AppState::MainMenu)),
        );

        app.add_systems(
            Update,
            (
                widgets::slider_interaction_system,
                widgets::dropdown_interaction_system,
                widgets::dropdown_option_interaction_system,
                widgets::update_slider_visuals,
            )
                .run_if(in_state(AppState::MainMenu)),
        );

        app.add_systems(
            Update,
            handle_menu_button_clicks.run_if(in_state(AppState::MainMenu)),
        );

        // Settings systems
        app.add_systems(
            Update,
            (
                spawn_settings_if_needed,
                handle_settings_tab_clicks,
                update_settings_tab_content,
                update_settings_ui,
            )
                .chain()
                .run_if(in_state(AppState::MainMenu)),
        );

        // Reactive settings: runtime audio state + apply-on-change system
        app.init_resource::<RuntimeAudioState>();
        app.init_resource::<SettingsAutoSaveTimer>();
        app.add_systems(
            Update,
            (
                broadcast_settings_changes,
                broadcast_theme_changes,
                schedule_settings_save,
                auto_save_settings,
                crate::core::localization::apply_language_change_system,
            ),
        );

        // Cleanup on exit
        app.add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}
