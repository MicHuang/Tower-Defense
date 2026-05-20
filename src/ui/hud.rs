use bevy::prelude::*;
use crate::components::player::{Player, WaveState};

#[derive(Component)]
pub struct HudRoot;

pub fn spawn_hud(mut commands: Commands) {
    // Root node — covers the top of the screen
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Vh(8.0),
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceEvenly,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::BLACK),
        HudRoot,
    )).with_children(|parent| {
        // Coins label
        parent.spawn((
            Text::new("Coins: 500"),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.84, 0.0)), // GOLD
        ));

        // Life label
        parent.spawn((
            Text::new("Life: 20"),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::srgb(0.0, 1.0, 0.0)), // GREEN
        ));

        // Wave label
        parent.spawn((
            Text::new("Wave: 0"),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 1.0)), // WHITE
        ));

        // Score label
        parent.spawn((
            Text::new("Score: 0"),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::srgb(0.0, 1.0, 1.0)), // CYAN
        ));

        // Next Wave button — for now a text label that will trigger wave start
        parent.spawn((
            Text::new("[Next Wave]"),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.65, 0.0)), // ORANGE
        ));
    });
}

pub fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<HudRoot>>) {
    for entity in hud_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update_hud(
    _player: Res<Player>,
    _wave_state: Res<WaveState>,
    _text_query: Query<&mut Text, With<HudRoot>>,
) {
    // This is simplified — in practice we'd tag each text node
    // For now, the HUD is re-spawned on state transitions
}
