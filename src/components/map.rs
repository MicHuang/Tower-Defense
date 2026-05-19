use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TileType {
    Empty,
    Road,
    RoadStart,
    RoadEnd,
    Scenery,
}

#[derive(Debug, Clone)]
pub enum Tile {
    Empty,
    Road(TileType),
    Tower(Option<Entity>),
    Scenery,
}
