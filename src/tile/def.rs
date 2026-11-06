use bevy::prelude::*;
use crate::item::ItemType;

// Crafting recipe for tiles
#[derive(Debug, Clone, Copy)]
pub struct Recipe {
    pub inputs: &'static [(ItemType, u32)],
    pub outputs: &'static [(ItemType, u32)],
    pub craft_time: f32,
}

// Everything that is true of tile type (not placed instance)
pub struct TileDef {
    pub name: &'static str,
    pub cost: u32,
    pub color: Color,
    pub processor: Option<Recipe>, //optional to set a processor
}

// Tile type list
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileType {
    Basic, // Test
    Furnace,
}

impl TileType {
    // Every tile type definition.
    pub const ALL: [TileType; 2] = [TileType::Basic, TileType::Furnace];

    pub fn def(self) -> TileDef {
        match self {

            TileType::Basic => TileDef {
                name: "Basic",
                cost: 10,
                color: Color::srgb(0.4, 0.7, 1.0),
                processor: None,
            },

            TileType::Furnace => TileDef {
                name: "Furnace",
                cost: 50,
                color: Color::srgb(1.0, 0.5, 0.2),
                processor: Some(Recipe {
                    inputs: &[(ItemType::IronOre, 1)],
                    outputs: &[(ItemType::IronPlate, 1)],
                    craft_time: 2.0,
                }),
            },
            
        }
    }
}
