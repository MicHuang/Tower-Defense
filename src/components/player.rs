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

impl Player {
    pub fn earn_coins(&mut self, amount: i32) {
        if amount > 0 {
            self.coins = self.coins.saturating_add(amount as u32);
        }
    }

    pub fn spend_coins(&mut self, amount: u32) -> bool {
        if self.coins >= amount {
            self.coins -= amount;
            true
        } else {
            false
        }
    }

    pub fn alter_life(&mut self, amount: i32) {
        if amount < 0 {
            self.life = self.life.saturating_sub(amount.unsigned_abs());
        } else {
            self.life = self.life.saturating_add(amount as u32);
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
