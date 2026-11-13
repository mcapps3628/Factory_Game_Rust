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

/// Direction definition for ports
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortDirection {
    North,
    East,
    South,
    West,
}

/// Port definition
#[derive(Clone, Debug)]
pub struct PortDef {
    pub local_coords: IVec2,
    pub direction: PortDirection, // Which way the port faces/connects
    pub is_output: bool,
}

/// Everything that is true of a tile *type* (not of one placed instance).
/// A tile gets behaviors by having them set to Some(..).
pub struct TileDef {
    pub name: &'static str,
    pub cost: u32,
    pub texture_path: &'static str,
    pub size: IVec2,
    pub ports: Vec<PortDef>,
    pub processor: Option<Recipe>,
    pub conveyor_speed: Option<f32>, // tiles per second
    pub producer: Option<ProducerDef>,
}

impl TileDef {
    /// Starting point for definitions: every behavior off.
    const BASE: TileDef = TileDef {
        name: "",
        cost: 0,
        texture_path: "",
        size: IVec2::new(1,1),
        ports: Vec::new(),
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
                texture_path: "tiles/basic.png", //TODO: Change to image.
                ..TileDef::BASE
            },

            TileType::Furnace => TileDef {
                name: "Furnace",
                cost: 50,
                texture_path: "tiles/furnace.png",
                processor: Some(Recipe {
                    inputs: &[(ItemType::IronOre, 1)],
                    outputs: &[(ItemType::IronPlate, 1)],
                    craft_time: 2.0,
                }),
                size: IVec2::new(1, 2),
                ..TileDef::BASE
            },

            TileType::Conveyor => TileDef {
                name: "Conveyor",
                cost: 2,
                texture_path: "tiles/conveyor.png",
                conveyor_speed: Some(1.5),
                ..TileDef::BASE
            },

            TileType::Miner => TileDef {
                name: "Miner",
                cost: 40,
                texture_path: "tiles/miner.png",
                producer: Some(ProducerDef {
                    item: ItemType::IronOre,
                    interval: 1.5,
                }),
                ..TileDef::BASE
            },
        }
    }
}