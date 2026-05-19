use bevy::prelude::*;

#[derive(Event)]
pub struct KillEvent {
    pub target: Entity,
    pub reward: i32,
    pub steal_amount: i32,
}
