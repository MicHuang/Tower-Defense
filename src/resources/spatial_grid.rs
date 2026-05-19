use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct SpatialGrid {
    pub cell_size: f32,
    pub buckets: HashMap<UVec2, Vec<Entity>>,
}
