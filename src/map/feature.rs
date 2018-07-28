use map::{CellType, DirectionX, DirectionY, Map};
use std::collections::BTreeMap;
use Point;

trait Feature {
    fn is_valid(is_wall: fn(Point) -> bool, can_dig: fn(Point) -> bool) -> bool;
    fn create(dig: fn(Point, CellType, &Map, &BTreeMap<Point, CellType>)) -> Self;
    fn create_random_at(
        top_left: Point,
        direction_x: DirectionX,
        direction_y: DirectionY,
        min_width: u32,
        max_width: u32,
    ) -> Self;
}

pub struct Corridor {
    top_left: Point,
    bottom_right: Point,
    ends_with_wall: bool
}

impl Feature for Corridor {
    fn is_valid(is_wall: fn(Point) -> bool, can_dig: fn(Point) -> bool) -> bool {

    }
}

pub struct Room {
    top_left: Point,
    bottom_right: Point,
    doors: BTreeMap<Point, u32>,
}

// impl Room {
//     // TODO
//     pub fn (&self) -> u32 {

//     }
// }
