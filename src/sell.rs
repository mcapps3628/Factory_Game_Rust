use bevy::prelude::*;
use crate::economy::Money;
use crate::player_inventory::{SelectedItem, PlayerInventory};
use crate::mode::PlayerMode;

pub fn sell(
    mut money: ResMut<Money>,
    mut player_inventory: ResMut<PlayerInventory>,
    selected_item: Res<SelectedItem>,
    count: u32,
) {
    if player_inventory.try_remove(
        selected_item.0,
        count,
    ) {
        money.0 += selected_item.0.def().cost * count;
    }
}

pub fn click_to_sell(
    keys: Res<ButtonInput<KeyCode>>,
    money: ResMut<Money>,
    player_inventory: ResMut<PlayerInventory>,
    selected_item: Res<SelectedItem>,
    mode: Res<State<PlayerMode>>,
) {
    if !keys.just_pressed(KeyCode::KeyS) || *mode.get() != PlayerMode::Inventory {
        return;
    }

    let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
    let amount = if shift {
        player_inventory.items.get(&selected_item.0).copied().unwrap_or(0)
    } else {
        1
    };
    if amount == 0 {
        return;
    }

    sell(
        money,
        player_inventory,
        selected_item,
        amount,
    );
}