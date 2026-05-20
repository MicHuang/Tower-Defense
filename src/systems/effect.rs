use bevy::prelude::*;
use crate::components::critter::Critter;
use crate::components::effect::{ActiveEffects, AppliedEffect};
use crate::components::tower::{Tower, TowerStats};
use crate::events::damage_event::DamageEvent;
use crate::events::kill_event::KillEvent;

/// Tick all active effects: apply damage, decrement timers, remove expired ones.
pub fn effect_system(
    time: Res<Time>,
    mut critter_query: Query<(Entity, &mut Critter, &mut ActiveEffects)>,
    mut tower_query: Query<&mut TowerStats, (With<Tower>, With<ActiveEffects>)>,
    mut damage_events: EventWriter<DamageEvent>,
    mut kill_events: EventWriter<KillEvent>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();

    // --- Tick effects on critters ---
    for (entity, mut critter, mut active_effects) in critter_query.iter_mut() {
        let mut expired = Vec::new();

        for (i, effect) in active_effects.effects.iter_mut().enumerate() {
            match effect {
                AppliedEffect::Slow { timer, .. } => {
                    *timer -= dt;
                    if *timer <= 0.0 {
                        expired.push(i);
                    }
                }
                AppliedEffect::Dot {
                    damage,
                    tick_interval,
                    ticks_remaining,
                    next_tick_timer,
                } => {
                    *next_tick_timer -= dt;
                    while *next_tick_timer <= 0.0 && *ticks_remaining > 0 {
                        // Apply one tick of DOT damage
                        critter.hp -= *damage;
                        damage_events.send(DamageEvent {
                            target: entity,
                            amount: *damage,
                        });

                        *next_tick_timer += *tick_interval;
                        *ticks_remaining -= 1;
                    }
                    if *ticks_remaining == 0 {
                        expired.push(i);
                    }
                }
                AppliedEffect::Splash { .. } => {
                    // Splash is instant — applied on projectile hit, not ticked.
                    // Remove immediately (it was a one-shot effect).
                    expired.push(i);
                }
                AppliedEffect::Motivate { timer, .. } => {
                    // Motivate is on towers, but could be on critters too.
                    // For critters, tick the timer and remove when expired.
                    *timer -= dt;
                    if *timer <= 0.0 {
                        expired.push(i);
                    }
                }
            }
        }

        // Remove expired effects (reverse order to preserve indices)
        for &i in expired.iter().rev() {
            active_effects.effects.remove(i);
        }

        // Check for death from DOT
        if critter.hp <= 0 {
            kill_events.send(KillEvent {
                target: entity,
                reward: critter.reward,
                steal_amount: 0, // DOT kills don't steal — critter didn't reach end
            });
            commands.entity(entity).despawn_recursive();
        }
    }

    // --- Tick effects on towers ---
    for tower_stats in tower_query.iter_mut() {
        // Tower effects (Motivate) are handled by the motivate start/stop logic.
        // For now, we just tick timers on Motivate effects.
        // Motivate applies a permanent boost that is removed when the effect expires.
        // This is handled in the 'start' phase (when the buff is applied) and
        // 'stop' phase (when it wears off). The effect system only handles ticking.
        //
        // TODO: implement Motivate start/stop properly
        let _ = tower_stats;
    }
}

/// Compute the effective speed of a critter with active slow effects.
pub fn effective_speed(base_speed: f32, active_effects: &ActiveEffects) -> f32 {
    let mut speed = base_speed;
    for effect in &active_effects.effects {
        if let AppliedEffect::Slow { multiplier, .. } = effect {
            speed *= *multiplier;
        }
    }
    speed
}
