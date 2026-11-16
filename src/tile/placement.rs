use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::def::TileType;
use super::direction::{Direction};
use super::grid::{cursor_grid_pos, grid_to_world, Grid, TILE_SIZE};
use super::processing::{Inventory, Processor, Producer};
use crate::economy::Money;
use crate::tile::belt::Belt;
use crate::tile::port::{NeedsLink, Ports};
use crate::tile::seller::Seller;

// Per-instance data for every placed tile.
#[derive(Component)]
pub struct Tile {
    pub tile_type: TileType,
    /// Bottom-left cell of the footprint.
    pub pos: IVec2,
    /// Footprint after rotation (a 1x2 tile facing South is 2x1).
    pub size: IVec2,
    pub rotation: Direction,
}

// Which tile the player currently has selected for placing.
#[derive(Resource)]
pub struct SelectedTile(pub TileType);

impl Default for SelectedTile {
    fn default() -> Self {
        Self(TileType::Basic)
    }
}

/// Which way the next placed tile will face. Rotated with R.
#[derive(Resource, Default)]
pub struct PlaceDirection(pub Direction);

pub fn rotate_placement(keys: Res<ButtonInput<KeyCode>>, mut dir: ResMut<PlaceDirection>) {
    if keys.just_pressed(KeyCode::KeyR) {
        dir.0 = dir.0.rotate_cw();
        info!("Placing facing {:?}", dir.0);
    }
}

#[derive(Debug)]
pub enum PlaceError {
    OutOfBounds,
    Occupied,
    CantAfford,
}

/// Pure placement logic: no input, no UI, no modes.
/// `pos` is the bottom-left cell of the footprint after rotation.
pub fn place_tile(
    commands: &mut Commands,
    grid: &mut Grid,
    money: &mut Money,
    asset_server: &AssetServer,
    tile_type: TileType,
    pos: IVec2,
    rotation: Direction,
) -> Result<Entity, PlaceError> {
    let def = tile_type.def();

    // The footprint depends on rotation, so work it out here, not in the caller
    let size = Direction::rotate_size(def.size, rotation);

    // Check BOTH corners, otherwise a tall tile can hang off the grid
    if !Grid::in_bounds(pos) || !Grid::in_bounds(pos + size - IVec2::ONE) {
        return Err(PlaceError::OutOfBounds);
    }
    if !grid.is_rect_free(pos, size) {
        return Err(PlaceError::Occupied);
    }
    if money.0 < def.cost {
        return Err(PlaceError::CantAfford);
    }
    money.0 -= def.cost; // only after every check has passed

    // Center of the footprint, so the sprite lines up with the cells it covers
    let world_pos = grid_to_world(pos);
    let offset = (size.as_vec2() - Vec2::ONE) / 2.0 * TILE_SIZE;
    let final_pos = world_pos + offset;

    // Art is drawn facing East. Clockwise is a NEGATIVE angle in Bevy.
    let angle = -(rotation.quarter_turns() as f32) * FRAC_PI_2;

    let mut tile = commands.spawn((
        Tile {
            tile_type,
            pos,
            size,
            rotation,
        },
        Sprite {
            image: asset_server.load(def.texture_path),
            // Use the UNROTATED size here: the transform's rotation does the turning
            custom_size: Some(def.size.as_vec2() * TILE_SIZE),
            ..default()
        },
        Transform::from_translation(final_pos.extend(0.0))
            .with_rotation(Quat::from_rotation_z(angle)),
    ));

    // Each behavior in the definition adds its components

    // Place processor
    if let Some(recipe) = def.processor {
        tile.insert((
            Processor {
                recipe,
                progress: 0.0,
            },
            Inventory::default(),
        ));
    }

    // Place producer
    if let Some(producer) = def.producer {
        tile.insert((
            Producer {
                item: producer.item,
                timer: Timer::from_seconds(producer.interval, TimerMode::Repeating),
            },
            Inventory::default(),
        ));
    }

    // Place belt
    if let Some(speed) = def.conveyor_speed {
        tile.insert(Belt {
            speed,
            items: Vec::new(),
        });
    }

    if !def.ports.is_empty() {
        tile.insert((Ports::from_defs(&def.ports, pos, def.size, rotation), NeedsLink));
    }

    if def.seller {
        tile.insert(Seller);
    }

    let entity = tile.id();
    grid.set_rect(pos, size, entity);
    Ok(entity)
}

// Gated to Build mode and "cursor not over UI" by run conditions in tile.rs.
pub fn click_to_place(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut grid: ResMut<Grid>,
    mut money: ResMut<Money>,
    selected: Res<SelectedTile>,
    direction: Res<PlaceDirection>,
    asset_server: Res<AssetServer>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let (camera, cam_t) = *camera;
    let Some(pos) = cursor_grid_pos(*window, camera, cam_t) else {
        return;
    };

    if let Err(e) = place_tile(
        &mut commands,
        &mut grid,
        &mut money,
        &asset_server,
        selected.0,
        pos,
        direction.0,
    ) {
        info!("Can't place tile: {e:?}");
    }
}
