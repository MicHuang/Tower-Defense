use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct WaveDef {
    pub spawn_count: u32,
    pub spawn_interval: f32,
    pub hitpoints: i32,
    pub speed: f32,
    pub reward: i32,
    pub steal_amount: i32,
}

#[derive(Debug, Deserialize, Resource)]
pub struct WaveConfigs {
    pub wave: Vec<WaveDef>,
}

impl WaveConfigs {
    pub fn load() -> Result<Self, toml::de::Error> {
        let content = include_str!("../../assets/config/waves.toml");
        toml::from_str(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_wave_config() {
        let configs = WaveConfigs::load().expect("Failed to load wave config");
        assert!(!configs.wave.is_empty());
        assert!(configs.wave[0].spawn_count > 0);
    }
}
