use crate::aoc::common::grid_point::GridPoint;

#[derive(Debug)]
pub struct Grid<T> {
    rows: Vec<T>,
    width: usize,
}

impl<T> Grid<T> {
    pub fn width(&self) -> i32 {
        self.width as i32
    }

    pub fn height(&self) -> i32 {
        (self.rows.len() / self.width) as i32
    }
    pub fn index(&self, p: &GridPoint) -> Option<&T> {
        if p.x < 0 || p.y < 0 || p.x >= self.width() || p.y >= self.height() {
            return None;
        }
        Some(&self.rows[p.linear(self.width()) as usize])
    }

    pub fn iter_points(&self) -> impl Iterator<Item = (GridPoint, &T)> {
        self.rows
            .iter()
            .enumerate()
            .map(|(i, v)| (GridPoint::from_width(i as i32, self.width()), v))
    }

    pub fn find_point(&self, t: T) -> Option<GridPoint>
    where
        T: Eq,
    {
        self.iter_points().find(|(p, v)| **v == t).map(|x| x.0)
    }
}

impl Grid<char> {
    pub fn from_str(s: &str) -> Grid<char> {
        let width = s.find("\n").unwrap();
        Grid {
            rows: s.chars().filter(|c| *c != '\n').collect(),
            width,
        }
    }
}
