use bevy::prelude::*;

#[derive(Component, Default)]
pub struct PowerNode {
    pub power: f32,
    pub connections: Vec<Entity>,
}

#[derive(Component)]
pub struct Powered;

#[derive(Resource, Default)]
pub struct PowerNetworks(pub Vec<Vec<Entity>>);