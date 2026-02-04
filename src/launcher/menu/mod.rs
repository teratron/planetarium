//! Stage 3: Main Menu system.
//! Handles UI layout, interaction, and settings.

use bevy::prelude::*;
use crate::core::states::AppState;

pub mod widgets;
pub mod screen;
pub mod settings;
pub mod reactive;

use widgets::button_interaction_system;
use screen::{spawn_main_menu, handle_menu_button_clicks, despawn_main_menu};
use settings::{SettingsOpen, spawn_settings_if_needed, update_settings_ui};
use reactive::{RuntimeAudioState, broadcast_settings_changes};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        info!("[MenuPlugin] Initializing...");
        // Initialize settings visibility resource
        app.init_resource::<SettingsOpen>();

        // Setup systems for MainMenu state
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu);

        // Update systems while in MainMenu
        app.add_systems(
            Update,
            button_interaction_system.run_if(in_state(AppState::MainMenu)),
        );

        app.add_systems(
            Update,
            handle_menu_button_clicks.run_if(in_state(AppState::MainMenu)),
        );

        // Settings spawn/despawn watcher
        app.add_systems(Update, spawn_settings_if_needed.run_if(in_state(AppState::MainMenu)));

        // Update settings UI display values
        app.add_systems(Update, update_settings_ui.run_if(in_state(AppState::MainMenu)));

        // Reactive settings: runtime audio state + apply-on-change system
        app.init_resource::<RuntimeAudioState>();
        app.add_systems(Update, broadcast_settings_changes);

        // Cleanup on exit
        app.add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}

