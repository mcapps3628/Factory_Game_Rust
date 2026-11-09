use bevy::prelude::*;

use crate::item::ItemType;

/// A crafting recipe. Data is shared among every tile.
#[derive(Debug, Clone, Copy)]
pub struct Recipe {
    pub inputs: &'static [(ItemType, u32)],
    pub outputs: &'static [(ItemType, u32)],
    pub craft_time: f32, // seconds
}

/// Item Producing Tile
#[derive(Debug, Clone, Copy)]
pub struct ProducerDef {
    pub item: ItemType,
    pub interval: f32, // seconds per item
}

/// Everything that is true of a tile *type* (not of one placed instance).
/// A tile gets behaviors by having them set to Some(..).
pub struct TileDef {
    pub name: &'static str,
    pub cost: u32,
    pub color: Color,
    pub processor: Option<Recipe>,
    pub conveyor_speed: Option<f32>, // tiles per second
    pub producer: Option<ProducerDef>,
}

impl TileDef {
    /// Starting point for definitions: every behavior off.
    const BASE: TileDef = TileDef {
        name: "",
        cost: 0,
        color: Color::WHITE,
        processor: None,
        conveyor_speed: None,
        producer: None,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileType {
    Basic,
    Furnace,
    Conveyor,
    Miner,
}

impl TileType {
    /// Every tile type.
    pub const ALL: [TileType; 4] = [
        TileType::Basic,
        TileType::Furnace,
        TileType::Conveyor,
        TileType::Miner,
    ];

    pub fn def(self) -> TileDef {
        match self {

            TileType::Basic => TileDef {
                name: "Basic",
                cost: 10,
                color: Color::srgb(0.4, 0.7, 1.0),
                ..TileDef::BASE
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
                ..TileDef::BASE
            },

            TileType::Conveyor => TileDef {
                name: "Conveyor",
                cost: 2,
                color: Color::srgb(0.25, 0.25, 0.30),
                conveyor_speed: Some(1.5),
                ..TileDef::BASE
            },

            TileType::Miner => TileDef {
                name: "Miner",
                cost: 40,
                color: Color::srgb(0.55, 0.35, 0.20),
                producer: Some(ProducerDef {
                    item: ItemType::IronOre,
                    interval: 1.5,
                }),
                ..TileDef::BASE
            },
        }
    }
}