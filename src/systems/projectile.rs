use bevy::prelude::*;
use crate::components::projectile::Projectile;
use crate::components::critter::Critter;
use crate::components::effect::{ActiveEffects, AppliedEffect};
use crate::events::damage_event::DamageEvent;
use crate::events::kill_event::KillEvent;

/// Updates projectile target positions from live critter positions
pub fn projectile_target_update_system(
    mut projectile_query: Query<&mut Projectile>,
    critter_query: Query<(Entity, &Transform)>,
) {
    for mut projectile in projectile_query.iter_mut() {
        if let Ok((_, transform)) = critter_query.get(projectile.target) {
            projectile.target_pos = transform.translation.truncate();
        }
    }
}

/// Moves projectiles toward their target, applies damage on hit
pub fn projectile_movement_system(
    time: Res<Time>,
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &Projectile, &mut Transform)>,
    mut critter_hp_query: Query<(&mut Critter, &mut ActiveEffects)>,
    mut damage_events: EventWriter<DamageEvent>,
    mut kill_events: EventWriter<KillEvent>,
) {
    // Collect hits to process outside the iteration to avoid borrow conflicts
    let mut hits: Vec<(Entity, i32, Option<AppliedEffect>)> = Vec::new();

    for (projectile_entity, projectile, mut transform) in projectile_query.iter_mut() {
        let current_pos = transform.translation.truncate();
        let target_pos = projectile.target_pos;
        let direction = target_pos - current_pos;
        let distance = direction.length();

        let step = projectile.speed * time.delta_secs();
        if step >= distance {
            // HIT!
            transform.translation = target_pos.extend(2.0);
            hits.push((projectile.target, projectile.damage, projectile.effect.clone()));
            commands.entity(projectile_entity).despawn();
        } else {
            let normalized = direction / distance;
            transform.translation += (normalized * step).extend(0.0);
        }
    }

    // Process hits (separate from projectile iteration)
    for (target, damage, effect) in hits {
        if let Ok((mut critter, mut active_effects)) = critter_hp_query.get_mut(target) {
            critter.hp -= damage;
            damage_events.send(DamageEvent {
                target,
                amount: damage,
            });

            // Apply effect to critter (Slow, Dot, etc.)
            if let Some(effect) = effect {
                active_effects.effects.push(effect);
            }

            if critter.hp <= 0 {
                let reward = critter.reward;
                let steal_amount = critter.steal_amount;
                kill_events.send(KillEvent { target, reward, steal_amount });
            }
        }
    }
}
