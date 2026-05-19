use bevy::prelude::*;
use crate::events::kill_event::KillEvent;
use crate::components::player::Player;

pub fn scoring_system(
    mut kill_events: EventReader<KillEvent>,
    mut player: ResMut<Player>,
) {
    for event in kill_events.read() {
        player.earn_coins(event.reward);
        player.score += 1; // +1 score per kill
    }
}
