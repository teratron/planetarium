//! Base widget trait for reusable UI elements.

use crate::ui::theme::Theme;
use bevy::prelude::*;

/// Common interface for widget creation.
pub trait Widget {
    type Spec;

    fn spawn(commands: &mut Commands, theme: &Theme, spec: Self::Spec, parent: Entity) -> Entity;
}
