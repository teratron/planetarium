use bevy::prelude::*;
use crate::ui::theme::{colors, fonts};

pub fn spawn_menu_button<T: Component + Copy>(
    parent: &mut ChildBuilder, 
    button_action: T, 
    label: &str
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(280.0),
                height: Val::Px(56.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(16.0)),
                ..default()
            },
            BackgroundColor(colors::BUTTON_NORMAL),
            BorderColor(colors::BUTTON_BORDER),
            BorderRadius::all(Val::Px(8.0)),
            button_action,
        ))
        .with_child((
            Text::new(label),
            TextFont {
                font_size: fonts::BUTTON_TEXT_SIZE,
                ..default()
            },
            TextColor(colors::TEXT_PRIMARY),
        ));
}
