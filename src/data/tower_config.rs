use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct TowerConfig {
    pub name: String,
    pub damage: f32,
    pub range: f32,
    pub attack_cooldown: f32,
    pub projectile_speed: f32,
    pub upgrade_prices: Vec<u32>,
    pub sell_ratios: Vec<f32>,
    pub cell_image: String,
    #[serde(default)]
    pub burn_damage: Option<f32>,
    #[serde(default)]
    pub burn_interval: Option<f32>,
    #[serde(default)]
    pub burn_ticks: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct TowerConfigs {
    pub tower: std::collections::HashMap<String, TowerConfig>,
}

impl TowerConfigs {
    pub fn load() -> Result<Self, toml::de::Error> {
        // include_str! resolves relative to this source file: src/data/
        // So "../../" goes up from src/data/ to project root
        let content = include_str!("../../assets/config/towers.toml");
        toml::from_str(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_bottle_tower() {
        let configs = TowerConfigs::load().expect("Failed to load tower config");
        let bottle = configs.tower.get("bottle").expect("BottleTower not found");
        assert_eq!(bottle.name, "Bottle Tower");
        assert!(bottle.damage > 0.0);
        assert_eq!(bottle.upgrade_prices.len(), 3);
    }
}
