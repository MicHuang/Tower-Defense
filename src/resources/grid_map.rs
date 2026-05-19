use crate::components::map::Tile;
use crate::data::map_config::MapConfig;
use bevy::prelude::*;

#[derive(Resource)]
pub struct FlatGrid {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Tile>,
    pub paths: Vec<Vec<UVec2>>,
    pub start_points: Vec<UVec2>,
    pub end_points: Vec<UVec2>,
}

impl FlatGrid {
    pub fn from_config(config: &MapConfig) -> Self {
        let mut tiles = Vec::with_capacity(config.tile_count());
        for tile_str in &config.tiles {
            let tile = match tile_str.as_str() {
                "road_start" => Tile::Road(crate::components::map::TileType::RoadStart),
                "road_end" => Tile::Road(crate::components::map::TileType::RoadEnd),
                "road" => Tile::Road(crate::components::map::TileType::Road),
                "scenery" => Tile::Scenery,
                _ => Tile::Empty,
            };
            tiles.push(tile);
        }

        let to_uv = |def: &crate::data::map_config::UVec2Def| UVec2::new(def.x, def.y);
        let paths = config
            .paths
            .iter()
            .map(|p| p.iter().map(to_uv).collect())
            .collect();
        let start_points = config.start_points.iter().map(to_uv).collect();
        let end_points = config.end_points.iter().map(to_uv).collect();

        FlatGrid {
            width: config.width,
            height: config.height,
            tiles,
            paths,
            start_points,
            end_points,
        }
    }

    pub fn index(&self, x: u32, y: u32) -> usize {
        assert!(
            x < self.width && y < self.height,
            "grid index out of bounds"
        );
        (y * self.width + x) as usize
    }

    pub fn tile_at(&self, x: u32, y: u32) -> &Tile {
        &self.tiles[self.index(x, y)]
    }
}
