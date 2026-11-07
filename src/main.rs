use bevy::{platform::collections::HashMap, prelude::*};

use crate::{item::ItemType, player_inventory::PlayerInventory, player_inventory::SelectedItem};

mod economy;
mod item;
mod tile;
mod player_inventory;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, tile::TilePlugin))
        .insert_resource(economy::Money(500))
        .insert_resource(PlayerInventory {
            items: HashMap::from_iter([
                (ItemType::IronOre, 15),
                (ItemType::IronPlate, 2),
            ]),
        })
        .init_resource::<SelectedItem>()
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
