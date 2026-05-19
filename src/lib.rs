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
