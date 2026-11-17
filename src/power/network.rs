use bevy::platform::collections::HashSet;
use bevy::prelude::*;

use crate::power::def::{PowerDef, PowerNode, PowerNetworks};

fn visit(
    current: Entity,
    nodes: &Query<(Entity, &PowerNode)>,
    seen: &mut HashSet<Entity>,
    members: &mut Vec<Entity>,
) {
    let Ok((_, node)) = nodes.get(current) else { return }; // dead entity: skip it
    if !seen.insert(current) {
        return; // already in a network
    }
    members.push(current);

    for &neighbour in &node.connections {
        visit(neighbour, nodes, seen, members);
    }
}

pub fn rebuild_networks(
    nodes: Query<(Entity, &PowerNode)>,
    mut networks: ResMut<PowerNetworks>,
) {
    networks.0.clear();
    let mut seen = HashSet::new();

    for (start, _) in &nodes {
        if seen.contains(&start) {
            continue;
        }
        let mut members = Vec::new();
        visit(start, &nodes, &mut seen, &mut members);
        networks.0.push(members);
    }
}