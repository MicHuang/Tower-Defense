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
