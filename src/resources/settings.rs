use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GameSettings {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub graphics_quality: GraphicsQuality,
}

#[derive(Default, Clone, Copy)]
pub enum GraphicsQuality {
    Low,
    #[default]
    Medium,
    High,
    Ultra,
}
