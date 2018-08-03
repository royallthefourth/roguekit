extern crate rand;

use self::rand::{thread_rng, Rng};
use map::{CellType, DirectionX, DirectionY};
use std::cmp;
use Point;

trait Feature {
    fn rectify(&mut self, is_wall: fn(&Point) -> bool, can_dig: fn(&Point) -> bool) -> bool;
    fn create(&self, dig: fn(&Point, CellType));
}

pub struct Corridor {
    start: Point,
    end: Point,
    ends_with_wall: bool,
}

pub struct CorridorOptions {
    pub min_length: i32,
    pub max_length: i32,
}

impl Feature for Corridor {
    fn rectify(&mut self, is_wall: fn(&Point) -> bool, can_dig: fn(&Point) -> bool) -> bool {
        let dist_x = self.end.x - self.start.x;
        let dist_y = self.end.y - self.start.y;
        let mut length = 1 + cmp::max(dist_x.abs(), dist_y.abs());
        let dir_x: i32;
        let dir_y: i32;

        if dist_x != 0 {
            dir_x = dist_x / dist_x.abs();
        } else {
            dir_x = 0;
        }

        if dist_y != 0 {
            dir_y = dist_y / dist_y.abs();
        } else {
            dir_y = 0;
        }

        let nx = dir_y;
        let ny = -dir_x;

        for i in 0..length {
            let x = self.start.x + i * dir_x;
            let y = self.start.y + i * dir_y;

            if !can_dig(&Point { x, y })
                || !is_wall(&Point {
                    x: x + nx,
                    y: y + ny,
                })
                || !is_wall(&Point {
                    x: x - nx,
                    y: y - ny,
                }) {
                length = i;
                self.end.x = x - dir_x;
                self.end.y = y - dir_y;
                break;
            }
        }

        if length == 0 {
            return false;
        }

        if length == 1 && is_wall(&Point {
            x: self.end.x + dir_x,
            y: self.end.y + dir_y,
        }) {
            return false;
        }

        let bad_corner = !(is_wall(&Point {
            x: (self.end.x + dir_x + nx),
            y: (self.end.y + dir_y + ny),
        }) || is_wall(&Point {
            x: (self.end.x + dir_x - nx),
            y: (self.end.y + dir_y - ny),
        }));
        self.ends_with_wall = is_wall(&Point {
            x: (self.end.x + dir_x),
            y: (self.end.y + dir_y),
        });
        if bad_corner && self.ends_with_wall {
            return false;
        }

        true
    }

    fn create(&self, dig: fn(&Point, CellType)) {
        let dist_x = self.end.x - self.start.x;
        let dist_y = self.end.y - self.start.y;
        let length = 1 + cmp::max(dist_x.abs(), dist_y.abs());
        let dir_x: i32;
        let dir_y: i32;

        if dist_x != 0 {
            dir_x = dist_x / dist_x.abs();
        } else {
            dir_x = 0;
        }

        if dist_y != 0 {
            dir_y = dist_y / dist_y.abs();
        } else {
            dir_y = 0;
        }

        for i in 0..length {
            let x = self.start.x + i * dir_x;
            let y = self.start.y + i * dir_y;
            dig(&Point { x, y }, CellType::Empty)
        }
    }
}

impl Corridor {
    fn create_random_at(
        top_left: Point,
        dx: DirectionX,
        dy: DirectionY,
        options: &CorridorOptions,
    ) -> Corridor {
        let mut rng = thread_rng();
        let length: i32 = rng.gen_range(options.min_length, options.max_length);
        Corridor {
            start: top_left.clone(),
            end: Point {
                x: top_left.x + dx as i32 * length,
                y: top_left.y + dy as i32 * length,
            },
            ends_with_wall: false,
        }
    }

    fn create_priority_walls(&mut self, priority_wall: fn(&Point)) {
        if self.ends_with_wall {
            return;
        }

        let dist_x = self.end.x - self.start.x;
        let dist_y = self.end.y - self.start.y;
        let dir_x: i32;
        let dir_y: i32;

        if dist_x != 0 {
            dir_x = dist_x / dist_x.abs();
        } else {
            dir_x = 0;
        }

        if dist_y != 0 {
            dir_y = dist_y / dist_y.abs();
        } else {
            dir_y = 0;
        }

        let nx = dir_y;
        let ny = -dir_x;

        priority_wall(&Point {
            x: (self.end.x + dir_x),
            y: (self.end.y + dir_y),
        });
        priority_wall(&Point {
            x: (self.end.x + nx),
            y: (self.end.y + ny),
        });
        priority_wall(&Point {
            x: (self.end.x - nx),
            y: (self.end.y - ny),
        })
    }
}

pub struct Room {
    top_left: Point,
    bottom_right: Point,
    doors: Vec<Point>,
}

pub struct RoomOptions {
    min_width: i32,
    max_width: i32,
    min_height: i32,
    max_height: i32,
}

impl Feature for Room {
    fn rectify(&mut self, is_wall: fn(&Point) -> bool, can_dig: fn(&Point) -> bool) -> bool {
        let left = self.top_left.x - 1;
        let right = self.bottom_right.x + 1;
        let top = self.top_left.y - 1;
        let bottom = self.bottom_right.y + 1;

        for x in left..=right {
            for y in top..=bottom {
                if (x == left || x == right || y == top || y == bottom) && !is_wall(&Point { x, y })
                {
                    return false;
                } else if !can_dig(&Point { x, y }) {
                    return false;
                }
            }
        }

        true
    }

