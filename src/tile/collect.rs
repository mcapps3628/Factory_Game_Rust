use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use super::grid::{cursor_grid_pos, Grid};
use super::processing::Inventory;
use super::processing::Processor;
use crate::player_inventory::{PlayerInventory, SelectedItem};

#[derive(Resource, Default, Debug)]
pub struct InventoryMode(bool);

pub fn collect_output(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    grid: Res<Grid>,
    mut tiles: Query<&mut Inventory>,
    mut player: ResMut<PlayerInventory>,
    inventory_mode: Res<InventoryMode>,
) {
    if inventory_mode.0 {
        if !mouse.just_pressed(MouseButton::Right) {
            return;
        }

        let (camera, cam_t) = *camera;
        let Some(pos) = cursor_grid_pos(*window, camera, cam_t) else { return };
        let Some(entity) = grid.get(pos) else { return };
        let Ok(mut inv) = tiles.get_mut(entity) else { return };

        for (item, count) in inv.output.drain() {
            player.add(item, count);
            info!("Collected {count} x {item:?}");
        }
    }
}

pub fn insert_input(
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    grid: Res<Grid>,
    mut tiles: Query<(&Processor, &mut Inventory)>,
    mut player: ResMut<PlayerInventory>,
    selected: Res<SelectedItem>,
    inventory_mode: Res<InventoryMode>,
) {
    if inventory_mode.0 {
        if !mouse.just_pressed(MouseButton::Left) {
            return;
        }

        let (camera, cam_t) = *camera;
        let Some(pos) = cursor_grid_pos(*window, camera, cam_t) else { return };
        let Some(entity) = grid.get(pos) else { return };
        let Ok((processor, mut inv)) = tiles.get_mut(entity) else { return };

        let item = selected.0;

        // Only accept items this recipe actually uses
        if !processor.recipe.inputs.iter().any(|(i, _)| *i == item) {
            info!("{item:?} isn't an input for this tile");
            return;
        }

        let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
        let amount = if shift {
            player.items.get(&item).copied().unwrap_or(0)
        } else {
            1
        };
        if amount == 0 {
            return;
        }

        // try_remove is the gate: no items taken means nothing is added
        if player.try_remove(item, amount) {
            *inv.input.entry(item).or_default() += amount;
            info!("Inserted {amount} x {item:?}");
        }
    }
}

pub fn inventory_mode(
    mut inventory: ResMut<InventoryMode>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyE) {
        inventory.0 = !inventory.0;
        info!("Inventory mode: {inventory:?}");
    }
}

