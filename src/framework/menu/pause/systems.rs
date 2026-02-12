//! Pause menu logic and action handling systems.

use super::super::reactive::RuntimeAudioState;
use super::super::settings::SettingsOpen;
use super::super::widgets::{ButtonAction, PrimaryButton};
use super::components::{PauseMenuButton, PauseMenuButtonAction};
use super::state::{PauseMenuActionEvent, PauseMenuMode, PauseMenuState};
use crate::core::assets::{AssetCache, AssetManifest};
use crate::core::states::AppState;
use bevy::app::AppExit;
use bevy::prelude::*;

/// Handles pause-menu button interactions and emits high-level actions.
#[allow(clippy::type_complexity)]
pub fn handle_pause_menu_button_clicks(
    mut commands: Commands,
    asset_server: Option<Res<AssetServer>>,
    manifest: Option<Res<AssetManifest>>,
    mut cache: Option<ResMut<AssetCache>>,
    audio_state: Option<Res<RuntimeAudioState>>,
    interactions: Query<(&Interaction, &PauseMenuButton), (Changed<Interaction>, With<Button>)>,
    mut events: MessageWriter<PauseMenuActionEvent>,
) {
    for (interaction, button) in &interactions {
        match *interaction {
            Interaction::Hovered => {
                let volume = audio_state.as_ref().map_or(0.5, |state| state.sfx * 0.5);
                try_play_ui_audio(
                    &mut commands,
                    asset_server.as_deref(),
                    manifest.as_deref(),
                    cache.as_deref_mut(),
                    "hover",
                    volume,
                );
            }
            Interaction::Pressed => {
                let volume = audio_state.as_ref().map_or(1.0, |state| state.sfx);
                try_play_ui_audio(
                    &mut commands,
                    asset_server.as_deref(),
                    manifest.as_deref(),
                    cache.as_deref_mut(),
                    "click",
                    volume,
                );

                let event = match button.action {
                    PauseMenuButtonAction::Resume => PauseMenuActionEvent::Resume,
                    PauseMenuButtonAction::OpenSettings => PauseMenuActionEvent::OpenSettings,
                    PauseMenuButtonAction::ExitToMainMenu => PauseMenuActionEvent::ExitToMainMenu,
                    PauseMenuButtonAction::ExitGame => PauseMenuActionEvent::ExitGame,
                };
                events.write(event);
            }
            Interaction::None => {}
        }
    }
}

/// Handles "Back" in settings to return to the pause menu.
#[allow(clippy::type_complexity)]
pub fn handle_pause_settings_back_button(
    interactions: Query<(&Interaction, &PrimaryButton), (Changed<Interaction>, With<Button>)>,
    pause_state: Res<PauseMenuState>,
    mut events: MessageWriter<PauseMenuActionEvent>,
) {
    if pause_state.mode != PauseMenuMode::Settings {
        return;
    }

    for (interaction, button) in &interactions {
        if *interaction == Interaction::Pressed && button.action == ButtonAction::Back {
            events.write(PauseMenuActionEvent::OpenMenu);
        }
    }
}

/// Applies pause-menu actions and routes state transitions.
pub fn apply_pause_menu_actions(
    mut events: MessageReader<PauseMenuActionEvent>,
    mut pause_state: ResMut<PauseMenuState>,
    mut settings_open: ResMut<SettingsOpen>,
    mut next_state: ResMut<NextState<AppState>>,
    mut app_exit: MessageWriter<AppExit>,
) {
    for event in events.read() {
        match event {
            PauseMenuActionEvent::Toggle => match pause_state.mode {
                PauseMenuMode::Closed => {
                    set_pause_mode(&mut pause_state, &mut settings_open, PauseMenuMode::Menu)
                }
                PauseMenuMode::Menu => {
                    set_pause_mode(&mut pause_state, &mut settings_open, PauseMenuMode::Closed)
                }
                PauseMenuMode::Settings => {
                    set_pause_mode(&mut pause_state, &mut settings_open, PauseMenuMode::Menu)
                }
            },
            PauseMenuActionEvent::OpenMenu => {
                set_pause_mode(&mut pause_state, &mut settings_open, PauseMenuMode::Menu);
            }
            PauseMenuActionEvent::OpenSettings => {
                set_pause_mode(
                    &mut pause_state,
                    &mut settings_open,
                    PauseMenuMode::Settings,
                );
            }
            PauseMenuActionEvent::Resume | PauseMenuActionEvent::CloseAll => {
                set_pause_mode(&mut pause_state, &mut settings_open, PauseMenuMode::Closed);
            }
            PauseMenuActionEvent::ExitToMainMenu => {
                set_pause_mode(&mut pause_state, &mut settings_open, PauseMenuMode::Closed);
                next_state.set(AppState::MainMenu);
            }
            PauseMenuActionEvent::ExitGame => {
                app_exit.write(AppExit::Success);
            }
        }
    }
}

/// Resets pause + settings flags when leaving in-game.
pub fn reset_pause_state_on_exit(
    mut pause_state: ResMut<PauseMenuState>,
    mut settings_open: ResMut<SettingsOpen>,
) {
    set_pause_mode(&mut pause_state, &mut settings_open, PauseMenuMode::Closed);
}

fn set_pause_mode(
    pause_state: &mut ResMut<PauseMenuState>,
    settings_open: &mut ResMut<SettingsOpen>,
    mode: PauseMenuMode,
) {
    pause_state.mode = mode;
    settings_open.0 = mode == PauseMenuMode::Settings;
}

fn try_play_ui_audio(
    commands: &mut Commands,
    asset_server: Option<&AssetServer>,
    manifest: Option<&AssetManifest>,
    cache: Option<&mut AssetCache>,
    key: &str,
    volume: f32,
) {
    let Some(asset_server) = asset_server else {
        return;
    };
    let Some(manifest) = manifest else {
        return;
    };
    let Some(cache) = cache else {
        return;
    };

    if let Some(handle) = cache.get_or_load_audio(key, asset_server, manifest) {
        commands.spawn((
            AudioPlayer::new(handle),
            PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: bevy::audio::Volume::Linear(volume),
                ..default()
            },
        ));
    }
}
