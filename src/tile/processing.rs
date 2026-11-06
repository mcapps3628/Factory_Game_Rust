use std::collections::HashMap;

use bevy::prelude::*;

use super::def::Recipe;
use crate::item::ItemType;

#[derive(Component)]
pub struct Processor {
    pub recipe: Recipe,
    pub progress: f32,
}

#[derive(Component, Default)]
pub struct Inventory {
    pub input: HashMap<ItemType, u32>,
    pub output: HashMap<ItemType, u32>,
}

// Runs in FixedUpdate.
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
            info!("Crafted! output buffer: {:?}", inv.output);
        }
    }
}