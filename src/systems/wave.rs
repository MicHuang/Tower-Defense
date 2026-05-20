use bevy::prelude::*;
use crate::components::critter::{Critter, PathProgress};
use crate::components::effect::ActiveEffects;
use crate::components::player::WaveState;
use crate::data::wave_config::WaveConfigs;
use crate::resources::grid_map::FlatGrid;

pub struct WavePlugin;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, wave_system);
    }
}

pub fn wave_system(
    mut wave_state: ResMut<WaveState>,
    time: Res<Time>,
    map: Res<FlatGrid>,
    wave_configs: Res<WaveConfigs>,
    mut commands: Commands,
) {
    // Only act in Producing phase
    if wave_state.phase != crate::components::player::WavePhase::Producing {
        return;
    }

    // Decrement spawn timer
    wave_state.spawn_timer -= time.delta_secs();
    if wave_state.spawn_timer <= 0.0 {
        // Get current wave definition
        if wave_state.current_index >= wave_configs.wave.len() {
            return;
        }
        let wave_def = &wave_configs.wave[wave_state.current_index];

        // Spawn a critter if we haven't spawned all
        if wave_state.critters_spawned < wave_def.spawn_count as usize {
            // Get start point
            if map.start_points.is_empty() {
                return;
            }
            let start = map.start_points[0];

            // Spawn critter entity
            commands.spawn((
                Critter {
                    hp: wave_def.hitpoints,
                    total_hp: wave_def.hitpoints,
                    base_speed: wave_def.speed,
                    reward: wave_def.reward,
                    steal_amount: wave_def.steal_amount,
                },
                ActiveEffects::default(),
                PathProgress {
                    path_index: 0,
                    segment_index: 0,
                    distance: 0.0,
                },
                // Sprite will be added later — use a marker for now
                SpatialBundle::from_transform(Transform::from_xyz(
                    start.x as f32 * 64.0,
                    start.y as f32 * 64.0,
                    1.0,
                )),
            ));

            wave_state.critters_spawned += 1;
            wave_state.spawn_timer = wave_def.spawn_interval;
        }

        // Check if all critters for this wave have been spawned
        if wave_state.critters_spawned >= wave_def.spawn_count as usize {
            wave_state.phase = crate::components::player::WavePhase::Produced;
        }
    }
}
