use crate::aoc::common::grid_point::{GridPoint, DIRECTIONS_4};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
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

    pub fn index_mut(&mut self, p: &GridPoint) -> Option<&mut T> {
        if p.x < 0 || p.y < 0 || p.x >= self.width() || p.y >= self.height() {
            return None;
        }
        let w = self.width();
        Some(&mut self.rows[p.linear(w) as usize])
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

    pub fn map<U>(&self, f: impl Fn(&T) -> U) -> Grid<U> {
        Grid {
            rows: self.rows.iter().map(f).collect(),
            width: self.width,
        }
    }

    pub fn neighbors_4(&self, p: &GridPoint) -> Vec<(GridPoint, &T)> {
        DIRECTIONS_4
            .iter()
            .map(|d| p.add(&d))
            .flat_map(|n| self.index(&n).iter().map(|v| (n, *v)).collect::<Vec<_>>())
            .collect::<Vec<_>>()
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

    pub fn to_digit(&self) -> Grid<i32> {
        self.map(|c| c.to_digit(10).unwrap() as i32)
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                s.push_str(&format!("{}", self.index(&GridPoint::new(x, y)).unwrap()));
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}
