use bevy::prelude::*;

mod economy;
mod item;
mod mode;
mod player_inventory;
mod tile;
mod ui;
mod sell;

use item::ItemType;
use player_inventory::{PlayerInventory, SelectedItem};
use crate::sell::click_to_sell;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            mode::ModePlugin,
            tile::TilePlugin,
            ui::UiPlugin,
        ))
        .insert_resource(economy::Money(500))
        .init_resource::<PlayerInventory>()
        .init_resource::<SelectedItem>()
        .add_systems(Startup, (setup_camera, give_starting_items))
        .add_systems(Update, click_to_sell)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// TEMP: so there is something to put into furnaces while testing
fn give_starting_items(mut player: ResMut<PlayerInventory>) {
    player.add(ItemType::IronOre, 20);
}
 
