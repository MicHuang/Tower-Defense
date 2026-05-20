use bevy::prelude::*;
use crate::components::critter::{Critter, PathProgress};
use crate::components::effect::ActiveEffects;
use crate::resources::grid_map::FlatGrid;
use crate::systems::effect::effective_speed;

pub fn movement_system(
    time: Res<Time>,
    map: Res<FlatGrid>,
    mut query: Query<(Entity, &mut PathProgress, &Critter, &ActiveEffects, &mut Transform)>,
    mut commands: Commands,
) {
    let cell_size = 64.0;

    for (entity, mut progress, critter, active_effects, mut transform) in query.iter_mut() {
        // Get the path for this critter
        if progress.path_index >= map.paths.len() {
            continue;
        }
        let path = &map.paths[progress.path_index];
        if path.is_empty() {
            continue;
        }

        // Advance distance
        let speed = effective_speed(critter.base_speed, active_effects);
        let step = speed * time.delta_secs();
        progress.distance += step;

        // Move along segment
        while progress.segment_index < path.len() - 1 {
            let current = path[progress.segment_index];
            let next = path[progress.segment_index + 1];
            let seg_len = ((next.x as f32 - current.x as f32).powi(2)
                + (next.y as f32 - current.y as f32).powi(2))
            .sqrt()
                * cell_size;

            if progress.distance >= seg_len {
                progress.distance -= seg_len;
                progress.segment_index += 1;
            } else {
                // Lerp position along segment
                let t = progress.distance / seg_len.max(0.001);
                let x = current.x as f32 + (next.x as f32 - current.x as f32) * t;
                let y = current.y as f32 + (next.y as f32 - current.y as f32) * t;
                transform.translation = Vec3::new(x * cell_size, y * cell_size, 1.0);
                break;
            }
        }

        // Check if critter reached end of path
        if progress.segment_index >= path.len() - 1 && progress.distance > 0.0 {
            // Reached endpoint — despawn
            commands.entity(entity).despawn();
        }
    }
}
