use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::conveyor::{Conveyor, Direction, PlaceDirection};
use super::def::TileType;
use super::grid::{cursor_grid_pos, grid_to_world, Grid, TILE_SIZE};
use super::processing::{Inventory, Processor, Producer};
use crate::economy::Money;

// Per-instance data for every placed tile.
#[derive(Component)]
pub struct Tile {
    pub tile_type: TileType,
    pub pos: IVec2,
}

// Which tile the player currently has selected for placing.
#[derive(Resource)]
pub struct SelectedTile(pub TileType);

impl Default for SelectedTile {
    fn default() -> Self {
        Self(TileType::Basic)
    }
}

#[derive(Debug)]
pub enum PlaceError {
    OutOfBounds,
    Occupied,
    CantAfford,
}

/// Pure placement logic: no input, no UI, no modes.
/// `direction` only matters for tiles that have one (conveyors).
pub fn place_tile(
    commands: &mut Commands,
    grid: &mut Grid,
    money: &mut Money,
    tile_type: TileType,
    pos: IVec2,
    direction: Direction,
) -> Result<Entity, PlaceError> {
    if !Grid::in_bounds(pos) {
        return Err(PlaceError::OutOfBounds);
    }
    if grid.get(pos).is_some() {
        return Err(PlaceError::Occupied);
    }

    let def = tile_type.def();
    if money.0 < def.cost {
        return Err(PlaceError::CantAfford);
    }
    money.0 -= def.cost; // only after every check has passed

    let mut tile = commands.spawn((
        Tile { tile_type, pos },
        Sprite::from_color(def.color, Vec2::splat(TILE_SIZE - 2.0)),
        Transform::from_translation(grid_to_world(pos).extend(0.0)),
    ));

    // Each behavior in the definition adds its components
    if let Some(recipe) = def.processor {
        tile.insert((
            Processor {
                recipe,
                progress: 0.0,
            },
            Inventory::default(),
        ));
    }

    if let Some(speed) = def.conveyor_speed {
        tile.insert(Conveyor { direction, speed });
        // A small bright block on the exit edge shows which way the belt faces
        tile.with_children(|parent| {
            parent.spawn((
                Sprite::from_color(Color::srgb(0.9, 0.8, 0.3), Vec2::splat(TILE_SIZE * 0.2)),
                Transform::from_translation(
                    (direction.delta().as_vec2() * TILE_SIZE * 0.3).extend(0.1),
                ),
            ));
        });
    }

    if let Some(producer) = def.producer {
        tile.insert((
            Producer {
                item: producer.item,
                timer: Timer::from_seconds(producer.interval, TimerMode::Repeating),
            },
            Inventory::default(),
        ));
    }

    let entity = tile.id();
    grid.set(pos, entity);
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
        selected.0,
        pos,
        direction.0,
    ) {
        info!("Can't place tile: {e:?}");
    }
}