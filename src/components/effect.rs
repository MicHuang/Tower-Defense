use bevy::prelude::*;

#[derive(Component)]
pub struct ActiveEffects {
    pub effects: Vec<AppliedEffect>,
}

#[derive(Debug, Clone)]
pub enum AppliedEffect {
    Slow {
        multiplier: f32,
        duration: f32,
        timer: f32,
    },
    Dot {
        damage: i32,
        tick_interval: f32,
        ticks_remaining: u32,
        next_tick_timer: f32,
    },
    Splash {
        damage: i32,
        range: f32,
    },
    Motivate {
        enhance_rate: f32,
        duration: f32,
        timer: f32,
    },
}
