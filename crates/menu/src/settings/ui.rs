//! Settings UI construction.
//!
//! Spawning and layout for the settings panel and its static elements.

use super::SettingsTab;
use super::components::{
    FadeDirection, SettingsContentArea, SettingsFade, SettingsRoot, SettingsTabButton,
};
use super::layout;
use crate::widgets::{ButtonAction, spawn_primary_button};
use bevy::prelude::*;
use localization::{Localization, LocalizedStrings};
use theme::{Theme, constants};

/// Spawns the settings UI root and basic layout.
pub fn spawn_settings_menu(
    commands: &mut Commands,
    theme: &Theme,
    loc: &Localization,
    strings: &mut LocalizedStrings,
) -> Entity {
    info!("[Settings] Spawning settings UI...");

    // 1. Root overlay
    let root_id = commands
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
            BackgroundColor(theme.colors.surface.with_alpha(0.0)), // Start transparent
            Transform::from_scale(Vec3::splat(0.9)),               // Start slightly smaller
            SettingsFade {
                timer: Timer::from_seconds(constants::timing::SETTINGS_FADE_IN, TimerMode::Once),
                direction: FadeDirection::In,
            },
        ))
        .id();

    // Title
    let title_id = commands
        .spawn((
            Text::new(strings.get("settings-title", loc)),
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
        ))
        .id();

    commands.entity(root_id).add_child(title_id);

    // Tab Hub
    let hub_id = commands
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
        .id();

    commands.entity(root_id).add_child(hub_id);

    spawn_tab_button(
        commands,
        hub_id,
        theme,
        loc,
        "tab-graphics",
        SettingsTab::Graphics,
    );
    spawn_tab_button(
        commands,
        hub_id,
        theme,
        loc,
        "tab-audio",
        SettingsTab::Audio,
    );
    spawn_tab_button(
        commands,
        hub_id,
        theme,
        loc,
        "tab-controls",
        SettingsTab::Controls,
    );
    spawn_tab_button(
        commands,
        hub_id,
        theme,
        loc,
        "tab-general",
        SettingsTab::General,
    );

    // 3. Content Area
    let content_area_id = commands
        .spawn((
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
        ))
        .id();

    commands.entity(root_id).add_child(content_area_id);

    // 4. Button Area
    let btn_area_id = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: layout::button_area::HEIGHT,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .id();

    commands.entity(root_id).add_child(btn_area_id);

    // Apply Button
    let apply_btn_id = commands
        .spawn((
            Button,
            super::ApplyChangesButton,
            Node {
                width: Val::Px(120.0),
                height: Val::Px(40.0),
                margin: UiRect::right(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(theme.colors.accent),
        ))
        .id();

    commands.entity(btn_area_id).add_child(apply_btn_id);

    commands.entity(apply_btn_id).with_children(|btn| {
        btn.spawn((
            Text::new(strings.get("settings-apply", loc)),
            TextFont {
                font: theme.fonts.bold.clone(),
                font_size: theme.sizes.font_body,
                ..default()
            },
            TextColor(theme.colors.text_primary),
        ));
    });

    // Reset Button
    let reset_btn_id = commands
        .spawn((
            Button,
            super::ResetChangesButton,
            Node {
                width: Val::Px(120.0),
                height: Val::Px(40.0),
                margin: UiRect::right(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(theme.colors.danger),
        ))
        .id();

    commands.entity(btn_area_id).add_child(reset_btn_id);

    commands.entity(reset_btn_id).with_children(|btn| {
        btn.spawn((
            Text::new(strings.get("settings-reset", loc)),
            TextFont {
                font: theme.fonts.bold.clone(),
                font_size: theme.sizes.font_body,
                ..default()
            },
            TextColor(theme.colors.text_primary),
        ));
    });

    // Back Button
    spawn_primary_button(
        commands,
        theme,
        &strings.get("menu-back", loc),
        ButtonAction::Back,
        btn_area_id,
    );

    root_id
}

/// Internal helper to spawn a single tab button.
fn spawn_tab_button(
    commands: &mut Commands,
    parent: Entity,
    theme: &Theme,
    loc: &Localization,
    key: &str,
    tab: SettingsTab,
) {
    let label = loc.t(key);

    let button_id = commands
        .spawn((
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::right(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(theme.colors.surface),
            SettingsTabButton(tab),
        ))
        .id();

    commands.entity(parent).add_child(button_id);

    let label_id = commands
        .spawn((
            Text::new(label),
            TextFont {
                font: theme.fonts.main.clone(),
                font_size: theme.sizes.font_body,
                ..default()
            },
            TextColor(theme.colors.text_primary),
        ))
        .id();

    commands.entity(button_id).add_child(label_id);
}
