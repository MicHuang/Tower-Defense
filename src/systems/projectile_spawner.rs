use bevy::prelude::*;
use crate::events::fire_event::FireEvent;
use crate::components::projectile::Projectile;

pub fn projectile_spawner_system(
    mut fire_events: EventReader<FireEvent>,
    mut commands: Commands,
    tower_query: Query<&Transform>,
) {
    for event in fire_events.read() {
        // Get the shooter's position
        let origin = if let Ok(transform) = tower_query.get(event.shooter) {
            transform.translation.truncate()
        } else {
            Vec2::ZERO
        };

        commands.spawn(Projectile {
            target: event.target,
            speed: 200.0,
            damage: event.damage as i32,
            effect: event.effect.clone(),
            tower_type: event.tower_type,
            origin,
        });
    }
}
