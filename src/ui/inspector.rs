use bevy::prelude::*;

use crate::components::critter::Critter;
use crate::components::effect::ActiveEffects;
use crate::components::grid_tower::GridTower;
use crate::components::tower::{Tower, TowerStats, TowerType};

#[derive(Component)]
pub struct InspectorPanel;

#[derive(Resource, Default)]
pub struct InspectorTarget {
    pub entity: Option<Entity>,
    pub info: String,
}

/// Click on a tower/critter to inspect it (Right-click shows info panel).
pub fn inspector_click(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    tower_query: Query<(Entity, &Tower, &TowerStats, &GridTower, &Transform)>,
    critter_query: Query<(Entity, &Critter, &ActiveEffects, &Transform)>,
    mut inspector: ResMut<InspectorTarget>,
) {
    if !mouse_button.just_pressed(MouseButton::Right) {
        return;
    }

    let window = windows.single();
    let (camera, camera_transform) = camera_query.single();

    let cursor_pos = window
        .cursor_position()
        .unwrap_or(Vec2::ZERO);

    // Convert screen coords → world coords
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else { return };

    let hit_radius = 48.0;

    // Check towers first
    for (entity, tower, stats, grid, transform) in tower_query.iter() {
        let dist = transform.translation.truncate().distance(world_pos);
        if dist < hit_radius {
            let type_name = format!("{:?}", tower.tower_type);
            let effects = match tower.tower_type {
                TowerType::Bottle => "None (pure damage)",
                TowerType::Fire => "Burn (30% DOT, 3 ticks)",
                TowerType::Ice => "Freeze (0.3x speed, 3s)",
                TowerType::Splash => "Splash (50% AOE dmg)",
                TowerType::Sun => "AOE all in range",
                TowerType::Moon => "Slow (0.5x speed, 2s)",
                TowerType::Energy => "Buff nearby towers",
            };
            inspector.info = format!(
                "{} Lv{}\nGrid: ({},{})\nDamage: {:.0}  Range: {:.0}\nCooldown: {:.1}s\nEffect: {}",
                type_name, tower.level, grid.grid_pos.x, grid.grid_pos.y,
                stats.damage, stats.range, stats.attack_cooldown, effects
            );
            inspector.entity = Some(entity);
            return;
        }
    }

    // Check critters
    for (entity, critter, effects, transform) in critter_query.iter() {
        let dist = transform.translation.truncate().distance(world_pos);
        if dist < hit_radius {
            let mut effect_list = String::new();
            for e in &effects.effects {
                match e {
                    crate::components::effect::AppliedEffect::Slow { multiplier, timer, .. } => {
                        effect_list.push_str(&format!("Slow {:.0}x {:.1}s ", multiplier, timer));
                    }
                    crate::components::effect::AppliedEffect::Dot { ticks_remaining, .. } => {
                        effect_list.push_str(&format!("Burn {}t ", ticks_remaining));
                    }
                    _ => {}
                }
            }
            if effect_list.is_empty() {
                effect_list = String::from("None");
            }
            inspector.info = format!(
                "Critter\nHP: {}/{}\nSpeed: {:.0}  Reward: {}\nSteal: {}\nEffects: {}",
                critter.hp, critter.total_hp, critter.base_speed,
                critter.reward, critter.steal_amount, effect_list
            );
            inspector.entity = Some(entity);
            return;
        }
    }

    // Clicked empty space — clear inspector
    inspector.entity = None;
    inspector.info.clear();
}

/// Draw the inspector panel as a floating UI element.
pub fn inspector_display(
    inspector: Res<InspectorTarget>,
    mut commands: Commands,
    existing_panels: Query<Entity, With<InspectorPanel>>,
) {
    // Clear old panels
    for entity in existing_panels.iter() {
        commands.entity(entity).despawn_recursive();
    }

    if inspector.entity.is_none() || inspector.info.is_empty() {
        return;
    }

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            width: Val::Px(220.0),
            padding: UiRect::all(Val::Px(12.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
        InspectorPanel,
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new(inspector.info.clone()),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgb(0.0, 1.0, 0.5)),
        ));
    });
}
