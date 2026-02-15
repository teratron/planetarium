//! # Modal Dialog System
//!
//! Provides modal dialogs for critical user interactions like exit confirmation.

// use bevy::hierarchy::BuildChildren; // Removed
use bevy::prelude::*;
use launcher::states::AppState;
use launcher::utils::despawn_recursive;
use localization::Localization;
use theme::Theme;

pub struct ModalPlugin;

impl Plugin for ModalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ModalState>();
        app.add_systems(Update, (handle_modal_buttons, update_modal_visibility));
    }
}

#[derive(Resource, Default, Debug, Clone)]
pub struct ModalState {
    pub active: Option<ModalType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModalType {
    ConfirmExit,
    ConfirmMainMenu,
    Error(String),
}

#[derive(Component)]
pub struct ModalRoot;

#[derive(Component, Clone, Copy)]
pub enum ModalAction {
    Confirm,
    Cancel,
}

/// Show exit confirmation modal.
pub fn show_exit_confirmation(mut modal_state: ResMut<ModalState>) {
    modal_state.active = Some(ModalType::ConfirmExit);
}

fn update_modal_visibility(
    mut commands: Commands,
    modal_state: Res<ModalState>,
    theme: Res<Theme>,
    loc: Res<Localization>,
    query: Query<Entity, With<ModalRoot>>,
    children_query: Query<&Children>,
) {
    if !modal_state.is_changed() {
        return;
    }

    // Despawn existing modal (recursive)
    for entity in query.iter() {
        despawn_recursive(&mut commands, entity, &children_query);
    }

    // Spawn new modal if needed
    if let Some(modal_type) = &modal_state.active {
        spawn_modal(&mut commands, &theme, &loc, modal_type);
    }
}

fn spawn_modal(commands: &mut Commands, theme: &Theme, loc: &Localization, modal_type: &ModalType) {
    let (title_key, message_key) = match modal_type {
        ModalType::ConfirmExit => ("modal-exit-title", "modal-exit-message"),
        ModalType::ConfirmMainMenu => ("modal-main-menu-title", "modal-main-menu-message"),
        ModalType::Error(_) => ("modal-error-title", "modal-error-generic"),
    };

    let title = loc.t(title_key);
    let message = if let ModalType::Error(msg) = modal_type {
        msg.clone()
    } else {
        loc.t(message_key)
    };

    let root_id = commands
        .spawn((
            ModalRoot,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            ZIndex(100),
        ))
        .id();

    let panel_id = commands
        .spawn((
            Node {
                width: Val::Px(500.0),
                border: UiRect::all(Val::Px(2.0)),
                padding: UiRect::all(theme.sizes.padding),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(theme.colors.surface),
        ))
        .id();

    commands.entity(root_id).add_child(panel_id);

    // Header
    let header_id = commands
        .spawn((
            Text::new(title),
            TextFont {
                font: theme.fonts.bold.clone(),
                font_size: theme.sizes.font_h2,
                ..default()
            },
            TextColor(theme.colors.accent),
            Node {
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            },
        ))
        .id();

    commands.entity(panel_id).add_child(header_id);

    // Content
    let content_id = commands
        .spawn((
            Text::new(message),
            TextFont {
                font: theme.fonts.main.clone(),
                font_size: theme.sizes.font_body,
                ..default()
            },
            TextColor(theme.colors.text_primary),
            Node {
                margin: UiRect::bottom(Val::Px(30.0)),
                ..default()
            },
        ))
        .id();

    commands.entity(panel_id).add_child(content_id);

    // Footer (Buttons)
    let foot_id = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceEvenly,
            ..default()
        })
        .id();

    commands.entity(panel_id).add_child(foot_id);

    spawn_modal_button(
        commands,
        foot_id,
        theme,
        loc,
        "modal-yes",
        ModalAction::Confirm,
        true,
    );
    spawn_modal_button(
        commands,
        foot_id,
        theme,
        loc,
        "modal-no",
        ModalAction::Cancel,
        false,
    );
}

fn spawn_modal_button(
    commands: &mut Commands,
    parent: Entity,
    theme: &Theme,
    loc: &Localization,
    key: &str,
    action: ModalAction,
    is_danger: bool,
) {
    let color = if is_danger {
        theme.colors.danger
    } else {
        theme.colors.accent
    };

    let button = commands
        .spawn((
            Button,
            action,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(color),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(loc.t(key)),
                TextFont {
                    font: theme.fonts.bold.clone(),
                    font_size: theme.sizes.font_body,
                    ..default()
                },
                TextColor(theme.colors.text_primary),
            ));
        })
        .id();

    commands.entity(parent).add_child(button);
}

#[allow(clippy::type_complexity)]
fn handle_modal_buttons(
    interaction_query: Query<(&Interaction, &ModalAction), (Changed<Interaction>, With<Button>)>,
    mut modal_state: ResMut<ModalState>,
    mut app_exit: MessageWriter<bevy::app::AppExit>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if let Some(modal_type) = &modal_state.active {
            match (modal_type, action) {
                (ModalType::ConfirmExit, ModalAction::Confirm) => {
                    info!("[Modal] Exit confirmed");
                    app_exit.write(bevy::app::AppExit::Success);
                }
                (ModalType::ConfirmMainMenu, ModalAction::Confirm) => {
                    info!("[Modal] MainMenu confirmed");
                    next_state.set(AppState::MainMenu);
                    modal_state.active = None;
                }
                (_, ModalAction::Cancel) | (ModalType::Error(_), _) => {
                    modal_state.active = None;
                }
            }
        }
    }
}
