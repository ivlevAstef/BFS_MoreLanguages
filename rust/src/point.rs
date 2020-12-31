pub const MAX_COORD: usize = 100;

pub type Coord = i8;

pub type IndexType = i16;

#[test]
fn test_limits() {
    assert!(MAX_COORD <= Coord::MAX as usize);
    assert!(MAX_COORD * MAX_COORD <= IndexType::MAX as usize);
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Point {
    #[cfg(not(feature = "point-index"))]
    x: Coord,
    #[cfg(not(feature = "point-index"))]
    y: Coord,
    #[cfg(feature = "point-index")]
    index: IndexType,
}

impl Point {
    pub fn new(x: Coord, y: Coord) -> Self {
        Self {
            #[cfg(feature = "point-index")]
            index: x as IndexType + y as IndexType * MAX_COORD as IndexType,
            #[cfg(not(feature = "point-index"))]
            x,
            #[cfg(not(feature = "point-index"))]
            y,
        }
    }
    pub fn with_index(index: IndexType) -> Self {
        Self {
            #[cfg(feature = "point-index")]
            index,
            #[cfg(not(feature = "point-index"))]
            x: (index % MAX_COORD as IndexType) as Coord,
            #[cfg(not(feature = "point-index"))]
            y: (index / MAX_COORD as IndexType) as Coord,
        }
    }
    pub fn x(&self) -> Coord {
        #[cfg(feature = "point-index")]
        return (self.index % MAX_COORD as IndexType) as Coord;
        #[cfg(not(feature = "point-index"))]
        return self.x;
    }
    pub fn y(&self) -> Coord {
        #[cfg(feature = "point-index")]
        return (self.index / MAX_COORD as IndexType) as Coord;
        #[cfg(not(feature = "point-index"))]
        return self.y;
    }
    pub fn index(&self) -> IndexType {
        #[cfg(feature = "point-index")]
        return self.index;
        #[cfg(not(feature = "point-index"))]
        return self.x as IndexType + self.y as IndexType * MAX_COORD as IndexType;
    }

    pub fn neighbors(self, width: usize, height: usize) -> impl Iterator<Item = Self> {
        type DiffType = i16;
        const OFFSETS: &[(DiffType, DiffType)] = &[(1, 0), (-1, 0), (0, 1), (0, -1)];
        return OFFSETS.iter().filter_map(move |offset| {
            if cfg!(feature = "neighbors-ignore-bounds") {
                Some(if cfg!(feature = "point-index") {
                    Self::with_index(
                        (self.index() as DiffType + (offset.0 + offset.1 * MAX_COORD as DiffType))
                            as IndexType,
                    )
                } else {
                    Self::new(
                        (self.x() as DiffType + offset.0) as Coord,
                        (self.y() as DiffType + offset.1) as Coord,
                    )
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
