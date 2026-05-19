use bevy::prelude::*;

use crate::components::map::Tile;
use crate::components::player::{Player, WaveState};
use crate::data::map_config::MapConfig;
use crate::resources::game_state::GameState;
use crate::resources::grid_map::FlatGrid;

pub mod components;
pub mod data;
pub mod events;
pub mod resources;
pub mod save;
pub mod systems;
pub mod ui;

/// Re-export wasm module only when compiling for WASM
#[cfg(target_arch = "wasm32")]
pub mod wasm;

pub fn run_game() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .insert_resource(Player::default())
        .insert_resource(WaveState::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let map_config = MapConfig::load().expect("Failed to load map config");
    let grid = FlatGrid::from_config(&map_config);

    let cell_size = 64.0;
    let total_w = grid.width as f32 * cell_size;
    let total_h = grid.height as f32 * cell_size;
    let offset_x = -total_w / 2.0 + cell_size / 2.0;
    let offset_y = -total_h / 2.0 + cell_size / 2.0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let tile = grid.tile_at(x, y);
            let color = match tile {
                Tile::Road(_) => Color::srgb(0.5, 0.5, 0.5),
                Tile::Tower(_) => Color::srgb(0.0, 0.0, 1.0),
                Tile::Scenery => Color::srgb(0.0, 0.8, 0.0),
                Tile::Empty => Color::srgb(0.2, 0.2, 0.2),
            };

            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(cell_size - 1.0, cell_size - 1.0)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    offset_x + x as f32 * cell_size,
                    offset_y + y as f32 * cell_size,
                    0.0,
                ),
                ..default()
            });
        }
    }

    commands.insert_resource(grid);
}
