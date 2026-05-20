use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TowerType {
    Bottle,
    Fire,
    Ice,
    Splash,
    Sun,
    Moon,
    Energy,
}

#[derive(Component)]
pub struct Tower {
    pub tower_type: TowerType,
    pub level: u32,
}

#[derive(Component)]
pub struct TowerStats {
    pub range: f32,
    pub damage: f32,
    pub attack_cooldown: f32,
    pub current_cooldown: f32,
}

#[derive(Component)]
pub struct TargetSelector {
    pub strategy: TargetingStrategy,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TargetingStrategy {
    ClosestToEnd,
    Strongest,
    Weakest,
    First,
}
