use bevy::prelude::*;
use crate::tile::direction::Direction;
use crate::item::ItemType;
use crate::tile::grid::Grid;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortKind {
    Input,
    Output,
}

impl PortKind {
    pub fn opposite(self) -> Self {
        match self {
            PortKind::Input => PortKind::Output,
            PortKind::Output => PortKind::Input,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PortDef {
    pub local: IVec2,
    pub facing: Direction,
    pub kind: PortKind
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PortRef {
    pub tile: Entity,
    pub index: usize,   // position in the other tile's Ports vector
}

#[derive(Debug)]
pub struct Port {
    pub cell: IVec2,             // actual grid cell
    pub facing: Direction,       // actual (rotated) direction
    pub kind: PortKind,
    pub link: Option<PortRef>,   // the partner port, once linked
    pub slot: Option<ItemType>,  // one-item buffer
}

// Final component that is used on an entity
#[derive(Component, Debug, Default)]
pub struct Ports(pub Vec<Port>);

impl Ports {
    pub fn from_defs(defs: &[PortDef], origin: IVec2, base_size: IVec2, rotation: Direction) -> Self {
        Self(
            defs.iter()
                .map(|d| Port {
                    cell: origin + Direction::rotate_cell(d.local, base_size, rotation),
                    facing: d.facing.turned_by(rotation),
                    kind: d.kind,
                    link: None,
                    slot: None,
                })
                .collect(),
        )
    }
}

#[derive(Component)]
pub struct NeedsLink;

pub fn link_new_ports(
    mut commands: Commands,
    grid: Res<Grid>,
    new_tiles:Query<Entity, With<NeedsLink>>,
    mut tiles: Query<&mut Ports>,
) {
    for entity in &new_tiles {
        link_tile(entity, &grid, &mut tiles);
        commands.entity(entity).remove::<NeedsLink>();
    }
}

fn link_tile(entity: Entity, grid: &Grid, tiles: &mut Query<&mut Ports>) {
    // copy our own ports first so we aren't borrowing while we edit neighbors
    let mine: Vec<(usize, IVec2, Direction, PortKind)> = match tiles.get(entity) {
        Ok(p) => p.0.iter().enumerate().map(|(i, p)| (i, p.cell, p.facing, p.kind)).collect(),
        Err(_) => return,
    };

    for (i, cell, facing, kind) in mine {
        let target = cell + facing.delta();
        let Some(other) = grid.get(target) else { continue };
        if other == entity { continue; }

        let j = {
            let Ok(mut target_port) = tiles.get_mut(other) else { continue };
            let Some(j) = target_port.0.iter().position(|p|
                p.cell == target
                && p.facing == facing.opposite()
                && p.kind == kind.opposite()
            ) else { continue };
            target_port.0[j].link = Some(PortRef { tile: entity, index: i });
            j
        };
        if let Ok(mut me) = tiles.get_mut(entity) {
            me.0[i].link = Some(PortRef { tile: other, index: j });
            info!("linked {entity:?}[{i}] <-> {other:?}[{j}]");
        }
    }
}