use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Direction { North, #[default] East, South, West }

impl Direction {
    pub fn delta(self) -> IVec2 {
        match self {
            Direction::North => IVec2::Y,
            Direction::East => IVec2::X,
            Direction::South => IVec2::NEG_Y,
            Direction::West => IVec2::NEG_X,
        }
    }
    pub fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
    pub fn rotate_cw(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn quarter_turns(self) -> u8 {
        match self {
            Direction::East => 0,
            Direction::South => 1,
            Direction::West => 2,
            Direction::North => 3,
        }
    }

    pub fn turned_by(self, rotation: Direction) -> Direction {
        (0..rotation.quarter_turns()).fold(self, |d, _| d.rotate_cw())
    }

    pub fn rotate_cell(local: IVec2, size: IVec2, rotation: Direction) -> IVec2 {
        let (mut cell, mut size) = (local, size);
        for _ in 0..rotation.quarter_turns() {
            cell = IVec2::new(cell.y, size.x - 1 - cell.x);
            size = IVec2::new(size.y, size.x);
        }
        cell
    }

    pub fn rotate_size(size: IVec2, rotation: Direction) -> IVec2 {
        if rotation.quarter_turns() % 2 == 1 {
            IVec2::new(size.y, size.x)
        } else {
            size
        }
    }
}