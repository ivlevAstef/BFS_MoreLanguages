use super::array2d::Array2D;
use super::queue::Queue;
use super::{point, Point};
use std::cell::RefCell;

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
        let mut from = Array2D::filled_with(Point::with_index(0), self.width, self.height);
        let mut queue = Queue::new();

        macro_rules! visited {
            ($pos:expr) => {
                from[$pos].index() != 0
            };
        }

        from[start] = start;
        queue.push(start);
        while let Some(pos) = queue.pop() {
            if pos == finish {
                break;
            }

            for new_pos in pos.neighbors(self.width, self.height) {
                if visited![new_pos] {
                    continue;
                }
                if self.walls[new_pos] {
                    continue;
                }
                from[new_pos] = pos;
                queue.push(new_pos);
            }
        }

        if !visited![finish] {
            return None;
        }

        let mut pos = finish;
        let mut result = Vec::new();
        result.push(pos);
        while pos != start {
            let prev_pos = from[pos];
            pos = prev_pos;
            result.push(pos);
        }

        result.reverse();
        Some(result)
    }
}
