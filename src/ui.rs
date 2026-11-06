mod build_menu;
mod inventory_menu;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, build_menu::spawn_build_menu)
            .add_systems(
                Update,
                (
                    build_menu::handle_build_buttons,
                    build_menu::highlight_selected,
                    build_menu::update_money_text,
                ),
            );
        app.add_systems(Startup, inventory_menu::spawn_inventory_menu)
            .add_systems(
                Update,
                (
                    inventory_menu::handle_inventory_buttons,
                    inventory_menu::highlight_selected,
                ),
            );    
    }
}