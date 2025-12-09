use crate::utils::point::Point;
use std::ops::{Index, IndexMut};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Grid<T> {
    pub width: i64,
    pub height: i64,
    pub bytes: Vec<T>,
}

impl Grid<u8> {
    #[inline]
    pub fn parse(input: &str) -> Self {
        let raw: Vec<_> = input.lines().map(str::as_bytes).collect();
        let width = raw[0].len() as i64;
        let height = raw.len() as i64;
        let mut bytes = Vec::with_capacity((width * height) as usize);
        raw.iter().for_each(|slice| bytes.extend_from_slice(slice));
        Grid {
            width,
            height,
            bytes,
        }
    }

    pub fn print(&self, focus: Option<Point>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point::new(x, y);

                if let Some(focus) = focus
                    && focus == point
                {
                    print!("@");
                    continue;
                }
                print!("{}", self[point] as char);
            }
            println!();
        }
        println!();
    }
}

impl<T: Clone> Grid<T> {
    pub fn new(width: i64, height: i64, seed: T) -> Self {
        let bytes = vec![seed; (width * height) as usize];

        Grid {
            width,
            height,
            bytes,
        }
    }

    pub fn in_bounds(&self, p: &Point) -> bool {
        p.x >= 0 && p.y >= 0 && p.x < self.width && p.y < self.height
    }

    #[inline]
    pub fn orthogonal_neighbors(&self, point: Point) -> Vec<Point> {
        point
            .orthogonal_neighbors()
            .into_iter()
            .filter(|p| self.in_bounds(p))
            .collect()
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Point) -> &Self::Output {
        &self.bytes[(self.width * index.y + index.x) as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.bytes[(self.width * index.y + index.x) as usize]
    }
}
