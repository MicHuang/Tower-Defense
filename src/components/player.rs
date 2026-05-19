use bevy::prelude::*;

#[derive(Resource)]
pub struct Player {
    pub coins: u32,
    pub life: u32,
    pub score: u32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            coins: 500,
            life: 20,
            score: 0,
        }
    }
}

#[derive(Resource)]
pub struct WaveState {
    pub current_index: usize,
    pub phase: WavePhase,
    pub spawn_timer: f32,
    pub critters_spawned: usize,
}

impl Default for WaveState {
    fn default() -> Self {
        WaveState {
            current_index: 0,
            phase: WavePhase::Waiting,
            spawn_timer: 0.0,
            critters_spawned: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum WavePhase {
    Waiting,
    Producing,
    Produced,
}
