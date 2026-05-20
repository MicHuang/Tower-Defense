use bevy::prelude::*;

use crate::components::map::Tile;
use crate::components::player::{Player, WaveState};
use crate::data::map_config::MapConfig;
use crate::data::wave_config::WaveConfigs;
use crate::events::damage_event::DamageEvent;
use crate::events::fire_event::FireEvent;
use crate::events::kill_event::KillEvent;
use crate::resources::game_state::GameState;
use crate::resources::grid_map::FlatGrid;
use crate::resources::spatial_grid::SpatialGrid;
use crate::save::save_system::save_load_input;
use crate::systems::combat::combat_system;
use crate::systems::effect::effect_system;
use crate::systems::input::input_system;
use crate::systems::movement::movement_system;
use crate::systems::projectile::{projectile_movement_system, projectile_target_update_system};
use crate::systems::projectile_spawner::projectile_spawner_system;
use crate::systems::scoring::scoring_system;
use crate::systems::wave::wave_system;
use crate::ui::hud::{despawn_hud, spawn_hud};
use crate::ui::inspector::{inspector_click, inspector_display, InspectorTarget};
use crate::ui::menu::{despawn_menu, play_button_clicked, spawn_menu};

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
        // Events
        .add_event::<FireEvent>()
        .add_event::<DamageEvent>()
        .add_event::<KillEvent>()
        // State
        .init_state::<GameState>()
        // Resources
        .insert_resource(Player::default())
        .insert_resource(WaveState::default())
        .insert_resource(InspectorTarget::default())
        .insert_resource(WaveConfigs::load().expect("Failed to load wave configs"))
        .insert_resource(SpatialGrid::default())
        // Menu systems
        .add_systems(OnEnter(GameState::Menu), spawn_menu)
        .add_systems(OnExit(GameState::Menu), despawn_menu)
        .add_systems(
            Update,
            play_button_clicked.run_if(in_state(GameState::Menu)),
        )
        // HUD systems
        .add_systems(OnEnter(GameState::Playing), spawn_hud)
        .add_systems(OnExit(GameState::Playing), despawn_hud)
        // Gameplay systems
        .add_systems(Update, (
            projectile_target_update_system,
            wave_system,
            movement_system,
            combat_system,
            projectile_spawner_system,
            projectile_movement_system,
            effect_system,
            scoring_system,
            input_system,
            save_load_input,
            inspector_click,
        ).run_if(in_state(GameState::Playing)))
        .add_systems(Update, inspector_display)
        // Startup
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

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

            commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(cell_size - 1.0, cell_size - 1.0)),
                    ..default()
                },
                Transform::from_xyz(
                    offset_x + x as f32 * cell_size,
                    offset_y + y as f32 * cell_size,
                    0.0,
                ),
            ));
        }
    }

    commands.insert_resource(grid);
}
