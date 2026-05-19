# Phase 0: Scaffold — Implementation Plan

> **For Hermes:** Use `subagent-driven-development` skill to implement this plan task-by-task.

**Goal:** Create a compilable Rust + Bevy project with empty grid rendering, TOML config for BottleTower and one map, and a working WASM build.

**Architecture:** Greenfield project alongside the existing Java `Tower-Defense/`. The Bevy app lives in `lib.rs::run_game()` from day one. `main.rs` (desktop) and `wasm.rs` (lib module, WASM) both call it. No `main.rs` rewrite needed later.

**Location:** `~/Playground/Github/tower-defense-rs/` (new directory)

**Tech Stack:** Rust, Bevy 0.15+, serde (TOML, JSON), wasm-bindgen, trunk

**Reference design doc:** `~/Playground/Github/Tower-Defense/docs/specs/2026-05-19-rust-bevy-rewrite-design.md`

---

## Task 1: Initialize Cargo project

**Objective:** Create the project skeleton with `cargo init --lib` and set up the directory structure. Add `.gitignore`, `index.html` for trunk.

**Files:**
- Create: `~/Playground/Github/tower-defense-rs/Cargo.toml`
- Create: `~/Playground/Github/tower-defense-rs/.gitignore`
- Create: `~/Playground/Github/tower-defense-rs/src/lib.rs`
- Create: `~/Playground/Github/tower-defense-rs/src/main.rs`
- Create all module stub files under `src/`

**Step 1: Create project**

```bash
cd ~/Playground/Github
cargo init --lib tower-defense-rs
cd tower-defense-rs
```

**Step 2: Create module directory structure**

```bash
mkdir -p src/components src/events src/systems src/resources src/save src/ui src/data
mkdir -p assets/sprites assets/maps assets/config
```

**Step 3: Write `Cargo.toml`**

```toml
[package]
name = "tower-defense"
version = "0.1.0"
edition = "2021"
description = "Tower Defense game — Rust + Bevy rewrite"

[lib]
crate-type = ["lib", "cdylib", "staticlib"]

[[bin]]
name = "tower-defense"
path = "src/main.rs"

[dependencies]
bevy = "0.15"
serde = { version = "1", features = ["derive"] }
toml = "0.8"
serde_json = "1"

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"

[profile.release]
opt-level = 3
```

**Key notes:**
- No `web-time` — Bevy 0.15 provides its own `web_time` re-export (v1.0). Adding `web-time = "0.2"` would conflict.
- `wasm-bindgen` only for `cfg(target_arch = "wasm32")` — not needed on desktop.
- `serde_json` is a placeholder for Phase 2 save/load; fine to keep.

**Step 4: Write `.gitignore`**

```
target/
dist/
*.swp
.DS_Store
```

**Step 5: Write module stub files**

- `src/components/mod.rs` — `pub mod tower; pub mod grid_tower; pub mod critter; pub mod projectile; pub mod effect; pub mod map; pub mod player;`
- `src/events/mod.rs` — `pub mod fire_event; pub mod damage_event; pub mod kill_event;`
- `src/systems/mod.rs` — `pub mod sets; pub mod wave; pub mod movement; pub mod combat; pub mod projectile_spawner; pub mod projectile; pub mod effect; pub mod scoring; pub mod tower_ops; pub mod input; pub mod ui;`
- `src/resources/mod.rs` — `pub mod grid_map; pub mod wave_manager; pub mod asset_loader; pub mod game_state;`
- `src/save/mod.rs` — `pub mod game_save; pub mod save_system; pub mod high_scores;`
- `src/ui/mod.rs` — `pub mod hud; pub mod inspector; pub mod build_palette; pub mod strategy_selector; pub mod menu; pub mod map_editor;`
- `src/data/mod.rs` — `pub mod tower_config; pub mod wave_config; pub mod map_config;`

Write each stub file with just the `pub mod` declaration. For `src/systems/sets.rs`, write the full GameSet enum:

```rust
use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSet {
    Wave,
    Movement,
    Combat,
    ProjectileSpawn,
    ProjectileMove,
    Effects,
    Scoring,
}
```

Write `src/lib.rs` with:

```rust
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
```

**Step 6: Verify compilation**

```bash
cargo check
```

Expected: Compilation succeeds (dead_code warnings are fine).

