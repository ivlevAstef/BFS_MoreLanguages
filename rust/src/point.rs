pub const MAX_WIDTH: usize = 100;

pub type Coord = i32;

pub type IndexType = u16;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Point {
    index: IndexType,
}

impl Point {
    pub fn new(x: Coord, y: Coord) -> Self {
        Self {
            index: x as IndexType + y as IndexType * MAX_WIDTH as IndexType,
        }
    }
    pub fn x(&self) -> Coord {
        (self.index % MAX_WIDTH as IndexType) as Coord
    }
    pub fn y(&self) -> Coord {
        (self.index / MAX_WIDTH as IndexType) as Coord
    }
    pub fn index(&self) -> usize {
        self.index as usize
    }

    pub fn neighbors(self, width: usize, height: usize) -> impl Iterator<Item = Self> {
        const OFFSETS: &[(isize, isize)] = &[(1, 0), (-1, 0), (0, 1), (0, -1)];
        return OFFSETS.iter().filter_map(move |offset| {
            if cfg!(feature = "neighbors-ignore-bounds") {
                Some(Self {
                    index: (self.index as isize + offset.0 + offset.1 * MAX_WIDTH as isize)
                        as IndexType,
                })
            } else {
                let p = (
                    self.x() as i32 + offset.0 as i32,
                    self.y() as i32 + offset.1 as i32,
                );
                if p.0 < 0 || p.1 < 0 || p.0 >= width as i32 || p.1 >= height as i32 {
                    return None;
                }
                Some(Self::new(p.0 as Coord, p.1 as Coord))
            }
        });
    }
}
