use crate::aoc::common::input::{parse_int, split_2};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct GridPoint {
    pub x: i32,
    pub y: i32,
}

impl GridPoint {
    pub const fn new(x: i32, y: i32) -> GridPoint {
        GridPoint { x, y }
    }

    pub fn from_width(i: i32, width: i32) -> GridPoint {
        GridPoint {
            x: i % width,
            y: i / width,
        }
    }

    pub fn within_rect(&self, l: i32, t: i32, r: i32, b: i32) -> bool {
        self.x >= l && self.x < r && self.y >= t && self.y < b
    }

    pub fn from_str_coord(s: &str) -> GridPoint {
        let (x, y) = split_2(s, ",");
        GridPoint::new(parse_int(x), parse_int(y))
    }

    pub fn from_char(c: char) -> Option<GridPoint> {
        match c {
            '^' => Some(GridPoint::new(0, -1)),
            '>' => Some(GridPoint::new(1, 0)),
            'v' => Some(GridPoint::new(0, 1)),
            '<' => Some(GridPoint::new(-1, 0)),
            _ => None,
        }
    }

    pub fn linear(&self, width: i32) -> i32 {
        self.x + width * self.y
    }

    pub fn add(&self, p2: &Self) -> Self {
        GridPoint {
            x: self.x + p2.x,
            y: self.y + p2.y,
        }
    }

    pub fn sub(&self, p: &Self) -> Self {
        GridPoint {
            x: self.x - p.x,
            y: self.y - p.y,
        }
    }

    pub fn scale(&self, s: i32) -> Self {
        GridPoint {
            x: self.x * s,
            y: self.y * s,
        }
    }

    pub fn turn_trigo(&self) -> Self {
        GridPoint {
            x: -self.y,
            y: self.x,
        }
    }
}

pub const DIRECTIONS_8: [GridPoint; 8] = [
    GridPoint::new(1, 0),
    GridPoint::new(1, 1),
    GridPoint::new(0, 1),
    GridPoint::new(-1, 1),
    GridPoint::new(-1, 0),
    GridPoint::new(-1, -1),
    GridPoint::new(0, -1),
    GridPoint::new(1, -1),
];

pub const DIRECTIONS_4: [GridPoint; 4] = [
    GridPoint::new(1, 0),
    GridPoint::new(0, 1),
    GridPoint::new(-1, 0),
    GridPoint::new(0, -1),
];
