use bevy::prelude::*;

use crate::components::effect::AppliedEffect;
use crate::components::tower::TowerType;

#[derive(Event)]
pub struct FireEvent {
    pub shooter: Entity,
    pub target: Entity,
    pub tower_type: TowerType,
    pub damage: f32,
    pub effect: Option<AppliedEffect>,
}
