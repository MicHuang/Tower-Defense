use bevy::prelude::*;

#[derive(Component)]
pub struct Critter {
    pub hp: i32,
    pub total_hp: i32,
    pub base_speed: f32,
    pub reward: i32,
    pub steal_amount: i32,
}

#[derive(Component)]
pub struct PathProgress {
    pub path_index: usize,
    pub segment_index: usize,
    pub distance: f32,
}
