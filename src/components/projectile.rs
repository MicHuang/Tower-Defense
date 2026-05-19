use bevy::prelude::*;

use crate::components::effect::AppliedEffect;
use crate::components::tower::TowerType;

#[derive(Component)]
pub struct Projectile {
    pub target: Entity,
    pub target_pos: Vec2,     // cached target position, updated each frame
    pub speed: f32,
    pub damage: i32,
    pub effect: Option<AppliedEffect>,
    pub tower_type: TowerType,
    pub origin: Vec2,
}
