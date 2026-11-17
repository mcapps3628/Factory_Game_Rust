mod def;
mod grid;
mod placement;
mod processing;
mod remove;
mod direction;
mod port;
mod belt;
mod flow;
mod seller;

pub use def::TileType;
pub use placement::SelectedTile;
pub use grid::Grid;
pub use placement::Tile;
pub use grid::*;

use bevy::prelude::*;

use crate::mode::{PlayerMode, cursor_not_over_ui};

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<grid::Grid>()
            .init_resource::<SelectedTile>()
            .init_resource::<placement::PlaceDirection>()
            .add_systems(Startup, grid::spawn_grid_background)
            .add_systems(
                Update,
                (
                    // Build mode: place tiles
                    placement::click_to_place
                        .run_if(in_state(PlayerMode::Build))
                        .run_if(cursor_not_over_ui),
                    placement::rotate_placement.run_if(in_state(PlayerMode::Build)),
                    remove::click_to_remove,
                    port::link_new_ports
                        .after(placement::click_to_place),
                    belt::sync_belt_visuals,
                ),
            )
            // The simulation runs at a fixed rate, in a fixed order
            .add_systems(
                FixedUpdate,
                (
                    processing::process,
                    processing::produce,
                    flow::machine_io,
                    flow::transfer_items,
                    seller::sell_items,
                    belt::update_belts,
                )
                    .chain(),
            );
    }
}