    fn create(&self, dig: fn(&Point, CellType)) {
        let left = self.top_left.x - 1;
        let right = self.bottom_right.x + 1;
        let top = self.top_left.y - 1;
        let bottom = self.bottom_right.y + 1;

        let mut fill_type: CellType;
        for x in left..=right {
            for y in top..=bottom {
                if self.doors.contains(&Point { x, y }) {
                    fill_type = CellType::Door
                } else if x == left || x == right || y == top || y == bottom {
                    fill_type = CellType::Wall
                } else {
                    fill_type = CellType::Empty
                }
                dig(&Point { x, y }, fill_type)
            }
        }
    }
}

impl Room {
    pub fn create_random_at(
        door: &Point,
        dx: DirectionX,
        dy: DirectionY,
        options: &RoomOptions,
    ) -> Result<Room, &'static str> {
        if dx == DirectionX::None && dy == DirectionY::None {
            return Err("A room must have either an X or a Y direction");
        }
        let mut rng = thread_rng();
        let height: i32 = rng.gen_range(options.min_height, options.max_height);
        let width: i32 = rng.gen_range(options.min_width, options.max_width);
        let scale: f32 = rng.gen_range(0., 1.);
        let x2 = door.x - (width as f32 * scale).floor() as i32;
        let y2 = door.y - (height as f32 * scale).floor() as i32;
        let mut d: Vec<Point> = Vec::new();
        d.push(door.clone());

        let r = match (dx, dy) {
            (DirectionX::Right, _) => Room {
                top_left: Point {
                    x: (door.x + 1),
                    y: y2,
                },
                bottom_right: Point {
                    x: (door.x + width),
                    y: (y2 + height - 1),
                },
                doors: d,
            },
            (DirectionX::Left, _) => Room {
                top_left: Point {
                    x: door.x - width,
                    y: y2,
                },
                bottom_right: Point {
                    x: door.x - 1,
                    y: y2 + height - 1,
                },
                doors: d,
            },
            (_, DirectionY::Down) => Room {
                top_left: Point {
                    x: x2,
                    y: door.y + 1,
                },
                bottom_right: Point {
                    x: x2 + width - 1,
                    y: door.y + height,
                },
                doors: d,
            },
            (_, _) => Room {
                top_left: Point {
                    x: x2,
                    y: door.y - height,
                },
                bottom_right: Point {
                    x: x2 + width - 1,
                    y: door.y - 1,
                },
                doors: d,
            },
        };

        Ok(r)
    }

    fn create_random_center(center: &Point, options: &RoomOptions) -> Room {
        let mut rng = thread_rng();
        let width = rng.gen_range(options.min_width, options.max_width);
        let height: i32 = rng.gen_range(options.min_height, options.max_height);

        let start = Point {
            x: center.x - (width as f32 * rng.gen_range(0., 1.)).floor() as i32,
            y: center.y - (height as f32 * rng.gen_range(0., 1.)).floor() as i32,
        };

        let end = Point {
            x: start.x + width - 1,
            y: start.y + height - 1,
        };

        Room {
            top_left: start,
            bottom_right: end,
            doors: Vec::new(),
        }
    }

    fn create_random(avail_height: &i32, avail_width: &i32, options: &RoomOptions) -> Room {
        let mut rng = thread_rng();
        let width = rng.gen_range(options.min_width, options.max_width);
        let height: i32 = rng.gen_range(options.min_height, options.max_height);

        let left = avail_width - width - 1;
        let top = avail_height - height - 1;

        let start = Point {
            x: 1 + (left as f32 * rng.gen_range(0., 1.)).floor() as i32,
            y: 1 + (top as f32 * rng.gen_range(0., 1.)).floor() as i32,
        };

        let end = Point {
            x: start.x + width - 1,
            y: start.y + height - 1,
        };

        Room {
            top_left: start,
            bottom_right: end,
            doors: Vec::new(),
        }
    }

    fn add_door(&mut self, door: Point) {
        if !self.doors.contains(&door) {
            self.doors.push(door)
        }
    }

    fn doors(&self) -> &Vec<Point> {
        &self.doors
    }

    fn clear_doors(&mut self) {
        self.doors = Vec::new();
    }

    fn add_doors(&mut self, is_wall: fn(&Point) -> bool) {
        let left = self.top_left.x - 1;
        let right = self.bottom_right.x + 1;
        let top = self.top_left.y - 1;
        let bottom = self.bottom_right.y + 1;

        for x in left..=right {
            for y in top..=bottom {
                if x != left && x != right && y != top && y != bottom {
                    continue;
                }
                if is_wall(&Point { x, y }) {
                    continue;
                }
                self.add_door(Point { x, y });
            }
        }
    }

    fn center(&self) -> Point {
        Point {
            x: (self.top_left.x + self.bottom_right.x) / 2,
            y: (self.top_left.x + self.bottom_right.x) / 2,
        }
    }

    fn left(&self) -> i32 {
        self.top_left.x
    }

    fn right(&self) -> i32 {
        self.bottom_right.x
    }

    fn top(&self) -> i32 {
        self.top_left.y
    }

    fn bottom(&self) -> i32 {
        self.bottom_right.y
    }
}
