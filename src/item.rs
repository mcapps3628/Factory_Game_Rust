use bevy::prelude::*;

pub struct ItemDef {
    pub name: &'static str,
    pub cost: u32,
    pub color: Color, //TODO: Change to image
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ItemType {
    IronOre,
    IronIngot,
    IronWire,
    Coal,
}
impl ItemType {
    pub const ALL: [ItemType; 4] = [ItemType::IronOre, ItemType::IronIngot, ItemType::IronWire, ItemType::Coal];

    pub fn def(self) -> ItemDef {
        match self {

            ItemType::IronOre => ItemDef {
                name: "Iron Ore",
                cost: 2,
                color: Color::srgb(0.4, 0.7, 1.0),
            },

            ItemType::IronIngot => ItemDef {
                name: "Iron Plate",
                cost: 5,
                color: Color::srgb(1.0, 0.5, 0.2),
            },

            ItemType::IronWire => ItemDef {
                name: "Iron Wire",
                cost: 10,
                color: Color::srgb(0.6, 1.0, 1.0),
            },

            ItemType::Coal => ItemDef {
                name: "Coal",
                cost: 1,
                color: Color::srgb(0.0, 0.0, 0.0),
            },
            
        }
    }
}