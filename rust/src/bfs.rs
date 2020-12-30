use ::array2d::Array2D;

type Point = (usize, usize);

const OFFSETS: &[(isize, isize)] = &[(1, 0), (-1, 0), (0, 1), (0, -1)];

fn neighbors(pos: Point) -> impl Iterator<Item = Point> {
    OFFSETS.iter().map(move |offset| {
        // TODO: out of bounds
        let new_pos = (
            (pos.0 as isize + offset.0) as usize,
            (pos.1 as isize + offset.1) as usize,
        );
        new_pos
    })
}

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

        for index in 0..width {
            walls[(index, 0)] = true;
            walls[(index, height - 1)] = true;
        }

        for index in 0..height {
            walls[(0, index)] = true;
            walls[(width - 1, index)] = true;
        }

        let h = height / 10;
        let w = width / 10;

        for index in 0..height - h {
            let x = 2 * w;
            let y = index;
            walls[(x, y)] = true;
        }

        for index in h..height {
            let x = 8 * w;
            let y = index;
            walls[(x, y)] = true;
        }

        walls
    }

    pub fn path(&self, from: Point, to: Point) -> Vec<Point> {
        let mut visited = Array2D::filled_with(false, self.width, self.height);
        let mut depth: Array2D<i16> = Array2D::filled_with(-1, self.width, self.height);

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
