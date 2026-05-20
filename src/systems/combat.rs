use bevy::prelude::*;
use crate::components::critter::Critter;
use crate::components::effect::AppliedEffect;
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
                // Simple: pick closest target (strategy system in Phase 2+)
                match best_target {
                    None => best_target = Some((critter_entity, dist)),
                    Some((_, best_dist)) if dist < best_dist => {
                        best_target = Some((critter_entity, dist));
                    }
                    _ => {}
                }
            }
        }

        // Fire if target found, with tower-type-specific effect
        if let Some((target, _)) = best_target {
            let effect = tower_effect(tower.tower_type, stats.damage);

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

/// Return the AppliedEffect (if any) for a given tower type.
fn tower_effect(tower_type: TowerType, base_damage: f32) -> Option<AppliedEffect> {
    match tower_type {
        TowerType::Bottle => None, // pure damage, no effect
        TowerType::Sun => None,    // AOE all critters in range — pure damage splash
        TowerType::Moon => Some(AppliedEffect::Slow {
            multiplier: 0.5,
            duration: 2.0,
            timer: 2.0,
        }),
        TowerType::Fire => Some(AppliedEffect::Dot {
            damage: (base_damage * 0.3) as i32,
            tick_interval: 1.0,
            ticks_remaining: 3,
            next_tick_timer: 1.0,
        }),
        TowerType::Ice => Some(AppliedEffect::Slow {
            multiplier: 0.3, // stronger slow than Moon
            duration: 3.0,
            timer: 3.0,
        }),
        TowerType::Splash => Some(AppliedEffect::Splash {
            damage: (base_damage * 0.5) as i32,
            range: 3.0, // grid cells
        }),
        TowerType::Energy => None, // Motivate is applied to nearby towers, not critters
    }
}
