use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::power::def::PowerNode;
use crate::tile::{Grid, Tile};
use crate::tile::*;

#[derive(Resource)]
pub struct IsWiring(bool);

#[derive(Resource)]
pub struct SelectedPowerNodes {
    pub nodes: Vec<Entity>,
}

pub fn select_power_node(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    grid: Res<Grid>,
    tiles: Query<&Tile>,
    mut is_wiring: ResMut<IsWiring>,
    mut selected_power: ResMut<SelectedPowerNodes>,
) {
    if !mouse.just_pressed(MouseButton::Left) && !is_wiring.0 {
        return;
    }

    let (camera, cam_t) = *camera;
    let Some(pos) = cursor_grid_pos(*window, camera, cam_t) else { return };

    let Some(entity) = grid.get(pos) else { return };
    if let Ok(tile) = tiles.get(entity) {
        //TODO: Add node check
        if selected_power.nodes.len() < 2 {
            selected_power.nodes.push(entity);
        } else {
            selected_power.nodes.remove(0);
            selected_power.nodes.push(entity);
        }
    }
}