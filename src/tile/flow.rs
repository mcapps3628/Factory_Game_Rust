use bevy::prelude::*;

use super::port::{PortKind, PortRef, Ports};
use super::processing::{Inventory, Processor};
use crate::item::ItemType;

/// A machine stops accepting an input item once it holds this many.
const MACHINE_INPUT_CAP: u32 = 20;

/// Connects a machine's inventory to its ports.
/// - Output port with an empty slot: take one item out of the inventory's
///   output and put it in the slot.
/// - Input port with an item in its slot: if the machine's recipe uses that
///   item and there's room, move it into the inventory. Otherwise, it stays
///   in the slot, which blocks whatever is feeding it.
pub fn machine_io(mut machines: Query<(&mut Ports, &mut Inventory, Option<&Processor>)>) {
    for (mut ports, mut inv, processor) in &mut machines {
        for port in ports.0.iter_mut() {
            match port.kind {
                PortKind::Output => {
                    if port.slot.is_some() {
                        continue;
                    }
                    // Pick any item type that has at least one in the output
                    let Some(item) = inv
                        .output
                        .iter()
                        .find(|(_, n)| **n > 0)
                        .map(|(item, _)| *item)
                    else {
                        continue;
                    };

                    let count = inv.output.get_mut(&item).unwrap();
                    *count -= 1;
                    if *count == 0 {
                        inv.output.remove(&item);
                    }
                    port.slot = Some(item);
                }

                PortKind::Input => {
                    // Only machines with a recipe accept items
                    let (Some(item), Some(processor)) = (port.slot, processor) else {
                        continue;
                    };
                    let accepts = processor.recipe.inputs.iter().any(|(i, _)| *i == item);
                    let have = inv.input.get(&item).copied().unwrap_or(0);

                    if accepts && have < MACHINE_INPUT_CAP {
                        *inv.input.entry(item).or_default() += 1;
                        port.slot = None;
                    }
                }
            }
        }
    }
}

/// Moves items from output slots into the linked input slots.
/// An item only moves if the receiving slot is empty, so a full belt or
/// machine automatically backs things up behind it.
pub fn transfer_items(mut tiles: Query<(Entity, &mut Ports)>) {
    // Pass 1: decide the moves using read-only access.
    // (Rust won't let us hold two mutable borrows of the query at once.)
    let mut moves: Vec<(Entity, usize, PortRef, ItemType)> = Vec::new();

    for (entity, ports) in tiles.iter() {
        for (i, port) in ports.0.iter().enumerate() {
            if port.kind != PortKind::Output {
                continue;
            }
            let (Some(item), Some(link)) = (port.slot, port.link) else {
                continue;
            };
            // The linked tile may have been despawned
            let Ok((_, theirs)) = tiles.get(link.tile) else {
                continue;
            };
            let target_is_free = theirs
                .0
                .get(link.index)
                .is_some_and(|p| p.slot.is_none());

            if target_is_free {
                moves.push((entity, i, link, item));
            }
        }
    }

    // Pass 2: apply them.
    for (from, from_index, to, item) in moves {
        let delivered = match tiles.get_mut(to.tile) {
            Ok((_, mut target)) => match target.0.get_mut(to.index) {
                Some(port) if port.slot.is_none() => {
                    port.slot = Some(item);
                    true
                }
                _ => false,
            },
            Err(_) => false,
        };

        if delivered {
            if let Ok((_, mut source)) = tiles.get_mut(from) {
                source.0[from_index].slot = None;
            }
            info!("moved {item:?}"); // remove once it works
        }
    }
}