**Step 7: First commit**

```bash
git add -A
git commit -m "chore: scaffold project structure"
```

---

## Task 2: Add Bevy app bootstrap with `run_game()` in lib

**Objective:** Create `lib.rs::run_game()` with a minimal Bevy app. `main.rs` calls it. This establishes the pattern that both desktop and WASM share.

**Files:**
- Create: `src/main.rs`
- Modify: `src/lib.rs`

**Step 1: Write `src/lib.rs` with `run_game()`**

```rust
use bevy::prelude::*;

pub mod components;
pub mod data;
pub mod events;
pub mod resources;
pub mod save;
pub mod systems;
pub mod ui;

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
```

**Step 2: Write `src/main.rs`**

```rust
fn main() {
    tower_defense::run_game();
}
```

`main.rs` is a thin wrapper — one line. No app logic lives here.

**Step 3: Verify compilation**

```bash
cargo check
```

Expected: Compilation succeeds.

**Step 4: Copy existing images from Java project (assets for later phases)**

```bash
cp -r ~/Playground/Github/Tower-Defense/images/*.png assets/sprites/
ls assets/sprites/ | head -10
```

Expected: PNG files listed. (Not used for rendering in Phase 0, but available for Phase 1.)

**Step 5: Commit**

```bash
git add -A
git commit -m "feat: add run_game() in lib, main.rs as thin wrapper"
```

---

## Task 3: Port BottleTower config to TOML + add parsing test

**Objective:** Create `towers.toml` with BottleTower stats. Add serde deserialization and a unit test.

**Files:**
- Create: `assets/config/towers.toml`
- Create: `src/data/tower_config.rs`

**Step 1: Create `assets/config/towers.toml`**

```toml
[tower.bottle]
name = "Bottle Tower"
damage = 15.0
range = 3.0
attack_cooldown = 0.8
projectile_speed = 200.0
upgrade_prices = [100, 200, 400]
sell_ratios = [0.5, 0.5, 0.5]
cell_image = "tower_bottle.png"
```

Note: `damage` is `f32` (not `i32`) because Bevy uses f32 for game stats. Match this in the struct.

