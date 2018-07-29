pub mod feature;

use self::feature::{Corridor, Room};
use super::Point;
use std::collections::BTreeMap;

pub enum CellType {
    Empty = 0,
    Wall = 1,
    Door = 2,
}

pub enum DirectionX {
    Left = -1,
    Right = 1,
    None = 0,
}

pub enum DirectionY {
    Down = 1,
    Up = -1,
    None = 0,
}

pub struct Map {
    width: i32,
    height: i32,
    pub map: BTreeMap<Point, CellType>,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Map {
        Map {
            width,
            height,
            map: BTreeMap::new(),
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }
}

pub struct Dungeon {
    pub map: Map,
    rooms: Vec<Room>,
    corridors: Vec<Corridor>,
}

impl Dungeon {
    pub fn width(&self) -> i32 {
        self.map.width()
    }

    pub fn height(&self) -> i32 {
        self.map.height()
    }
}
