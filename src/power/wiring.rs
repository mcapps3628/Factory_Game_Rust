use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::def::PowerNode;
use crate::tile::{cursor_grid_pos, Grid};

/// Is the player in wiring mode? (P toggles it.)
#[derive(Resource, Default)]
pub struct IsWiring(pub bool);

/// The node(s) picked so far for the wire being made. Cleared after every pair.
#[derive(Resource, Default)]
pub struct SelectedPowerNodes {
    pub nodes: Vec<Entity>,
}

pub fn toggle_wiring(
    keys: Res<ButtonInput<KeyCode>>,
    mut wiring: ResMut<IsWiring>,
    mut selected: ResMut<SelectedPowerNodes>,
) {
    if keys.just_pressed(KeyCode::KeyP) {
        wiring.0 = !wiring.0;
        selected.nodes.clear(); // never carry a half-made wire across modes
        info!("Wiring: {}", wiring.0);
    }
}

pub fn select_power_node(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    grid: Res<Grid>,
    mut nodes: Query<&mut PowerNode>,
    is_wiring: Res<IsWiring>,
    mut selected: ResMut<SelectedPowerNodes>,
) {
    if !is_wiring.0 || !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let (camera, cam_t) = *camera;
    let Some(pos) = cursor_grid_pos(*window, camera, cam_t) else { return };
    let Some(entity) = grid.get(pos) else { return };

    if !nodes.contains(entity) || selected.nodes.contains(&entity) {
        return;
    }

    selected.nodes.push(entity);

    // Second pick completes the pair: wire them (or unwire them if already wired), then reset
    if selected.nodes.len() == 2 {
        let (a, b) = (selected.nodes[0], selected.nodes[1]);
        toggle_connection(&mut nodes, a, b);
        selected.nodes.clear();
    }
}

/// Wire two nodes together, or remove the wire if they are already connected.
fn toggle_connection(
    nodes: &mut Query<&mut PowerNode>,
    a: Entity,
    b: Entity
) {
    if a == b {
        return;
    }

    let already = nodes
        .get_mut(a)
        .map(|node| node.connections.contains(&b))
        .unwrap_or(false);

    if let Ok(mut node) = nodes.get_mut(a) {
        if already {
            node.connections.retain(|&e| e != b);
        } else {
            node.connections.push(b);
        }
    }
    if let Ok(mut node) = nodes.get_mut(b) {
        if already {
            node.connections.retain(|&e| e != a);
        } else {
            node.connections.push(a);
        }
    }
}

/// The preview line from the first picked node to the cursor.
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

/// Every finished wire. Gizmos last one frame, this redraws them every frame.
pub fn draw_wires(
    mut gizmos: Gizmos,
    nodes: Query<(Entity, &PowerNode)>,
    transforms: Query<&Transform>,
) {
    for (entity, node) in &nodes {
        let Ok(from) = transforms.get(entity) else { continue };

        for &other in &node.connections {
            // Each wire is listed by both ends; draw it once
            if entity.to_bits() > other.to_bits() {
                continue;
            }
            let Ok(to) = transforms.get(other) else { continue };
            gizmos.line_2d(
                from.translation.truncate(),
                to.translation.truncate(),
                Color::srgb(1.0, 0.9, 0.2),
            );
        }
    }
}