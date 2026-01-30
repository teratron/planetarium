use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SaveData {
    pub last_session: Option<String>,
    pub total_playtime: f64,
}
