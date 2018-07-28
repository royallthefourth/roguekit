pub mod map;

use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl std::cmp::Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        if self.y < other.y {
            return Ordering::Less;
        } else if self.y == other.y {
            if self.x < other.y {
                return Ordering::Less;
            } else if self.x == other.x {
                return Ordering::Equal;
            } else {
                return Ordering::Greater;
            }
        } else {
            return Ordering::Greater;
        }
    }
}

impl std::cmp::PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
