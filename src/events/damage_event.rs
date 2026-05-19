use bevy::prelude::*;

pub struct DamageEvent {
    pub target: Entity,
    pub amount: i32,
}
