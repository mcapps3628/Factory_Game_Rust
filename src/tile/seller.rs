use bevy::prelude::*;
use crate::economy::Money;
use crate::tile::port::Ports;

#[derive(Component)]
pub struct Seller;

pub fn sell_items(mut sellers: Query<&mut Ports, With<Seller>>, mut money: ResMut<Money>) {
    for mut ports in &mut sellers {
        for port in ports.0.iter_mut() {
            if let Some(item) = port.slot.take() {
                money.0 += item.def().cost;
            }
        }
    }
}