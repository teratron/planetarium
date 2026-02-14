//! Settings UI construction.
//!
//! Spawning and layout for the settings panel and its static elements.

use super::SettingsTab;
use super::components::{
    FadeDirection, SettingsContentArea, SettingsFade, SettingsRoot, SettingsTabButton,
};
use super::layout;
use crate::framework::localization::{Localization, LocalizedStrings};
use crate::framework::ui::theme::Theme;
use crate::framework::ui::theme::constants;
use crate::framework::ui::widgets::{ButtonAction, spawn_primary_button};
use bevy::prelude::*;

/// Spawns the settings UI root and basic layout.
///
/// # Examples
/// ```no_run
/// # use bevy::prelude::*;
/// # use planetarium::framework::settings::ui::spawn_settings_menu;
/// # use planetarium::framework::ui::theme::Theme;
/// # use planetarium::framework::localization::{Localization, LocalizedStrings};
/// fn example_system(
///     mut commands: Commands,
///     theme: Res<Theme>,
///     loc: Res<Localization>,
///     mut strings: ResMut<LocalizedStrings>,
/// ) {
///     spawn_settings_menu(&mut commands, &theme, &loc, &mut strings);
/// }
/// ```
pub fn spawn_settings_menu(
    commands: &mut Commands,
    theme: &Theme,
    loc: &Localization,
    strings: &mut LocalizedStrings,
) -> Entity {
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
            BackgroundColor(theme.colors.surface.with_alpha(0.0)), // Start transparent
            Transform::from_scale(Vec3::splat(0.9)),               // Start slightly smaller
            SettingsFade {
                timer: Timer::from_seconds(constants::timing::SETTINGS_FADE_IN, TimerMode::Once),
                direction: FadeDirection::In,
            },
        ))
        .id();

    // 2. Title & Tabs
    commands.entity(root).with_children(|parent| {
        // Title
        parent.spawn((
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
                spawn_tab_button(
                    tabs,
                    theme,
                    loc,
                    strings,
                    "tab-graphics",
                    SettingsTab::Graphics,
                );
                spawn_tab_button(tabs, theme, loc, strings, "tab-audio", SettingsTab::Audio);
                spawn_tab_button(
                    tabs,
                    theme,
                    loc,
                    strings,
                    "tab-controls",
                    SettingsTab::Controls,
                );
                spawn_tab_button(
                    tabs,
                    theme,
                    loc,
                    strings,
                    "tab-general",
                    SettingsTab::General,
                );
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

                // Apply Button
                commands
                    .spawn((
                        Button,
                        crate::framework::settings::ApplyChangesButton,
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
                    .with_children(|btn| {
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
                commands
                    .spawn((
                        Button,
                        crate::framework::settings::ResetChangesButton,
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
                    .with_children(|btn| {
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
    strings: &mut LocalizedStrings,
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
                Text::new(strings.get(key, loc)),
                TextFont {
                    font: theme.fonts.main.clone(),
                    font_size: theme.sizes.font_body,
                    ..default()
                },
                TextColor(theme.colors.text_secondary),
            ));
        });
}
