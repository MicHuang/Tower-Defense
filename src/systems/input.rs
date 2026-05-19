use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::resources::grid_map::FlatGrid;
use crate::components::map::Tile;
use crate::components::tower::{Tower, TowerType, TowerStats, TargetSelector, TargetingStrategy};
use crate::components::grid_tower::GridTower;
use crate::components::player::Player;

pub fn input_system(
    mouse_button: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut grid: ResMut<FlatGrid>,
    mut commands: Commands,
    mut player: ResMut<Player>,
) {
    // Only handle left click
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    // Get window and camera
    let window = if let Ok(w) = window_query.get_single() {
        w
    } else {
        return;
    };
    let (camera, camera_transform) = if let Ok(c) = camera_query.get_single() {
        c
    } else {
        return;
    };

    // Get mouse position in world coordinates
    let cursor_pos = match window.cursor_position() {
        Some(pos) => pos,
        None => return,
    };
    let world_pos = match camera.viewport_to_world_2d(camera_transform, cursor_pos) {
        Ok(pos) => pos,
        Err(_) => return,
    };

    // Convert to grid coordinates
    let cell_size = 64.0;
    // The grid is centered at (0,0), so we need to reverse the offset from lib.rs setup()
    let total_w = grid.width as f32 * cell_size;
    let total_h = grid.height as f32 * cell_size;
    let offset_x = -total_w / 2.0 + cell_size / 2.0;
    let offset_y = -total_h / 2.0 + cell_size / 2.0;

    let grid_x = ((world_pos.x - offset_x) / cell_size + 0.5).floor() as i32;
    let grid_y = ((world_pos.y - offset_y) / cell_size + 0.5).floor() as i32;

    // Validate grid bounds
    if grid_x < 0 || grid_x >= grid.width as i32 || grid_y < 0 || grid_y >= grid.height as i32 {
        return;
    }
    let gx = grid_x as u32;
    let gy = grid_y as u32;

    // Check if cell is empty
    let idx = grid.index(gx, gy);
    let is_empty = matches!(grid.tiles[idx], Tile::Empty);
    if !is_empty {
        return;
    }

    // Check if player can afford a BottleTower
    let bottle_price = 100; // Will be config-driven later
    if !player.spend_coins(bottle_price) {
        return;
    }

    // Place the tower
    let tower_entity = commands.spawn((
        Tower {
            tower_type: TowerType::Bottle,
            level: 1,
        },
        TowerStats {
            range: 3.0,
            damage: 15.0,
            attack_cooldown: 0.8,
            current_cooldown: 0.0,
        },
        TargetSelector {
            strategy: TargetingStrategy::ClosestToEnd,
        },
        GridTower {
            grid_pos: UVec2::new(gx, gy),
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.0, 0.0, 1.0),
                custom_size: Some(Vec2::new(cell_size - 2.0, cell_size - 2.0)),
                ..default()
            },
            transform: Transform::from_xyz(
                offset_x + gx as f32 * cell_size,
                offset_y + gy as f32 * cell_size,
                1.0,
            ),
            ..default()
        },
    )).id();

    // Update grid tile
    grid.tiles[idx] = Tile::Tower(Some(tower_entity));
}
