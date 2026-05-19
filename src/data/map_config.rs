use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct UVec2Def {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Deserialize)]
pub struct MapConfig {
    pub width: u32,
    pub height: u32,
    pub start_points: Vec<UVec2Def>,
    pub end_points: Vec<UVec2Def>,
    pub paths: Vec<Vec<UVec2Def>>,
    pub tiles: Vec<String>,
}

impl MapConfig {
    pub fn load() -> Result<Self, toml::de::Error> {
        let content = include_str!("../../assets/maps/demo_map.toml");
        toml::from_str(content)
    }

    pub fn tile_count(&self) -> usize {
        (self.width * self.height) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_demo_map() {
        let config = MapConfig::load().expect("Failed to load map");
        assert_eq!(config.width, 8);
        assert_eq!(config.height, 6);
        assert_eq!(config.tiles.len() as u32, config.width * config.height);
        assert_eq!(config.start_points.len(), 1);
        assert_eq!(config.end_points.len(), 1);
        assert_eq!(config.paths.len(), 1);
    }
}
