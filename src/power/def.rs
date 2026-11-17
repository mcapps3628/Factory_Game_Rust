use bevy::prelude::*;

#[derive(Component, Default)]
pub struct PowerNode {
    pub connections: Vec<Entity>,
}

#[derive(Component, Default)]
pub struct PowerDef {
    pub power: f32,
}

#[derive(Resource, Default)]
pub struct PowerNetworks(pub Vec<Vec<Entity>>);
