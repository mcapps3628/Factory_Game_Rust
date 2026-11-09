use bevy::gltf::gltf::mesh::Mode;
use bevy::prelude::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::economy::Money;
use crate::mode::PlayerMode;
use crate::tile::placement::Tile;
use crate::tile::TileType;
use super::grid::{cursor_grid_pos, Grid};

#[derive(Debug)]
pub enum RemoveError {
    OutOfBounds,
    Empty,
}

pub fn remove_tile(
    pos: IVec2,
    grid: &mut Grid,
    commands: &mut Commands,
    // Pass queries and resources by mutable reference
    query: &mut Query<&mut Tile>,
    money: &mut ResMut<Money>,
) -> Result<(), RemoveError> {

    if !Grid::in_bounds(pos) {
        return Err(RemoveError::OutOfBounds);
    }

    let Some(entity) = grid.get(pos) else {
        return Err(RemoveError::Empty);
    };
    
    // Use get_mut on the borrowed query
    if let Ok(mut tile) = query.get_mut(entity) {
        let tile_def = tile.tile_type.def();
        money.0 += tile_def.cost;
    }

    commands.entity(entity).despawn();
    grid.clear(pos);
    Ok(())
}

pub fn click_to_remove(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut grid: ResMut<Grid>,
    mut money: ResMut<Money>,
    mut query: Query<&mut Tile>,
    mode: Res<State<PlayerMode>>,
) {
    if !mouse.just_pressed(MouseButton::Right) || *mode.get() != PlayerMode::Build {
        return;
    }

    let (camera, cam_t) = *camera;
    let Some(pos) = cursor_grid_pos(*window, camera, cam_t) else {
        return;
    };

    if let Err(e) = remove_tile(
        pos,
        &mut grid,
        &mut commands,
        &mut query,
        &mut money,
    ) {
        info!("Cannot remove tile: {:?}", e);
    }
}