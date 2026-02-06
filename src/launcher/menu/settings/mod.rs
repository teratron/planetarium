//! Settings Screen UI
//!
//! Implements a professional modal settings panel with categorized tabs:
//! Graphics, Audio, Controls, and General.
//! The panel is fully localized and synced with `UserSettings`.

use super::widgets::{ButtonAction, spawn_primary_button};
use crate::core::config::UserSettings;
use crate::core::localization::Localization;
use crate::ui::theme::Theme;
use bevy::prelude::*;

pub mod components;
pub mod layout;
pub mod tabs;

pub use components::*;
pub use layout::panel as panel_layout;

/// Resource tracking visibility.
#[derive(Resource, Default, Debug, Clone)]
pub struct SettingsOpen(pub bool);

/// Categories available in the settings menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
pub enum SettingsTab {
    #[default]
    Graphics,
    Audio,
    Controls,
    General,
}

/// Resource tracking the currently active tab in the settings menu.
#[derive(Resource, Default, Debug, Clone)]
pub struct ActiveSettingsTab(pub SettingsTab);

/// Marker for the back button in settings.
#[derive(Component)]
pub struct SettingsBackButton;

/// Spawns the settings UI root and basic layout.
pub fn spawn_settings_menu(commands: &mut Commands, theme: &Theme, loc: &Localization) -> Entity {
    info!("[Settings] Spawning settings UI...");

    // 1. Root overlay
    let root = commands
        .spawn((
            SettingsRoot,
            Node {
                width: layout::panel::WIDTH,
                height: layout::panel::HEIGHT,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                padding: UiRect::all(theme.sizes.padding),
                ..default()
            },
            BackgroundColor(theme.colors.surface),
        ))
        .id();

    // 2. Title & Tabs
    commands.entity(root).with_children(|parent| {
        // Title
        parent.spawn((
            Text::new(loc.t("settings-title")),
            TextFont {
                font: theme.fonts.bold.clone(),
                font_size: theme.sizes.font_h2,
                ..default()
            },
            TextColor(theme.colors.text_primary),
            Node {
                margin: UiRect::top(layout::panel::TITLE_MARGIN_TOP),
                ..default()
            },
        ));

        // Tab Hub
        parent
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: layout::tabs::HEIGHT,
                    margin: UiRect::top(layout::tabs::MARGIN_TOP),
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(theme.colors.background),
            ))
            .with_children(|tabs| {
                spawn_tab_button(tabs, theme, loc, "tab-graphics", SettingsTab::Graphics);
                spawn_tab_button(tabs, theme, loc, "tab-audio", SettingsTab::Audio);
                spawn_tab_button(tabs, theme, loc, "tab-controls", SettingsTab::Controls);
                spawn_tab_button(tabs, theme, loc, "tab-general", SettingsTab::General);
            });

        // 3. Content Area (will be hydrated by system)
        parent.spawn((
            SettingsContentArea,
            Node {
                width: Val::Percent(100.0),
                height: layout::content::HEIGHT,
                margin: UiRect::all(layout::content::MARGIN),
                padding: UiRect::all(layout::content::PADDING),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(theme.colors.background),
        ));

        // 4. Back Button
        parent
            .spawn((Node {
                width: Val::Percent(100.0),
                height: layout::button_area::HEIGHT,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },))
            .with_children(|btn_area| {
                let parent = btn_area.target_entity();
                let commands = btn_area.commands_mut();
                spawn_primary_button(
                    commands,
                    theme,
                    &loc.t("menu-back"),
                    ButtonAction::Back,
                    parent,
                );
            });
    });

    root
}

