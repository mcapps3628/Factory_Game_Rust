use std::collections::HashMap;

use bevy::prelude::*;

use super::def::Recipe;
use crate::item::ItemType;

/// A miner stops producing once its output buffer holds this many.
const OUTPUT_CAP: u32 = 10;

#[derive(Component)]
pub struct Processor {
    pub recipe: Recipe,
    pub progress: f32,
}

/// Makes items out of nothing on a timer and puts them in the output buffer.
#[derive(Component)]
pub struct Producer {
    pub item: ItemType,
    pub timer: Timer,
}

#[derive(Component, Default)]
pub struct Inventory {
    pub input: HashMap<ItemType, u32>,
    pub output: HashMap<ItemType, u32>,
}

/// Runs in FixedUpdate so production speed doesn't depend on frame rate.
pub fn process(time: Res<Time>, mut query: Query<(&mut Processor, &mut Inventory)>) {
    let dt = time.delta_secs();

    for (mut processor, mut inv) in &mut query {
        let recipe = processor.recipe;

        // Don't start (or keep progressing) without all inputs
        let has_inputs = recipe
            .inputs
            .iter()
            .all(|(item, n)| inv.input.get(item).copied().unwrap_or(0) >= *n);
        if !has_inputs {
            processor.progress = 0.0;
            continue;
        }

        processor.progress += dt;
        if processor.progress >= recipe.craft_time {
            processor.progress -= recipe.craft_time;

            for (item, n) in recipe.inputs {
                *inv.input.get_mut(item).unwrap() -= n; // safe: checked above
            }
            for (item, n) in recipe.outputs {
                *inv.output.entry(*item).or_default() += n;
            }
        }
    }
}

pub fn produce(time: Res<Time>, mut query: Query<(&mut Producer, &mut Inventory)>) {
    for (mut producer, mut inv) in &mut query {
        producer.timer.tick(time.delta());
        if producer.timer.just_finished() {
            let stored = inv.output.entry(producer.item).or_default();
            if *stored < OUTPUT_CAP {
                *stored += 1;
            }
        }
    }
}