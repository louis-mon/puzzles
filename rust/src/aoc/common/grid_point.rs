#[derive(Copy, Clone, Debug)]
pub struct GridPoint {
    pub x: i32,
    pub y: i32,
}

impl GridPoint {
    pub fn new(x: i32, y: i32) -> GridPoint {
        GridPoint { x, y }
    }

    pub fn from_width(i: i32, width: i32) -> GridPoint {
        GridPoint {
            x: i % width,
            y: i / width,
        }
    }

    pub fn add(&self, p2: &Self) -> Self {
        GridPoint {
            x: self.x + p2.x,
            y: self.y + p2.y,
        }
    }
}