/// Internal helper to spawn a single tab button.
fn spawn_tab_button(
    parent: &mut bevy::ecs::hierarchy::ChildSpawnerCommands,
    theme: &Theme,
    loc: &Localization,
    key: &str,
    tab: SettingsTab,
) {
    parent
        .spawn((
            Button,
            SettingsTabButton(tab),
            Node {
                padding: UiRect::horizontal(Val::Px(15.0)),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(theme.colors.background),
        ))
        .with_children(|b| {
            b.spawn((
                Text::new(loc.t(key)),
                TextFont {
                    font: theme.fonts.main.clone(),
                    font_size: theme.sizes.font_body,
                    ..default()
                },
                TextColor(theme.colors.text_secondary),
            ));
        });
}

/// System to handle tab switching.
#[allow(clippy::type_complexity)]
pub fn handle_settings_tab_clicks(
    mut tab_query: Query<(&Interaction, &SettingsTabButton), (Changed<Interaction>, With<Button>)>,
    mut active_tab: ResMut<ActiveSettingsTab>,
) {
    for (interaction, tab_btn) in &mut tab_query {
        if *interaction == Interaction::Pressed && active_tab.0 != tab_btn.0 {
            info!("[Settings] Switching to tab: {:?}", tab_btn.0);
            active_tab.0 = tab_btn.0;
        }
    }
}

/// System to hydrate/refresh the content area when tab changes.
pub fn update_settings_tab_content(
    mut commands: Commands,
    active_tab: Res<ActiveSettingsTab>,
    theme: Res<Theme>,
    loc: Res<Localization>,
    settings: Res<UserSettings>,
    content_area_query: Query<Entity, With<SettingsContentArea>>,
    children_query: Query<&Children>,
) {
    if !active_tab.is_changed() {
        return;
    }

    if let Ok(content_area) = content_area_query.single() {
        // 1. Clear existing content
        if let Ok(children) = children_query.get(content_area) {
            for child in children.iter() {
                commands.entity(child).despawn();
            }
        }

        // 2. Spawn new panel based on tab
        commands
            .entity(content_area)
            .with_children(|parent| match active_tab.0 {
                SettingsTab::Graphics => tabs::spawn_graphics_tab(parent, &theme, &loc, &settings),
                SettingsTab::Audio => tabs::spawn_audio_tab(parent, &theme, &loc, &settings),
                SettingsTab::Controls => tabs::spawn_controls_tab(parent, &theme, &loc, &settings),
                SettingsTab::General => tabs::spawn_general_tab(parent, &theme, &loc, &settings),
            });
    }
}

/// Spawns or despawns settings depending on `SettingsOpen` resource.
pub fn spawn_settings_if_needed(
    mut commands: Commands,
    theme: Res<Theme>,
    loc: Res<Localization>,
    settings_open: Res<SettingsOpen>,
    query: Query<Entity, With<SettingsRoot>>,
    mut active_tab: ResMut<ActiveSettingsTab>,
) {
    if settings_open.is_changed() {
        if settings_open.0 && query.is_empty() {
            // Reset tab to default when opening
            active_tab.0 = SettingsTab::default();
            spawn_settings_menu(&mut commands, &theme, &loc);
        } else if !settings_open.0 && !query.is_empty() {
            for e in &query {
                commands.entity(e).despawn();
            }
        }
    }
}

/// Update UI display values to match current UserSettings.
#[allow(clippy::type_complexity)]
pub fn update_settings_ui(
    settings: Res<UserSettings>,
    mut master_q: Query<
        &mut Text,
        (
            With<MasterVolumeControl>,
            Without<MusicVolumeControl>,
            Without<SFXVolumeControl>,
        ),
    >,
    mut music_q: Query<
        &mut Text,
        (
            With<MusicVolumeControl>,
            Without<MasterVolumeControl>,
            Without<SFXVolumeControl>,
        ),
    >,
    mut sfx_q: Query<
        &mut Text,
        (
            With<SFXVolumeControl>,
            Without<MasterVolumeControl>,
            Without<MusicVolumeControl>,
        ),
    >,
) {
    if let Ok(mut text) = master_q.single_mut() {
        text.0 = format!("{:.2}", settings.audio.master_volume);
    }
    if let Ok(mut text) = music_q.single_mut() {
        text.0 = format!("{:.2}", settings.audio.music_volume);
    }
    if let Ok(mut text) = sfx_q.single_mut() {
        text.0 = format!("{:.2}", settings.audio.sfx_volume);
    }
}
