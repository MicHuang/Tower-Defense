use bevy::prelude::*;
use crate::components::projectile::Projectile;
use crate::components::critter::Critter;
use crate::events::damage_event::DamageEvent;
use crate::events::kill_event::KillEvent;

pub fn projectile_movement_system(
    time: Res<Time>,
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &mut Projectile, &mut Transform)>,
    mut critter_query: Query<(Entity, &mut Critter, &Transform)>,
    mut damage_events: EventWriter<DamageEvent>,
    mut kill_events: EventWriter<KillEvent>,
) {
    for (projectile_entity, projectile, mut transform) in projectile_query.iter_mut() {
        // Find the target's current position
        let target_pos = critter_query
            .iter()
            .find(|(e, _, _)| *e == projectile.target)
            .map(|(_, _, t)| t.translation.truncate());

        let target_pos = match target_pos {
            Some(pos) => pos,
            None => {
                // Target is gone — despawn projectile
                commands.entity(projectile_entity).despawn();
                continue;
            }
        };

        let current_pos = transform.translation.truncate();
        let direction = target_pos - current_pos;
        let distance = direction.length();

        // Move toward target
        let step = projectile.speed * time.delta_secs();
        if step >= distance {
            // HIT! Apply damage
            transform.translation = target_pos.extend(2.0);

            // Apply damage to critter
            if let Ok((_, mut critter, _)) = critter_query.get_mut(projectile.target) {
                critter.hp -= projectile.damage;

                // Fire damage event
                damage_events.send(DamageEvent {
                    target: projectile.target,
                    amount: projectile.damage,
                });

                // Check if killed
                if critter.hp <= 0 {
                    kill_events.send(KillEvent {
                        target: projectile.target,
                        reward: critter.reward,
                        steal_amount: critter.steal_amount,
                    });
                }
            }

            commands.entity(projectile_entity).despawn();
        } else {
            // Move toward target
            let normalized = direction / distance;
            transform.translation += (normalized * step).extend(0.0);
        }
    }
}
