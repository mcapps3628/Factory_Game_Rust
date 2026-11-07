mod def;
mod grid;
mod placement;
mod processing;
mod collect;

pub use placement::SelectedTile;
use bevy::prelude::*;
use crate::tile::processing::Inventory;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<grid::Grid>()
            .init_resource::<SelectedTile>()
            .init_resource::<collect::InventoryMode>()
            .add_systems(Startup, grid::spawn_grid_background)
            .add_systems(Update, (placement::select_tile, placement::click_to_place))
            .add_systems(FixedUpdate, processing::process)
            .add_systems(Update, placement::select_tile)
            .add_systems(Update, placement::click_to_place)
            .add_systems(Update, collect::collect_output)
            .add_systems(Update, collect::insert_input)
            .add_systems(Update, collect::inventory_mode);
    }
}
