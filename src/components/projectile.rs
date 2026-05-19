use bevy::prelude::*;

use crate::components::effect::AppliedEffect;
use crate::components::tower::TowerType;

#[derive(Component)]
pub struct Projectile {
    pub target: Entity,
    pub speed: f32,
    pub damage: i32,
    pub effect: Option<AppliedEffect>,
    pub tower_type: TowerType,
    pub origin: Vec2,
}
