use bevy::prelude::*;
use crate::components::critter::Critter;
use crate::components::tower::{Tower, TowerStats, TowerType};
use crate::events::fire_event::FireEvent;
use crate::resources::grid_map::FlatGrid;

pub fn combat_system(
    time: Res<Time>,
    _map: Res<FlatGrid>,
    mut tower_query: Query<(Entity, &mut TowerStats, &Tower, &Transform)>,
    critter_query: Query<(Entity, &Critter, &Transform)>,
    mut fire_events: EventWriter<FireEvent>,
) {
    let cell_size = 64.0;

    for (tower_entity, mut stats, tower, tower_transform) in tower_query.iter_mut() {
        // Decrement cooldown
        stats.current_cooldown -= time.delta_secs();
        if stats.current_cooldown > 0.0 {
            continue;
        }

        // Find target in range
        let tower_pos = tower_transform.translation.truncate();
        let range_px = stats.range * cell_size;

        let mut best_target: Option<(Entity, f32)> = None;

        for (critter_entity, _critter, critter_transform) in critter_query.iter() {
            let critter_pos = critter_transform.translation.truncate();
            let dist = tower_pos.distance(critter_pos);

            if dist <= range_px {
                // Simple: pick closest target
                match best_target {
                    None => best_target = Some((critter_entity, dist)),
                    Some((_, best_dist)) if dist < best_dist => {
                        best_target = Some((critter_entity, dist));
                    }
                    _ => {}
                }
            }
        }

        // Fire if target found
        if let Some((target, _)) = best_target {
            // No special effects for BottleTower in Phase 1
            let effect = match tower.tower_type {
                TowerType::Bottle => None,
                _ => None, // Other effects added in Phase 2
            };

            fire_events.send(FireEvent {
                shooter: tower_entity,
                target,
                tower_type: tower.tower_type,
                damage: stats.damage,
                effect,
            });

            stats.current_cooldown = stats.attack_cooldown;
        }
    }
}
