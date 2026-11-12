mod collect;
mod conveyor;
mod def;
mod grid;
mod placement;
mod processing;
mod remove;

pub use def::TileType;
pub use placement::SelectedTile;

use bevy::prelude::*;

use crate::mode::{PlayerMode, cursor_not_over_ui};
pub(crate) use crate::tile::conveyor::{Direction, PlaceDirection};

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<grid::Grid>()
            .init_resource::<SelectedTile>()
            .init_resource::<conveyor::PlaceDirection>()
            .add_systems(Startup, grid::spawn_grid_background)
            .add_systems(
                Update,
                (
                    // Build mode: place tiles
                    conveyor::rotate_placement.run_if(in_state(PlayerMode::Build)),
                    placement::click_to_place
                        .run_if(in_state(PlayerMode::Build))
                        .run_if(cursor_not_over_ui),
                    // Inventory mode: move items in and out of machines
                    collect::collect_output
                        .run_if(in_state(PlayerMode::Inventory))
                        .run_if(cursor_not_over_ui),
                    collect::insert_input
                        .run_if(in_state(PlayerMode::Inventory))
                        .run_if(cursor_not_over_ui),
                    // Always: draw items where the simulation says they are
                    conveyor::sync_item_transforms,
                    remove::click_to_remove,
                ),
            )
            // The simulation runs at a fixed rate, in a fixed order
            .add_systems(
                FixedUpdate,
                (
                    processing::process,
                    processing::produce,
                    conveyor::machine_output,
                    conveyor::move_belt_items,
                )
                    .chain(),
            );
    }
}
