use hashbrown::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub const ORIGIN: Point = Point::new(0, 0);
pub const UP: Point = Point::new(0, -1);
pub const DOWN: Point = Point::new(0, 1);
pub const LEFT: Point = Point::new(-1, 0);
pub const RIGHT: Point = Point::new(1, 0);
pub const ORTHOGONAL: [Point; 4] = [UP, DOWN, LEFT, RIGHT];
pub const NE: Point = Point::new(1, -1);
pub const SE: Point = Point::new(1, 1);
pub const SW: Point = Point::new(-1, 1);
pub const NW: Point = Point::new(-1, -1);
// Reading order
pub const DIAGONAL: [Point; 8] = [NW, UP, NE, LEFT, RIGHT, SW, DOWN, SE];

impl Point {
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    #[must_use]
    pub fn clockwise(self) -> Self {
        Point::new(-self.y, self.x)
    }

    #[inline]
    #[must_use]
    pub fn counter_clockwise(self) -> Self {
        Point::new(self.y, -self.x)
    }

    #[inline]
    pub fn orthogonal_neighbors(self) -> Vec<Self> {
        vec![
            Point::new(self.x + 1, self.y),
            Point::new(self.x, self.y + 1),
            Point::new(self.x - 1, self.y),
            Point::new(self.x, self.y - 1),
        ]
    }

    #[inline]
    pub fn manhattan(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    #[inline]
    pub fn distance(self, other: Self) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }

    #[inline]
    pub fn signum(self, other: Self) -> Self {
        Point::new((self.x - other.x).signum(), (self.y - other.y).signum())
    }

    #[inline]
    pub fn wrap(self, size: &Self) -> Self {
        Point::new((self.x + size.x) % size.x, (self.y + size.y) % size.y)
    }

    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Point::new(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y))
    }
}

impl Hash for Point {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.x as u32);
        state.write_u32(self.y as u32);
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: i32) -> Self {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

pub fn get_point_vec_bounds(vecs: &[Point]) -> (Point, Point) {
    let mut x_min = i32::MAX;
    let mut x_max = i32::MIN;
    let mut y_min = i32::MAX;
    let mut y_max = i32::MIN;

    for vec in vecs.iter() {
        x_min = x_min.min(vec.x);
        x_max = x_max.max(vec.x);
        y_min = y_min.min(vec.y);
        y_max = y_max.max(vec.y);
    }

    (Point::new(x_min, y_min), Point::new(x_max, y_max))
}

pub fn print_point_vec(vecs: &[Point]) {
    let (min, max) = get_point_vec_bounds(vecs);

    let w = (max.x - min.x) as usize + 1;
    let h = (max.y - min.y) as usize + 1;
    let mut map = vec![vec![false; w]; h];

    for vec in vecs {
        let x = (vec.x - min.x) as usize;
        let y = (vec.y - min.y) as usize;
        map[y][x] = true;
    }

    for row in &map {
        for &cell in row {
            let c = match cell {
                true => '#',
                false => '.',
            };

            print!("{c}");
        }
        println!();
    }
    println!();
}

pub fn print_point_set(s: &HashSet<Point>) {
    let vecs: Vec<Point> = s.iter().cloned().collect();
    print_point_vec(&vecs);
}
