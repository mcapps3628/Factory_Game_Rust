use bevy::prelude::*;

use crate::item::ItemType;
use crate::tile::direction::Direction;
use crate::tile::port::{PortDef, PortKind};

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

/// Everything that is true of a tile type (not of one placed instance).
/// A tile gets behaviors by having them set to Some(..).
pub struct TileDef {
    pub name: &'static str,
    pub cost: u32,
    pub texture_path: &'static str,
    pub size: IVec2,
    pub ports: Vec<PortDef>,
    pub processor: Option<Recipe>,
    pub producer: Option<ProducerDef>,
    pub conveyor_speed: Option<f32>,
    pub seller: bool,
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
        producer: None,
        conveyor_speed: None,
        seller: false,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileType {
    Basic,
    Furnace,
    IronMine,
    CoalMine,
    Belt,
    WireMaker,
    Seller,
}

impl TileType {
    /// Every tile type.
    pub const ALL: [TileType; 7] = [
        TileType::Basic,
        TileType::Furnace,
        TileType::IronMine,
        TileType::CoalMine,
        TileType::Belt,
        TileType::WireMaker,
        TileType::Seller,
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
                    inputs: &[(ItemType::IronOre, 1), (ItemType::Coal, 1)],
                    outputs: &[(ItemType::IronIngot, 1)],
                    craft_time: 4.0,
                }),
                ports: vec![
                    PortDef { local: IVec2::ZERO, facing: Direction::West, kind: PortKind::Input },
                    PortDef { local: IVec2::new(0, 1), facing: Direction::West, kind: PortKind::Input },
                    PortDef { local: IVec2::new(1, 0), facing: Direction::East, kind: PortKind::Output },
                ],
                size: IVec2::new(2, 2),
                ..TileDef::BASE
            },

            TileType::IronMine => TileDef {
                name: "Iron Mine",
                cost: 40,
                texture_path: "tiles/miner.png",
                producer: Some(ProducerDef {
                    item: ItemType::IronOre,
                    interval: 4.0,
                }),
                ports: vec![
                    PortDef { local: IVec2::new(0, 0), facing: Direction::South, kind: PortKind::Output },
                ],
                size: IVec2::new(1, 2),
                ..TileDef::BASE
            },

            TileType::CoalMine => TileDef {
                name: "Coal Mine",
                cost: 20,
                texture_path: "tiles/miner.png",
                producer: Some(ProducerDef {
                    item: ItemType::Coal,
                    interval: 2.0,
                }),
                ports: vec![
                    PortDef { local: IVec2::new(0, 0), facing: Direction::South, kind: PortKind::Output },
                ],
                size: IVec2::new(1, 2),
                ..TileDef::BASE
            },

            TileType::Belt => TileDef {
                name: "Belt",
                cost: 5,
                texture_path: "tiles/conveyor.png",
                conveyor_speed: Some(1.0),
                ports: vec![
                    PortDef { local: IVec2::ZERO, facing: Direction::West,  kind: PortKind::Input },
                    PortDef { local: IVec2::ZERO, facing: Direction::North, kind: PortKind::Input },
                    PortDef { local: IVec2::ZERO, facing: Direction::South, kind: PortKind::Input },
                    PortDef { local: IVec2::ZERO, facing: Direction::East,  kind: PortKind::Output },
                ],
                ..TileDef::BASE
            },

            TileType::WireMaker => TileDef {
                name: "Wire Maker",
                cost: 50,
                texture_path: "tiles/basic.png",
                processor: Some(Recipe {
                    inputs: &[(ItemType::IronIngot, 1)],
                    outputs: &[(ItemType::IronWire, 1)],
                    craft_time: 2.0,
                }),
                ports: vec![
                    PortDef { local: IVec2::ZERO, facing: Direction::West, kind: PortKind::Input },
                    PortDef { local: IVec2::ZERO, facing: Direction::East, kind: PortKind::Output },
                ],
                size: IVec2::new(1, 2),
                ..TileDef::BASE
            },

            TileType::Seller => TileDef {
                name: "Seller",
                cost: 30,
                texture_path: "tiles/basic.png",
                ports: vec![
                    PortDef { local: IVec2::ZERO, facing: Direction::West,  kind: PortKind::Input },
                ],
                size: IVec2::new(2, 2),
                seller: true,
                ..TileDef::BASE
            },
        }
    }
}