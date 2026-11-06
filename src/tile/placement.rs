use bevy::prelude::*;
use bevy::ui_widgets::Button;
use bevy::window::PrimaryWindow;

use super::def::TileType;
use super::grid::{cursor_grid_pos, grid_to_world, Grid, TILE_SIZE};
use super::processing::{Inventory, Processor};
use crate::economy::Money;
use crate::item::ItemType;

// Per-instance data for every placed tile.
#[derive(Component)]
#[allow(dead_code)]
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

pub fn place_tile(
    commands: &mut Commands,
    grid: &mut Grid,
    money: &mut Money,
    tile_type: TileType,
    pos: IVec2,
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

    // Tiles that craft things get extra components
    if let Some(recipe) = def.processor {
        let mut inventory = Inventory::default();
        inventory.input.insert(ItemType::IronOre, 10); // TEMP: no belts yet, so seed some ore
        tile.insert((
            Processor {
                recipe,
                progress: 0.0,
            },
            inventory,
        ));
    }

    let entity = tile.id();
    grid.set(pos, entity);
    Ok(entity)
}

pub fn select_tile(keys: Res<ButtonInput<KeyCode>>, mut selected: ResMut<SelectedTile>) {
    if keys.just_pressed(KeyCode::Digit1) {
        selected.0 = TileType::Basic;
    }
    if keys.just_pressed(KeyCode::Digit2) {
        selected.0 = TileType::Furnace;
    }
}

pub fn click_to_place(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut grid: ResMut<Grid>,
    mut money: ResMut<Money>,
    selected: Res<SelectedTile>,
    buttons: Query<&Interaction, With<Button>>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    // Don't place a tile through a UI button
    if buttons.iter().any(|i| *i != Interaction::None) {
        return;
    }

    let (camera, cam_t) = *camera;
    let Some(pos) = cursor_grid_pos(*window, camera, cam_t) else {
        return;
    };

    if let Err(e) = place_tile(&mut commands, &mut grid, &mut money, selected.0, pos) {
        info!("Can't place tile: {e:?}");
    }
}