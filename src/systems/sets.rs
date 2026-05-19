use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSet {
    Wave,
    Movement,
    Combat,
    ProjectileSpawn,
    ProjectileMove,
    Effects,
    Scoring,
}
