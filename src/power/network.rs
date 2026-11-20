use bevy::platform::collections::HashSet;
use bevy::prelude::*;
use crate::power::def::{PowerNetworks, PowerNode, Powered};

/// Rebuild power network
pub fn rebuild_networks(
    nodes: Query<(Entity, &PowerNode)>,
    changed: Query<(), Changed<PowerNode>>,
    mut removed: RemovedComponents<PowerNode>,
    mut networks: ResMut<PowerNetworks>,
) {
    let removed_any = removed.read().count() > 0;
    if changed.is_empty() && !removed_any {
        return;
    }

    networks.0.clear();
    let mut seen: HashSet<Entity> = HashSet::new();

    for (start, _) in &nodes {
        if seen.contains(&start) {
            continue;
        }

        // Depth-first search with an explicit stack
        let mut members = Vec::new();
        let mut stack = vec![start];
        seen.insert(start);

        while let Some(current) = stack.pop() {
            members.push(current);

            let Ok((_, node)) = nodes.get(current) else { continue };
            for &next in &node.connections {
                // contains(): skip dead entities; insert(): true only the first time seen
                if nodes.contains(next) && seen.insert(next) {
                    stack.push(next);
                }
            }
        }

        networks.0.push(members);
    }
}

/// Update network
pub fn tick_networks(
    mut commands: Commands,
    networks: Res<PowerNetworks>,
    nodes: Query<(&PowerNode, Has<Powered>)>,
) {
    if !networks.is_changed() {
        return;
    }

    for network in &networks.0 {
        let mut total_power = 0.0;
        for &entity in network {
            if let Ok((node, _)) = nodes.get(entity) {
                total_power += node.power;
            }
        }

        let has_power = total_power >= 0.0;

        for &entity in network {
            let Ok((_, is_powered)) = nodes.get(entity) else { continue };

            if has_power && !is_powered {
                commands.entity(entity).insert(Powered);
            } else if !has_power && is_powered {
                commands.entity(entity).remove::<Powered>();
            }
        }
    }
}