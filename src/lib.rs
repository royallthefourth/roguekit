pub mod map;

use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl std::cmp::Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        if self.y < other.y {
            Ordering::Less
        } else if self.y == other.y {
            if self.x < other.y {
                Ordering::Less
            } else if self.x == other.x {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        } else {
            Ordering::Greater
        }
    }
}

impl std::cmp::PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
