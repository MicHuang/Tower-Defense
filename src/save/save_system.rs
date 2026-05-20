use std::fs;
use std::path::PathBuf;

use bevy::prelude::*;

use crate::components::grid_tower::GridTower;
use crate::components::player::{Player, WaveState, WavePhase};
use crate::components::tower::{Tower, TowerStats};
use crate::save::game_save::{GameSave, PlayerSave, TowerSave, WaveSave};
use crate::resources::game_state::GameState;

/// Directory where save files are stored.
fn save_dir() -> PathBuf {
    PathBuf::from("saves")
}

/// Collect current ECS state into a serializable GameSave.
pub fn collect_save(
    player: &Player,
    wave_state: &WaveState,
    tower_query: &Query<(&Tower, &TowerStats, &GridTower)>,
) -> GameSave {
    let player_save = PlayerSave {
        coins: player.coins,
        life: player.life,
        score: player.score,
    };

    let wave_save = WaveSave {
        current_index: wave_state.current_index,
        phase: format!("{:?}", wave_state.phase),
    };

    let mut towers = Vec::new();
    for (tower, stats, grid_tower) in tower_query.iter() {
        towers.push(TowerSave {
            grid_x: grid_tower.grid_pos.x,
            grid_y: grid_tower.grid_pos.y,
            tower_type: tower.tower_type,
            level: tower.level,
            damage: stats.damage,
            range: stats.range,
            attack_cooldown: stats.attack_cooldown,
        });
    }

    GameSave::new(player_save, wave_save, towers)
}

/// Save game to JSON file.
pub fn save_game(save: &GameSave, slot: u32) -> Result<(), String> {
    let dir = save_dir();
    fs::create_dir_all(&dir).map_err(|e| format!("Failed to create save dir: {}", e))?;

    let path = dir.join(format!("slot_{}.json", slot));
    let json = serde_json::to_string_pretty(save)
        .map_err(|e| format!("Failed to serialize: {}", e))?;
    fs::write(&path, json)
        .map_err(|e| format!("Failed to write save file: {}", e))?;

    info!("Game saved to slot {} ({})", slot, path.display());
    Ok(())
}

/// Load game from JSON file.
pub fn load_game(slot: u32) -> Result<GameSave, String> {
    let path = save_dir().join(format!("slot_{}.json", slot));
    let json = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read save file: {}", e))?;
    let save: GameSave = serde_json::from_str(&json)
        .map_err(|e| format!("Failed to deserialize: {}", e))?;

    info!("Game loaded from slot {} ({})", slot, path.display());
    Ok(save)
}

/// Restore ECS state from a GameSave.
pub fn restore_save(
    _commands: Commands,
    save: GameSave,
    mut player: ResMut<Player>,
    mut wave_state: ResMut<WaveState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Restore player
    player.coins = save.player.coins;
    player.life = save.player.life;
    player.score = save.player.score;

    // Restore wave state
    wave_state.current_index = save.wave.current_index;
    wave_state.phase = match save.wave.phase.as_str() {
        "Producing" => WavePhase::Producing,
        "Produced" => WavePhase::Produced,
        _ => WavePhase::Waiting,
    };

    // TODO: restore towers — requires clearing existing towers and spawning new ones
    // For now, just log what towers would be restored
    info!("Restoring {} towers from save", save.towers.len());
    for t in &save.towers {
        info!(
            "  Tower {:?} Lv{} at ({}, {}): dmg={}, range={}",
            t.tower_type, t.level, t.grid_x, t.grid_y, t.damage, t.range
        );
    }

    // Transition to Playing state
    next_state.set(GameState::Playing);
}

/// Keyboard shortcuts: F5 = save, F9 = load (runs in Playing state)
pub fn save_load_input(
    keys: Res<ButtonInput<KeyCode>>,
    player: Res<Player>,
    wave_state: Res<WaveState>,
    tower_query: Query<(&Tower, &TowerStats, &GridTower)>,
) {
    if keys.just_pressed(KeyCode::F5) {
        let save = collect_save(&player, &wave_state, &tower_query);
        if let Err(e) = save_game(&save, 0) {
            warn!("Save failed: {}", e);
        }
    }

    if keys.just_pressed(KeyCode::F9) {
        match load_game(0) {
            Ok(_save) => {
                info!("Load requested — full restore not yet implemented (Phase 2)");
            }
            Err(e) => {
                warn!("Load failed: {}", e);
            }
        }
    }
}
