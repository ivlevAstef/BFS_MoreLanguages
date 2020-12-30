use super::array2d::Array2D;
use super::{Point, PointCoord};
use std::cell::RefCell;

const OFFSETS: &[(isize, isize)] = &[(1, 0), (-1, 0), (0, 1), (0, -1)];

fn neighbors(pos: Point) -> impl Iterator<Item = Point> {
    OFFSETS.iter().map(move |offset| {
        // TODO: out of bounds
        Point(
            (pos.0 as i32 + offset.0 as i32) as PointCoord,
            (pos.1 as i32 + offset.1 as i32) as PointCoord,
        )
    })
}

pub struct BFS {
    width: usize,
    height: usize,
    walls: Array2D<bool>,
    visited: RefCell<Array2D<bool>>,
    depth: RefCell<Array2D<i16>>,
}

impl BFS {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            walls: Self::generate_walls(width, height),
            visited: RefCell::new(Array2D::new_default(width, height)),
            depth: RefCell::new(Array2D::new_default(width, height)),
        }
    }

    fn generate_walls(width: usize, height: usize) -> Array2D<bool> {
        let mut walls = Array2D::filled_with(false, width, height);

        for index in 0..width as PointCoord {
            walls[Point(index, 0)] = true;
            walls[Point(index, height as PointCoord - 1)] = true;
        }

        for index in 0..height as PointCoord {
            walls[Point(0, index)] = true;
            walls[Point(width as PointCoord - 1, index)] = true;
        }

        let h = height / 10;
        let w = width / 10;

        for index in 0..height - h {
            let x = 2 * w;
            let y = index;
            walls[Point(x as PointCoord, y as PointCoord)] = true;
        }

        for index in h..height {
            let x = 8 * w;
            let y = index;
            walls[Point(x as PointCoord, y as PointCoord)] = true;
        }

        walls
    }

    pub fn path(&self, from: Point, to: Point) -> Vec<Point> {
        let mut visited = self.visited.borrow_mut();
        visited.fill(false);
        let mut depth = self.depth.borrow_mut();
        depth.fill(-1);

        visited[from] = true;
        depth[from] = 0;

        let mut queue = std::collections::VecDeque::with_capacity(self.width * self.height);
        queue.push_back(from);
        while let Some(pos) = queue.pop_front() {
            let length = depth[pos];
            if pos == to {
                break;
            }

            for new_pos in neighbors(pos) {
                if visited[new_pos] || self.walls[new_pos] {
                    continue;
                }
                visited[new_pos] = true;
                depth[new_pos] = length + 1;
                queue.push_back(new_pos);
            }
        }

        // not found
        if !visited[to] {
            return Vec::new();
        }

        let mut pos = to;
        let mut result = Vec::with_capacity(depth[pos] as usize);
        result.push(pos);
        while pos != from {
            let length = depth[pos];
            for prev_pos in neighbors(pos) {
                if depth[prev_pos] == length - 1 {
                    pos = prev_pos;
                    result.push(pos);
                    break; // push first found point
                }
            }
        }

        result.reverse();
        result
    }
}
