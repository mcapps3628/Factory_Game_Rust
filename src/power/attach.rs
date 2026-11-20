use bevy::prelude::*;
use super::def::PowerNode;
use crate::tile::{Tile, TileType};

// Positive = produces, negative = consumes
fn power_of(tile_type: TileType) -> Option<f32> {
    match tile_type {
        TileType::BasicGenerator => Some(50.0),
        TileType::Furnace => Some(-10.0),
        _ => None,
    }
}

pub fn attach_power(mut commands: Commands, new_tiles: Query<(Entity, &Tile), Added<Tile>>) {
    for (entity, tile) in &new_tiles {
        if let Some(power) = power_of(tile.tile_type) {
            commands.entity(entity).insert(PowerNode { power, ..default() });
        }
    }
}