use bevy::prelude::*;

use crate::item::ItemType;
use crate::tile::grid::TILE_SIZE;
use crate::tile::port::{PortKind, Ports};

// Minimum gap between two items, in tiles.
const SPACING: f32 = 0.3;

// Item Size, it must be smaller than SPACING * TILE_SIZE or items will overlap.
const ITEM_SIZE: f32 = 0.25;

pub struct BeltItem {
    pub item: ItemType,
    // 0.0 = entry edge, 1.0 = exit edge.
    pub progress: f32,
    // The sprite showing this item. It's a child of the belt entity, so it is despawned automatically if the belt is removed.
    pub sprite: Entity,
}

#[derive(Component)]
pub struct Belt {
    pub speed: f32,
    // Front of the belt first (highest progress), rearmost last.
    pub items: Vec<BeltItem>,
}

/// Item position in the belt's LOCAL space. The belt entity is already
/// rotated to face its direction, so items always move along local +X.
fn local_item_pos(progress: f32) -> Vec3 {
    Vec3::new((progress - 0.5) * TILE_SIZE, 0.0, 1.0)
}

/// Runs in FixedUpdate.
pub fn update_belts(
    mut commands: Commands,
    time: Res<Time>,
    mut belts: Query<(Entity, &mut Belt, &mut Ports)>,
) {
    let dt = time.delta_secs();
    for (belt_entity, mut belt, mut ports) in &mut belts {
        // Each item is clamped to stay SPACING behind the one in front.
        let step = belt.speed * dt;
        let mut limit = 1.0;
        for it in belt.items.iter_mut() {
            it.progress = (it.progress + step).min(limit).max(it.progress);
            limit = it.progress - SPACING;
        }

        // Front item leaves if it hit the end AND an output slot is free.
        let ready = belt
            .items
            .first()
            .filter(|f| f.progress >= 1.0)
            .map(|f| f.item);
        if let Some(item) = ready {
            if let Some(out) = ports
                .0
                .iter_mut()
                .find(|p| p.kind == PortKind::Output && p.slot.is_none() && p.link.is_some())
            {
                out.slot = Some(item);
                let done = belt.items.remove(0);
                commands.entity(done.sprite).despawn();
            }
        }

        // Take one item from an input slot if there's room at the back.
        let room = belt
            .items
            .last()
            .map_or(true, |l| l.progress >= SPACING);
        if room {
            if let Some(inp) = ports
                .0
                .iter_mut()
                .find(|p| p.kind == PortKind::Input && p.slot.is_some())
            {
                let item = inp.slot.take().unwrap();

                // Item sprite, as a child of the belt.
                let sprite = commands
                    .spawn((
                        Sprite::from_color(item.def().color, Vec2::splat(TILE_SIZE * ITEM_SIZE)),
                        Transform::from_translation(local_item_pos(0.0)),
                    ))
                    .id();
                commands.entity(belt_entity).add_child(sprite);

                belt.items.push(BeltItem {
                    item,
                    progress: 0.0,
                    sprite,
                });
            }
        }
    }
}

/// Runs every frame in Update: puts each sprite where the simulation says it is.
pub fn sync_belt_visuals(
    belts: Query<&Belt>,
    mut transforms: Query<&mut Transform, Without<Belt>>,
) {
    for belt in &belts {
        for it in &belt.items {
            if let Ok(mut transform) = transforms.get_mut(it.sprite) {
                transform.translation = local_item_pos(it.progress);
            }
        }
    }
}