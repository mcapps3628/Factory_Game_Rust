use bevy::prelude::*;

pub struct ItemDef {
    pub name: &'static str,
    pub cost: u32,
    pub color: Color,
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ItemType {
    IronOre,
    IronPlate,
}
impl ItemType {
    pub const ALL: [ItemType; 2] = [ItemType::IronOre, ItemType::IronPlate];

    pub fn def(self) -> ItemDef {
        match self {

            ItemType::IronOre => ItemDef {
                name: "Iron Ore",
                cost: 1,
                color: Color::srgb(0.4, 0.7, 1.0),
            },

            ItemType::IronPlate => ItemDef {
                name: "Iron Plate",
                cost: 5,
                color: Color::srgb(1.0, 0.5, 0.2),
            },
            
        }
    }
}