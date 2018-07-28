pub mod feature;

use self::feature::{Corridor, Room};
use super::Point;
use std::collections::BTreeMap;

pub enum CellType {
    Door,
    Empty,
    Wall,
}

pub enum DirectionX {
    Left,
    Right,
}

pub enum DirectionY {
    Down,
    Up,
}

pub struct Map {
    width: u32,
    height: u32,
    pub map: BTreeMap<Point, CellType>,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        Map {
            width,
            height,
            map: BTreeMap::new(),
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

pub struct Dungeon {
    pub map: Map,
    rooms: Vec<Room>,
    corridors: Vec<Corridor>,
}

impl Dungeon {
    pub fn width(&self) -> u32 {
        self.map.width()
    }

    pub fn height(&self) -> u32 {
        self.map.height()
    }
}
