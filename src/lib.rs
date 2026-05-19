use bevy::prelude::*;

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
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
