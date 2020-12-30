#[cfg(feature = "i32-coords")]
pub type Coord = i32;
#[cfg(not(feature = "i32-coords"))]
pub type Coord = usize;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Point(Coord, Coord);

impl Point {
    pub fn new(x: Coord, y: Coord) -> Self {
        Self(x, y)
    }
    pub fn x(&self) -> Coord {
        self.0
    }
    pub fn y(&self) -> Coord {
        self.1
    }
}