**Step 2: Create `src/data/tower_config.rs`**

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct TowerConfig {
    pub name: String,
    pub damage: f32,
    pub range: f32,
    pub attack_cooldown: f32,
    pub projectile_speed: f32,
    pub upgrade_prices: Vec<u32>,
    pub sell_ratios: Vec<f32>,
    pub cell_image: String,
    #[serde(default)]
    pub burn_damage: Option<f32>,
    #[serde(default)]
    pub burn_interval: Option<f32>,
    #[serde(default)]
    pub burn_ticks: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct TowerConfigs {
    pub tower: std::collections::HashMap<String, TowerConfig>,
}

impl TowerConfigs {
    pub fn load() -> Result<Self, toml::de::Error> {
        // include_str! resolves relative to this source file: src/data/
        let content = include_str!("../../assets/config/towers.toml");
        toml::from_str(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_bottle_tower() {
        let configs = TowerConfigs::load().expect("Failed to load tower config");
        let bottle = configs.tower.get("bottle").expect("BottleTower not found");
        assert_eq!(bottle.name, "Bottle Tower");
        assert!(bottle.damage > 0.0);
        assert_eq!(bottle.upgrade_prices.len(), 3);
    }
}
```

**Step 3: Run tests**

```bash
cargo test test_load_bottle_tower -- --nocapture
```

Expected: `test result: ok. 1 passed`

**Step 4: Verify compilation**

```bash
cargo check
```

Expected: Compilation succeeds.

**Step 5: Commit**

```bash
git add -A
git commit -m "feat: add BottleTower TOML config + deserialization with test"
```

---

## Task 4: Port demo map to TOML + add parsing test

**Objective:** Create a TOML map file from the Java XML demo map, with deserialization and a test.

**Files:**
- Create: `assets/maps/demo_map.toml`
- Create: `src/data/map_config.rs`
- Create: `src/components/map.rs`

**Step 1: Create `src/components/map.rs`**

```rust
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TileType {
    Empty,
    Road,
    RoadStart,
    RoadEnd,
    Scenery,
}

#[derive(Debug, Clone)]
pub enum Tile {
    Empty,
    Road(TileType),
    Tower(Option<Entity>),
    Scenery,
}
```

**Step 2: Create `assets/maps/demo_map.toml`**

An 8×6 grid with a path from (0,2) → (3,2) → (3,4) → (7,4):

```toml
width = 8
height = 6
start_points = [{ x = 0, y = 2 }]
end_points = [{ x = 7, y = 4 }]
paths = [
    [{ x = 0, y = 2 }, { x = 3, y = 2 }, { x = 3, y = 4 }, { x = 7, y = 4 }]
]
tiles = [
    "road_start", "empty",     "empty",     "road", "road", "road", "road", "road",
    "empty",      "empty",     "empty",     "road", "empty", "empty", "empty", "empty",
    "empty",      "empty",     "empty",     "road", "empty", "empty", "empty", "empty",
    "empty",      "empty",     "empty",     "road", "empty", "empty", "empty", "road_end",
    "empty",      "empty",     "empty",     "empty", "empty", "empty", "empty", "empty",
    "empty",      "empty",     "empty",     "empty", "empty", "empty", "empty", "empty",
]
```

Note: `tiles` length must equal `width × height = 48`. Row-major: first 8 entries = row 0, next 8 = row 1, etc.

**Step 3: Create `src/data/map_config.rs`**

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct UVec2Def {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Deserialize)]
pub struct MapConfig {
    pub width: u32,
    pub height: u32,
    pub start_points: Vec<UVec2Def>,
    pub end_points: Vec<UVec2Def>,
    pub paths: Vec<Vec<UVec2Def>>,
    pub tiles: Vec<String>,
}

impl MapConfig {
    pub fn load() -> Result<Self, toml::de::Error> {
        let content = include_str!("../../assets/maps/demo_map.toml");
        toml::from_str(content)
    }

    pub fn tile_count(&self) -> usize {
        (self.width * self.height) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_demo_map() {
        let config = MapConfig::load().expect("Failed to load map");
        assert_eq!(config.width, 8);
        assert_eq!(config.height, 6);
        assert_eq!(config.tiles.len() as u32, config.width * config.height);
        assert_eq!(config.start_points.len(), 1);
        assert_eq!(config.end_points.len(), 1);
        assert_eq!(config.paths.len(), 1);
    }
}
```

**Step 4: Run tests**

```bash
cargo test test_load_demo_map -- --nocapture
```

Expected: `test result: ok. 1 passed`

**Step 5: Verify all tests**

```bash
cargo test
```

Expected: `2 passed`

**Step 6: Commit**

```bash
git add -A
git commit -m "feat: add demo map TOML + deserialization with test"
```

---

## Task 5: Render the grid from TOML map data

**Objective:** Load the TOML map in `setup()`, build a `FlatGrid` resource, and spawn colored rectangles for each tile.

**Files:**
- Create: `src/resources/grid_map.rs`
- Modify: `src/lib.rs` (add grid rendering in `setup()`)

**Step 1: Create `src/resources/grid_map.rs`**

```rust
use bevy::prelude::*;
use crate::components::map::Tile;
use crate::data::map_config::MapConfig;

#[derive(Resource)]
pub struct FlatGrid {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Tile>,
    pub paths: Vec<Vec<UVec2>>,
    pub start_points: Vec<UVec2>,
    pub end_points: Vec<UVec2>,
}

impl FlatGrid {
    pub fn from_config(config: &MapConfig) -> Self {
        let mut tiles = Vec::with_capacity(config.tile_count());
        for tile_str in &config.tiles {
            let tile = match tile_str.as_str() {
                "road_start" => Tile::Road(crate::components::map::TileType::RoadStart),
                "road_end" => Tile::Road(crate::components::map::TileType::RoadEnd),
                "road" => Tile::Road(crate::components::map::TileType::Road),
                "scenery" => Tile::Scenery,
                _ => Tile::Empty,
            };
            tiles.push(tile);
        }

        let to_uv = |def: &crate::data::map_config::UVec2Def| UVec2::new(def.x, def.y);
        let paths = config.paths.iter()
            .map(|p| p.iter().map(to_uv).collect())
            .collect();
        let start_points = config.start_points.iter().map(to_uv).collect();
        let end_points = config.end_points.iter().map(to_uv).collect();

        FlatGrid { width: config.width, height: config.height, tiles, paths, start_points, end_points }
    }

    pub fn index(&self, x: u32, y: u32) -> usize {
        assert!(x < self.width && y < self.height, "grid index out of bounds");
        (y * self.width + x) as usize
    }

    pub fn tile_at(&self, x: u32, y: u32) -> &Tile {
        &self.tiles[self.index(x, y)]
    }
}
```

**Step 2: Modify `src/lib.rs` to render the grid**

Replace the `setup` function:

```rust
use bevy::prelude::*;
use crate::data::map_config::MapConfig;
use crate::resources::grid_map::FlatGrid;
use crate::components::map::Tile;

pub fn run_game() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let map_config = MapConfig::load().expect("Failed to load map config");
    let grid = FlatGrid::from_config(&map_config);

    // Cell size in pixels. On mobile this will be adapted in Phase 1.
    let cell_size = 64.0;
    let total_w = grid.width as f32 * cell_size;
    let total_h = grid.height as f32 * cell_size;
    let offset_x = -total_w / 2.0 + cell_size / 2.0;
    let offset_y = -total_h / 2.0 + cell_size / 2.0;

    // Spawn one coloured rectangle per tile
    for y in 0..grid.height {
        for x in 0..grid.width {
            let tile = grid.tile_at(x, y);
            let color = match tile {
                Tile::Road(_) => Color::GRAY,
                Tile::Tower(_) => Color::BLUE,
                Tile::Scenery => Color::GREEN,
                Tile::Empty => Color::DARK_GRAY,
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
```

Make sure `lib.rs` has the right imports at the top:

```rust
use bevy::prelude::*;
pub mod components;
// ... rest of pub mod declarations ...
```

Note: The top-level `use bevy::prelude::*` is necessary for `lib.rs::setup()` to access Bevy types.

**Step 3: Verify compilation**

```bash
cargo check
```

Expected: Compilation succeeds.

**Step 4: Commit**

```bash
git add -A
git commit -m "feat: render grid from TOML map data with FlatGrid resource"
```

---

## Task 6: WASM build test

**Objective:** Verify the project compiles to WASM target using `trunk`.

**Files:**
- Create: `index.html` (trunk entry point)

**Step 1: Check if `trunk` is installed**

```bash
which trunk || cargo install trunk
```

**Step 2: Create `index.html` at project root**

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Tower Defense</title>
    <style>
        html, body { margin: 0; padding: 0; width: 100%; height: 100%; overflow: hidden; background: #1a1a1a; }
    </style>
</head>
<body>
    <script type="module">
        import init from './target/wasm32-unknown-unknown/release/tower-defense.js';
        init().catch(console.error);
    </script>
</body>
</html>
```

**Step 3: Create the WASM module in `lib.rs`**

The WASM entry is already set up. In lib.rs, the wasm module is conditionally compiled:

```rust
#[cfg(target_arch = "wasm32")]
pub mod wasm;
```

Create `src/wasm.rs` (this is a lib module, not a binary):

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start() {
    crate::run_game();
}
```

`wasm_bindgen` is only available on `cfg(target_arch = "wasm32")`, and the module is conditionally compiled, so it won't cause errors on desktop builds.

**Step 4: Verify WASM compilation with `--lib`**

```bash
cargo check --lib --target wasm32-unknown-unknown
```

Note: `--lib` is required because `main.rs` (a binary) cannot compile for WASM. The `cdylib` crate type in Cargo.toml enables WASM export.

Expected: Compilation succeeds.

**Step 5: Commit**

```bash
git add -A
git commit -m "feat: add WASM entry point + index.html for trunk"
```

---

## Task 7: Add GameState (Bevy State), Player, WaveState, WaveConfig

**Objective:** Add the game state management resources and wave config data structures.

**Files:**
- Create: `src/resources/game_state.rs`
- Create: `src/components/player.rs`
- Create: `assets/config/waves.toml`
- Create: `src/data/wave_config.rs`
- Modify: `src/lib.rs` (add `add_state` + resources)

**Step 1: Create `src/resources/game_state.rs`**

```rust
use bevy::prelude::*;

/// Bevy State for game screen transitions.
/// Must derive States, Clone, Copy, PartialEq, Eq, Hash for Bevy 0.15.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States, Default)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
    Victory,
    Editing,
}
```

Note: `#[default]` on `Menu` ensures the game starts in the menu state.

**Step 2: Create `src/components/player.rs`**

```rust
use bevy::prelude::*;

#[derive(Resource)]
pub struct Player {
    pub coins: u32,
    pub life: u32,
    pub score: u32,
}

impl Default for Player {
    fn default() -> Self {
        Player { coins: 500, life: 20, score: 0 }
    }
}

#[derive(Resource)]
pub struct WaveState {
    pub current_index: usize,
    pub phase: WavePhase,
    pub spawn_timer: f32,
    pub critters_spawned: usize,
}

impl Default for WaveState {
    fn default() -> Self {
        WaveState {
            current_index: 0,
            phase: WavePhase::Waiting,
            spawn_timer: 0.0,
            critters_spawned: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum WavePhase {
    Waiting,
    Producing,
    Produced,
}
```

**Step 3: Create `assets/config/waves.toml`**

```toml
[[wave]]
spawn_count = 5
spawn_interval = 1.0
hitpoints = 50
speed = 60.0
reward = 10
steal_amount = 1

[[wave]]
spawn_count = 8
spawn_interval = 0.8
hitpoints = 75
speed = 65.0
reward = 15
steal_amount = 1

[[wave]]
spawn_count = 12
spawn_interval = 0.6
hitpoints = 100
speed = 70.0
reward = 20
steal_amount = 2
```

**Step 4: Create `src/data/wave_config.rs`**

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct WaveDef {
    pub spawn_count: u32,
    pub spawn_interval: f32,
    pub hitpoints: i32,
    pub speed: f32,
    pub reward: i32,
    pub steal_amount: i32,
}

#[derive(Debug, Deserialize)]
pub struct WaveConfigs {
    pub wave: Vec<WaveDef>,
}

impl WaveConfigs {
    pub fn load() -> Result<Self, toml::de::Error> {
        let content = include_str!("../../assets/config/waves.toml");
        toml::from_str(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_wave_config() {
        let configs = WaveConfigs::load().expect("Failed to load wave config");
        assert!(!configs.wave.is_empty());
        assert!(configs.wave[0].spawn_count > 0);
    }
}
```

**Step 5: Insert resources and state into `lib.rs::run_game()`**

Modify the `run_game()` function:

```rust
pub fn run_game() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()  // Register GameState as a Bevy State
        .insert_resource(Player::default())
        .insert_resource(WaveState::default())
        .add_systems(Startup, setup)
        .run();
}
```

Need to add the import:

```rust
use crate::resources::game_state::GameState;
use crate::components::player::{Player, WaveState};
```

These go at the top of `lib.rs`, after the existing `use bevy::prelude::*;`.

**Step 6: Run all tests**

```bash
cargo test
```

Expected: `3 passed` (tower config + map config + wave config).

**Step 7: Verify compilation**

```bash
cargo check
cargo check --lib --target wasm32-unknown-unknown
```

Expected: Both succeed.

**Step 8: Commit**

```bash
git add -A
git commit -m "feat: add GameState, Player/WaveState resources, wave config"
```

---

## Verification Checklist

After all Phase 0 tasks are complete:

- [ ] `cargo check` passes with zero errors
- [ ] `cargo test` passes (3+ tests: config parsing, map parsing, wave parsing)
- [ ] `cargo check --lib --target wasm32-unknown-unknown` passes
- [ ] `.gitignore` ignores `target/`, `dist/`, `*.swp`, `.DS_Store`
- [ ] Project structure has all module directories under `src/`
- [ ] `lib.rs` has `pub fn run_game()` called by both `main.rs` and `wasm.rs`
- [ ] `assets/config/towers.toml` contains BottleTower config
- [ ] `assets/config/waves.toml` contains 3 wave definitions
- [ ] `assets/maps/demo_map.toml` contains an 8×6 grid with a path
- [ ] `GameState` derives `States`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`, `Default`
- [ ] `run_game()` calls `init_state::<GameState>()`
- [ ] `index.html` exists for trunk
- [ ] `wasm.rs` calls `run_game()` not `crate::main::main()`
- [ ] Player defaults: 500 coins, 20 life, 0 score

**Phase 0 complete.** Ready for Phase 1: Vertical Slice.
