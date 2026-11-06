mod def;
mod grid;
mod placement;
mod processing;

pub use def::TileType;
pub use placement::SelectedTile;

use bevy::prelude::*;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<grid::Grid>()
            .init_resource::<SelectedTile>()
            .add_systems(Startup, grid::spawn_grid_background)
            .add_systems(Update, (placement::select_tile, placement::click_to_place))
            .add_systems(FixedUpdate, processing::process);
    }
}
