use bevy::prelude::*;

/// Global audio settings resource.
#[derive(Resource, Debug, Clone, Copy, Default)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub music_volume: f32,
}
