use std::collections::HashMap;

use bevy::prelude::*;

use super::grid::{grid_to_world, Grid, TILE_SIZE};
use super::placement::Tile;
use super::processing::{Inventory, Processor};
use crate::item::ItemType;

/// Minimum gap between two items on a belt, measured in tiles.
const ITEM_SPACING: f32 = 0.4;
/// A machine stops accepting an input item once it holds this many.
const MACHINE_INPUT_CAP: u32 = 20;

// ---------------------------------------------------------------------------
// Direction
// ---------------------------------------------------------------------------

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    #[default]
    East,
    South,
    West,
}

impl Direction {
    pub const ALL: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    /// The grid offset of one step in this direction.
    pub fn delta(self) -> IVec2 {
        match self {
            Direction::North => IVec2::Y,
            Direction::East => IVec2::X,
            Direction::South => IVec2::NEG_Y,
            Direction::West => IVec2::NEG_X,
        }
    }

    pub fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    pub fn rotate_cw(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

/// Which way the next conveyor will face when placed. Rotate with R.
#[derive(Resource, Default)]
pub struct PlaceDirection(pub Direction);

pub fn rotate_placement(keys: Res<ButtonInput<KeyCode>>, mut dir: ResMut<PlaceDirection>) {
    if keys.just_pressed(KeyCode::KeyR) {
        dir.0 = dir.0.rotate_cw();
        info!("Placing facing {:?}", dir.0);
    }
}

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

/// A conveyor tile. Moves items toward the edge it faces.
#[derive(Component)]
pub struct Conveyor {
    pub direction: Direction,
    pub speed: f32, // tiles per second
}

/// A physical item that exists in the world as its own entity.
#[derive(Component)]
pub struct WorldItem(pub ItemType);

/// Where an item is riding. `progress` runs from 0.0 (entry edge) to 1.0 (exit edge).
#[derive(Component)]
pub struct OnBelt {
    pub belt: Entity,
    pub progress: f32,
}

// ---------------------------------------------------------------------------
// Spawning items
// ---------------------------------------------------------------------------

// Move this into your item definitions once items have an ItemDef.
fn item_color(item: ItemType) -> Color {
    match item {
        ItemType::IronOre => Color::srgb(0.60, 0.45, 0.35),
        ItemType::IronPlate => Color::srgb(0.80, 0.80, 0.90),
    }
}

/// World position of an item at `progress` along a belt tile.
fn belt_item_position(tile_pos: IVec2, dir: Direction, progress: f32) -> Vec3 {
    let center = grid_to_world(tile_pos);
    let offset = dir.delta().as_vec2() * (progress - 0.5) * TILE_SIZE;
    (center + offset).extend(1.0) // z = 1 draws above tiles
}

fn spawn_belt_item(
    commands: &mut Commands,
    item: ItemType,
    belt: Entity,
    belt_pos: IVec2,
    dir: Direction,
) {
    commands.spawn((
        WorldItem(item),
        OnBelt { belt, progress: 0.0 },
        Sprite::from_color(item_color(item), Vec2::splat(TILE_SIZE * 0.3)),
        Transform::from_translation(belt_item_position(belt_pos, dir, 0.0)),
    ));
}

// ---------------------------------------------------------------------------
// Machines put their output onto adjacent belts
// ---------------------------------------------------------------------------

pub fn machine_output(
    mut commands: Commands,
    grid: Res<Grid>,
    belts: Query<(&Conveyor, &Tile)>,
    items: Query<&OnBelt>,
    mut machines: Query<(&Tile, &mut Inventory)>,
) {
    // Item closest to each belt's entrance; a new item needs ITEM_SPACING of room
    let mut rearmost: HashMap<Entity, f32> = HashMap::new();
    for item in &items {
        let slot = rearmost.entry(item.belt).or_insert(f32::MAX);
        *slot = slot.min(item.progress);
    }

    for (tile, mut inv) in &mut machines {
        if inv.output.is_empty() {
            continue;
        }

        for dir in Direction::ALL {
            let Some(belt_entity) = grid.get(tile.pos + dir.delta()) else { continue };
            let Ok((conveyor, belt_tile)) = belts.get(belt_entity) else { continue };

            // Never push onto a belt that is carrying items INTO this machine
            if belt_tile.pos + conveyor.direction.delta() == tile.pos {
                continue;
            }
            // Room at the entrance?
            if rearmost.get(&belt_entity).copied().unwrap_or(f32::MAX) < ITEM_SPACING {
                continue;
            }

            let Some(item) = inv.output.iter().find(|(_, n)| **n > 0).map(|(i, _)| *i) else {
                break;
            };

            let count = inv.output.get_mut(&item).unwrap();
            *count -= 1;
            if *count == 0 {
                inv.output.remove(&item);
            }

            spawn_belt_item(&mut commands, item, belt_entity, belt_tile.pos, conveyor.direction);
            rearmost.insert(belt_entity, 0.0);
            break; // one item per machine per tick
        }
    }
}

// ---------------------------------------------------------------------------
// Moving items along belts
// ---------------------------------------------------------------------------

enum Handoff {
    Belt(Entity),
    Machine,
    Blocked,
}

/// What happens when an item reaches the end of its belt.
fn try_handoff(
    next_pos: IVec2,
    from_dir: Direction,
    item: ItemType,
    grid: &Grid,
    belts: &Query<(&Conveyor, &Tile)>,
    machines: &mut Query<(&Processor, &mut Inventory)>,
    rearmost: &HashMap<Entity, f32>,
) -> Handoff {
    let Some(target) = grid.get(next_pos) else {
        return Handoff::Blocked;
    };

    // Next tile is a belt: it must not face us, and must have room at its entrance
    if let Ok((next_conveyor, _)) = belts.get(target) {
        if next_conveyor.direction == from_dir.opposite() {
            return Handoff::Blocked;
        }
        if rearmost.get(&target).copied().unwrap_or(f32::MAX) < ITEM_SPACING {
            return Handoff::Blocked;
        }
        return Handoff::Belt(target);
    }

    // Next tile is a machine: it must use this item and have space
    if let Ok((processor, mut inv)) = machines.get_mut(target) {
        let accepts = processor.recipe.inputs.iter().any(|(i, _)| *i == item);
        let have = inv.input.get(&item).copied().unwrap_or(0);
        if accepts && have < MACHINE_INPUT_CAP {
            *inv.input.entry(item).or_default() += 1;
            return Handoff::Machine;
        }
    }

    Handoff::Blocked
}

pub fn move_belt_items(
    mut commands: Commands,
    time: Res<Time>,
    grid: Res<Grid>,
    belts: Query<(&Conveyor, &Tile)>,
    mut items: Query<(Entity, &WorldItem, &mut OnBelt)>,
    mut machines: Query<(&Processor, &mut Inventory)>,
) {
    let dt = time.delta_secs();

    // 1. Group items by belt, front of the belt first
    let mut by_belt: HashMap<Entity, Vec<(Entity, f32, ItemType)>> = HashMap::new();
    for (entity, world_item, on_belt) in &items {
        by_belt
            .entry(on_belt.belt)
            .or_default()
            .push((entity, on_belt.progress, world_item.0));
    }
    for list in by_belt.values_mut() {
        list.sort_by(|a, b| b.1.total_cmp(&a.1));
    }

    // 2. Entrance clearance per belt (kept up to date as items hop between belts)
    let mut rearmost: HashMap<Entity, f32> = by_belt
        .iter()
        .map(|(belt, list)| (*belt, list.last().map(|(_, p, _)| *p).unwrap_or(f32::MAX)))
        .collect();

    // 3. Advance each belt's items, front to back
    for (belt_entity, list) in &by_belt {
        let Ok((conveyor, tile)) = belts.get(*belt_entity) else {
            // The belt was removed: clean up its items
            for &(item_entity, _, _) in list {
                commands.entity(item_entity).despawn();
            }
            continue;
        };

        let mut limit = 1.0; // how far the next item is allowed to travel
        for &(item_entity, progress, item_type) in list {
            let step = conveyor.speed * dt;
            let mut new_progress = (progress + step).min(limit).max(progress);

            if new_progress >= 1.0 {
                let next_pos = tile.pos + conveyor.direction.delta();
                match try_handoff(
                    next_pos,
                    conveyor.direction,
                    item_type,
                    &grid,
                    &belts,
                    &mut machines,
                    &rearmost,
                ) {
                    Handoff::Machine => {
                        commands.entity(item_entity).despawn();
                        continue; // left this belt; the next item's limit is unchanged
                    }
                    Handoff::Belt(next_belt) => {
                        if let Ok((_, _, mut on_belt)) = items.get_mut(item_entity) {
                            on_belt.belt = next_belt;
                            on_belt.progress = 0.0;
                        }
                        rearmost.insert(next_belt, 0.0);
                        continue;
                    }
                    Handoff::Blocked => new_progress = 1.0,
                }
            }

            if let Ok((_, _, mut on_belt)) = items.get_mut(item_entity) {
                on_belt.progress = new_progress;
            }
            limit = new_progress - ITEM_SPACING;
        }
    }
}

/// Puts each item's sprite at its belt position. Runs every frame.
pub fn sync_item_transforms(
    belts: Query<(&Conveyor, &Tile)>,
    mut items: Query<(&OnBelt, &mut Transform)>,
) {
    for (on_belt, mut transform) in &mut items {
        let Ok((conveyor, tile)) = belts.get(on_belt.belt) else { continue };
        transform.translation = belt_item_position(tile.pos, conveyor.direction, on_belt.progress);
    }
}