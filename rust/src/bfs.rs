use super::array2d::Array2D;
use super::queue::Queue;
use super::{point, Point};

pub struct BFS {
    width: usize,
    height: usize,
    walls: Array2D<bool>,
}

impl BFS {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            walls: Self::generate_walls(width, height),
        }
    }

    fn generate_walls(width: usize, height: usize) -> Array2D<bool> {
        let mut walls = Array2D::filled_with(false, width, height);

        for index in 0..width as point::Coord {
            walls[Point::new(index, 0)] = true;
            walls[Point::new(index, height as point::Coord - 1)] = true;
        }

        for index in 0..height as point::Coord {
            walls[Point::new(0, index)] = true;
            walls[Point::new(width as point::Coord - 1, index)] = true;
        }

        let h = height / 10;
        let w = width / 10;

        for index in 0..height - h {
            let x = 2 * w;
            let y = index;
            walls[Point::new(x as point::Coord, y as point::Coord)] = true;
        }

        for index in h..height {
            let x = 8 * w;
            let y = index;
            walls[Point::new(x as point::Coord, y as point::Coord)] = true;
        }

        walls
    }

    pub fn path(&self, start: Point, finish: Point) -> Option<Vec<Point>> {
        if start == finish {
            return Some(vec![start]);
        }

        #[derive(Debug, Copy, Clone)]
        struct CellState {
            #[cfg(feature = "hacky-cell-state")]
            from: Point, // Point (0, 0) contains wall so is never put in the queue, so is used instead of None
            #[cfg(not(feature = "hacky-cell-state"))]
            from: Option<Point>,
        }

        impl CellState {
            pub fn unvisited() -> Self {
                Self {
                    #[cfg(feature = "hacky-cell-state")]
                    from: Point::with_index(0),
                    #[cfg(not(feature = "hacky-cell-state"))]
                    from: None,
                }
            }
            pub fn new_from(from: Point) -> Self {
                Self {
                    #[cfg(feature = "hacky-cell-state")]
                    from,
                    #[cfg(not(feature = "hacky-cell-state"))]
                    from: Some(from),
                }
            }
            pub fn is_visited(&self) -> bool {
                #[cfg(feature = "hacky-cell-state")]
                return self.from.index() != 0;
                #[cfg(not(feature = "hacky-cell-state"))]
                return self.from.is_some();
            }
            pub fn from(&self) -> Point {
                #[cfg(feature = "hacky-cell-state")]
                return self.from;
                #[cfg(not(feature = "hacky-cell-state"))]
                return self.from.unwrap();
            }
        }

        let mut cell_states: Array2D<CellState> =
            Array2D::filled_with(CellState::unvisited(), self.width, self.height);

        let mut queue = Queue::new();

        cell_states[start] = CellState::new_from(start);
        queue.push(start);
        while let Some(pos) = queue.pop() {
            if pos == finish {
                break;
            }

            for new_pos in pos.neighbors(self.width, self.height) {
                if cell_states[new_pos].is_visited() {
                    continue;
                }
                if self.walls[new_pos] {
                    continue;
                }
                cell_states[new_pos] = CellState::new_from(pos);
                queue.push(new_pos);
            }
        }

        if !cell_states[finish].is_visited() {
            return None;
        }

        let mut pos = finish;
        let mut result = Vec::new();
        result.push(pos);
        while pos != start {
            pos = cell_states[pos].from();
            result.push(pos);
        }

        result.reverse();
        Some(result)
    }
}
