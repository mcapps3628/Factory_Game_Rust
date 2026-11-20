pub mod def;
mod attach;
mod network;
mod wiring;

use bevy::prelude::*;

use crate::mode::cursor_not_over_ui;

/// Is_Wiring resource
fn wiring_active(wiring: Res<wiring::IsWiring>) -> bool {
    wiring.0
}

pub struct PowerPlugin;

impl Plugin for PowerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<def::PowerNetworks>()
            .init_resource::<wiring::IsWiring>()
            .init_resource::<wiring::SelectedPowerNodes>()
            .add_systems(
                Update,
                (
                    // Player input: pick two nodes to wire or unwire them
                    wiring::toggle_wiring,
                    wiring::select_power_node
                        .run_if(wiring_active)
                        .run_if(cursor_not_over_ui),
                    wiring::draw_mouse_wire.run_if(wiring_active),
                    wiring::draw_wires,
                    (
                        attach::attach_power,
                        network::rebuild_networks,
                        network::tick_networks,
                    )
                        .chain(),
                ),
            );
    }
}