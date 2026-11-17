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
    nodes: Query<(), With<PowerNode>>,
    is_wiring: Res<IsWiring>,
    mut selected_power: ResMut<SelectedPowerNodes>,
) {
    if !is_wiring.0 || !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let (camera, cam_t) = *camera;
    let Some(pos) = cursor_grid_pos(*window, camera, cam_t) else { return };
    let Some(entity) = grid.get(pos) else { return };

    if !nodes.contains(entity) || selected_power.nodes.contains(&entity) {
        return;
    }

    selected_power.nodes.push(entity);
    if selected_power.nodes.len() > 2 {
        selected_power.nodes.remove(0); // keep only the two most recent
    }
}

pub fn draw_mouse_wire(
    mut gizmos: Gizmos,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    is_wiring: Res<IsWiring>,
    selected: Res<SelectedPowerNodes>,
    transforms: Query<&Transform>,
) {
    // Only while wiring and exactly one end has been picked
    if !is_wiring.0 || selected.nodes.len() != 1 {
        return;
    }
    let Ok(start) = transforms.get(selected.nodes[0]) else { return };

    let (camera, cam_t) = *camera;
    let Some(cursor) = window.cursor_position() else { return };
    let Ok(world) = camera.viewport_to_world_2d(cam_t, cursor) else { return };

    gizmos.line_2d(start.translation.truncate(), world, Color::srgb(1.0, 0.9, 0.2));
}