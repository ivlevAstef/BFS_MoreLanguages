use super::array2d::Array2D;
use super::{Point, PointCoord};
use std::cell::RefCell;

fn neighbors(pos: Point, width: usize, height: usize) -> impl Iterator<Item = Point> {
    const OFFSETS: &[(isize, isize)] = &[(1, 0), (-1, 0), (0, 1), (0, -1)];
    return OFFSETS.iter().filter_map(move |offset| {
        let p = (
            pos.0 as i32 + offset.0 as i32,
            pos.1 as i32 + offset.1 as i32,
        );
        if !cfg!(feature = "neighbors-ignore-bounds") {
            if p.0 < 0 || p.1 < 0 || p.0 >= width as i32 || p.1 >= height as i32 {
                return None;
            }
        }
        Some(Point(p.0 as PointCoord, p.1 as PointCoord))
    });
}

struct State {
    visited: Array2D<bool>,
    depth: Array2D<i16>,
}

impl State {
    fn new(width: usize, height: usize) -> Self {
        Self {
            visited: Array2D::filled_with(false, width, height),
            depth: Array2D::filled_with(-1, width, height),
        }
    }
    fn clear(&mut self) {
        self.visited.fill(false);
        self.depth.fill(-1);
    }
}

pub struct BFS {
    width: usize,
    height: usize,
    walls: Array2D<bool>,
    state: RefCell<State>,
}

impl BFS {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            walls: Self::generate_walls(width, height),
            state: RefCell::new(State::new(width, height)),
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

    pub fn path(&self, from: Point, to: Point) -> Option<Vec<Point>> {
        let mut state = self.state.borrow_mut();
        state.clear();

        state.visited[from] = true;
        state.depth[from] = 0;

        let mut queue = std::collections::VecDeque::with_capacity(self.width * self.height);
        queue.push_back(from);
        while let Some(pos) = queue.pop_front() {
            let length = state.depth[pos];
            if pos == to {
                break;
            }

            for new_pos in neighbors(pos, self.width, self.height) {
                if state.visited[new_pos] || self.walls[new_pos] {
                    continue;
                }
                state.visited[new_pos] = true;
                state.depth[new_pos] = length + 1;
                queue.push_back(new_pos);
            }
        }

        // not found
        if !state.visited[to] {
            return None;
        }

        let mut pos = to;
        let mut result = Vec::with_capacity(state.depth[pos] as usize);
        result.push(pos);
        while pos != from {
            let length = state.depth[pos];
            for prev_pos in neighbors(pos, self.width, self.height) {
                if state.depth[prev_pos] == length - 1 {
                    pos = prev_pos;
                    result.push(pos);
                    break; // push first found point
                }
            }
        }

        result.reverse();
        Some(result)
    }
}
