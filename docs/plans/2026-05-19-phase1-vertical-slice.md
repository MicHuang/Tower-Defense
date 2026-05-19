# Phase 1: Vertical Slice — Implementation Plan

> **For Hermes:** Execute task-by-task. Tasks are sequential — each builds on the previous.

**Goal:** Playable single wave — map renders, BottleTower can be placed with visible projectile, critters walk the path, projectile hits critter, critter dies → coins/life update. HUD shows state. WASM build works.

**Depends on:** Phase 0 complete (FlatGrid, GameState, Player, WaveState, configs, `run_game()`)

**Location:** `~/Playground/Github/tower-defense-rs/`

**Reference design doc:** `~/Playground/Github/Tower-Defense/docs/specs/2026-05-19-rust-bevy-rewrite-design.md`

---

## Task 1: Create event types + SpatialGrid

**Files:**
- Create: `src/events/fire_event.rs`
- Create: `src/events/damage_event.rs`
- Create: `src/events/kill_event.rs`
- Create: `src/resources/spatial_grid.rs`

**Events:**

```rust
// fire_event.rs
use bevy::prelude::*;
use crate::components::effect::AppliedEffect;
use crate::components::tower::TowerType;

pub struct FireEvent {
    pub shooter: Entity,
    pub target: Entity,
    pub tower_type: TowerType,
    pub damage: f32,
    pub effect: Option<AppliedEffect>,
}
```

```rust
// damage_event.rs
use bevy::prelude::*;

pub struct DamageEvent {
    pub target: Entity,
    pub amount: i32,
}
```

```rust
// kill_event.rs
use bevy::prelude::*;

pub struct KillEvent {
    pub target: Entity,
    pub reward: i32,
    pub steal_amount: i32,
}
```

**SpatialGrid:**

```rust
// spatial_grid.rs is a new resource file
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct SpatialGrid {
    pub cell_size: f32,  // = 1.0 (matching grid unit)
    pub buckets: HashMap<UVec2, Vec<Entity>>,
}
```

## Task 2: Critter component + CritterBundle + Wave spawning

**Files:**
- Create: `src/components/critter.rs`
- Create: `src/components/tower.rs` (TowerType enum + Tower component)
- Create: `src/components/effect.rs` (AppliedEffect enum)
- Create: `src/components/grid_tower.rs`
- Create: `src/components/projectile.rs`

**Critter component:**

```rust
use bevy::prelude::*;

#[derive(Component)]
pub struct Critter {
    pub hp: i32,
    pub total_hp: i32,
    pub base_speed: f32,
    pub reward: i32,
    pub steal_amount: i32,
}

#[derive(Component)]
pub struct PathProgress {
    pub path_index: usize,
    pub segment_index: usize,
    pub distance: f32,
}
```

**TowerType + Tower (in tower.rs):**

```rust
use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum TowerType {
    Bottle, Fire, Ice, Splash, Sun, Moon, Energy,
}

#[derive(Component)]
pub struct Tower {
    pub tower_type: TowerType,
    pub level: u32,
}

#[derive(Component)]
pub struct TowerStats {
    pub range: f32,
    pub damage: f32,
    pub attack_cooldown: f32,
    pub current_cooldown: f32,
}

#[derive(Component)]
pub struct TargetSelector {
    pub strategy: TargetingStrategy,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TargetingStrategy {
    ClosestToEnd,
}

#[derive(Component)]
pub struct GridTower {
    pub grid_pos: UVec2,
}
```

**Effect (in effect.rs):**

```rust
use bevy::prelude::*;

#[derive(Component)]
pub struct ActiveEffects {
    pub effects: Vec<AppliedEffect>,
}

#[derive(Debug, Clone)]
pub enum AppliedEffect {
    Slow { multiplier: f32, duration: f32, timer: f32 },
    Dot { damage: i32, tick_interval: f32, ticks_remaining: u32, next_tick_timer: f32 },
    Splash { damage: i32, range: f32 },
    Motivate { enhance_rate: f32, duration: f32, timer: f32 },
}
```

**Projectile (in projectile.rs):**

```rust
use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile {
    pub target: Entity,
    pub speed: f32,
    pub damage: i32,
    pub effect: Option<AppliedEffect>,
    pub tower_type: TowerType,
    pub origin: Vec2,
}
```

**Wave config struct (already exists in data/wave_config.rs — reference it when spawning).**

## Task 3: WaveSystem + MovementSystem

**Files:**
- Create: `src/systems/wave.rs`
- Create: `src/systems/movement.rs`
- Create: `src/systems/combat.rs`

**WaveSystem:** Spawns critters at config-defined intervals. Transitions wave phases: Waiting → Producing → Produced.

**MovementSystem:** Advances critters along path segments. On endpoint reached, despawn critter and fire endpoint-reached logic.

**CombatSystem:** For each tower, decrement cooldown. If cooldown <= 0, query SpatialGrid for critters in range, apply targeting strategy, fire FireEvent.

## Task 4: ProjectileSpawner + ProjectileMove + Damage

**Files:**
- Create: `src/systems/projectile_spawner.rs`
- Create: `src/systems/projectile.rs`
- Create: `src/systems/scoring.rs`

**ProjectileSpawner:** Listens for FireEvent, spawns Projectile entity.

**ProjectileSystem:** Moves projectile toward target. On hit, apply damage, fire DamageEvent. If hp <= 0 after damage, fire KillEvent.

**ScoringSystem:** Listens for KillEvent → earn coins/score. Listens for endpoint reached → alter life.

## Task 5: Tower placement + input

**Files:**
- Create: `src/systems/input.rs`
- Create: `src/systems/tower_ops.rs`
- Modify: `src/lib.rs` (add click → grid placement)

**InputSystem:** Convert mouse/touch click to grid coordinates.

**TowerOpsSystem:** Place tower on empty cell. Validate: cell is Empty, player has coins. Deduct coins, spawn Tower entity, update tile to Tower(Some(entity)).

## Task 6: HUD + GameState transitions

**Files:**
- Create: `src/ui/hud.rs`
- Create: `src/ui/menu.rs`
- Modify: `src/lib.rs` (add UI systems, OnEnter/OnExit for GameState)

**HUD:** Display coins, life, wave counter, score.

**Menu:** Title screen with "Play" button.

**GameState transitions:** OnEnter(Playing) → setup game. OnExit(Playing) → cleanup. GameOver → show message → back to Menu.

## Task 7: Wire everything in lib.rs + WASM test

**Modify:** `src/lib.rs`

Add all systems to the Bevy app with proper GameSet ordering. Register events. Initialize resources. Verify `cargo test`, `cargo check`, and `cargo check --lib --target wasm32-unknown-unknown` all pass.
