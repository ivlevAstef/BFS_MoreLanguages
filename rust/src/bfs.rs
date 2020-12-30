use super::array2d::Array2D;
use super::{point, Point};
use std::cell::RefCell;

fn neighbors(pos: Point, width: usize, height: usize) -> impl Iterator<Item = Point> {
    const OFFSETS: &[(isize, isize)] = &[(1, 0), (-1, 0), (0, 1), (0, -1)];
    return OFFSETS.iter().filter_map(move |offset| {
        let p = (
            pos.x() as i32 + offset.0 as i32,
            pos.y() as i32 + offset.1 as i32,
        );
        if !cfg!(feature = "neighbors-ignore-bounds") {
            if p.0 < 0 || p.1 < 0 || p.0 >= width as i32 || p.1 >= height as i32 {
                return None;
            }
        }
        Some(Point::new(p.0 as point::Coord, p.1 as point::Coord))
    });
}

struct State {
    // from: Array2D<Option<Point>>,
// queue: std::collections::VecDeque<Point>,
}

impl State {
    fn new(width: usize, height: usize) -> Self {
        Self {
            // from: Array2D::filled_with(None, width, height),
            // queue: std::collections::VecDeque::with_capacity(width * height),
        }
    }
    fn clear(&mut self) {
        // self.from.fill(None);
        // self.queue.clear();
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
        let mut state = self.state.borrow_mut();
        let state: &mut State = &mut state;
        state.clear();

        let mut from = Array2D::filled_with(None, self.width, self.height);
        let mut queue = std::collections::VecDeque::new();

        from[start] = Some(start);
        queue.push_back(start);
        while let Some(pos) = queue.pop_front() {
            if pos == finish {
                break;
            }

            for new_pos in neighbors(pos, self.width, self.height) {
                if self.walls[new_pos] || from[new_pos].is_some() {
                    continue;
                }
                from[new_pos] = Some(pos);
                queue.push_back(new_pos);
            }
        }

        // not found
        if from[finish].is_none() {
            return None;
        }

        let mut pos = finish;
        let mut result = Vec::new();
        result.push(pos);
        while pos != start {
            let prev_pos = from[pos].unwrap();
            pos = prev_pos;
            result.push(pos);
        }

        result.reverse();
        Some(result)
    }
}
