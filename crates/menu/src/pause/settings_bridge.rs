//! Bridge systems reusing settings UI inside the in-game pause flow.

use crate::settings::{
    animate_settings_fade, components, handle_settings_tab_clicks, spawn_settings_if_needed,
    update_settings_tab_content, update_settings_ui, ActiveSettingsTab, SettingsOpen, UserSettings,
};
use bevy::prelude::*;
use localization::{Localization, LocalizedStrings};
use theme::Theme;

/// Reuse settings spawn/despawn logic for the pause flow.
pub fn spawn_settings_if_needed_bridge(
    commands: Commands,
    theme: Res<Theme>,
    loc: Res<Localization>,
    strings: ResMut<LocalizedStrings>,
    settings_open: Res<SettingsOpen>,
    query: Query<Entity, With<crate::settings::components::SettingsRoot>>,
    active_tab: ResMut<ActiveSettingsTab>,
) {
    spawn_settings_if_needed(
        commands,
        theme,
        loc,
        strings,
        settings_open,
        query,
        active_tab,
    );
}

/// Reuse tab click handling.
#[allow(clippy::type_complexity)]
pub fn handle_settings_tab_clicks_bridge(
    tab_query: Query<
        (
            &Interaction,
            &crate::settings::components::SettingsTabButton,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    active_tab: ResMut<ActiveSettingsTab>,
    localization: Res<Localization>,
) {
    handle_settings_tab_clicks(tab_query, active_tab, localization);
}

/// Reuse dynamic tab content rendering.
#[allow(clippy::too_many_arguments)]
pub fn update_settings_tab_content_bridge(
    commands: Commands,
    active_tab: Res<ActiveSettingsTab>,
    theme: Res<Theme>,
    loc: Res<Localization>,
    strings: ResMut<LocalizedStrings>,
    settings: Res<UserSettings>,
    content_area_query: Query<Entity, With<crate::settings::components::SettingsContentArea>>,
    children_query: Query<&Children>,
) {
    update_settings_tab_content(
        commands,
        active_tab,
        theme,
        loc,
        strings,
        settings,
        content_area_query,
        children_query,
    );
}

/// Reuse settings fade animation.
pub fn animate_settings_fade_bridge(
    commands: Commands,
    time: Res<Time>,
    theme: Res<Theme>,
    query: Query<(
        Entity,
        &mut BackgroundColor,
        &mut Transform,
        &mut components::SettingsFade,
    )>,
    children_query: Query<&Children>,
) {
    animate_settings_fade(commands, time, theme, query, children_query);
}

/// Reuse settings value-to-UI synchronization.
#[allow(clippy::type_complexity)]
pub fn update_settings_ui_bridge(
    settings: Res<UserSettings>,
    queries: ParamSet<(
        Query<&mut Text, With<crate::settings::components::MasterVolumeControl>>,
        Query<&mut Text, With<crate::settings::components::MusicVolumeControl>>,
        Query<&mut Text, With<crate::settings::components::SFXVolumeControl>>,
    )>,
) {
    update_settings_ui(settings, queries);
}
