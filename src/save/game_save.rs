use serde::{Deserialize, Serialize};
use crate::components::tower::TowerType;

/// Serializable snapshot of the entire game state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSave {
    pub player: PlayerSave,
    pub wave: WaveSave,
    pub towers: Vec<TowerSave>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSave {
    pub coins: u32,
    pub life: u32,
    pub score: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaveSave {
    pub current_index: usize,
    pub phase: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerSave {
    pub grid_x: u32,
    pub grid_y: u32,
    pub tower_type: TowerType,
    pub level: u32,
    pub damage: f32,
    pub range: f32,
    pub attack_cooldown: f32,
}

impl GameSave {
    pub fn new(player: PlayerSave, wave: WaveSave, towers: Vec<TowerSave>) -> Self {
        GameSave {
            player,
            wave,
            towers,
            version: "0.1.0".into(),
        }
    }
}
