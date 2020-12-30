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
    walls: Vec<Vec<bool>>,
}

impl BFS {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            walls: Self::generate_walls(width, height),
        }
    }

    fn generate_walls(width: usize, height: usize) -> Vec<Vec<bool>> {
        let mut walls = vec![vec![false; height]; width];

        for index in 0..width {
            walls[index][0] = true;
            walls[index][height - 1] = true;
        }

        for index in 0..height {
            walls[0][index] = true;
            walls[width - 1][index] = true;
        }

        let h = height / 10;
        let w = width / 10;

        for index in 0..height - h {
            let x = 2 * w;
            let y = index;
            walls[x][y] = true;
        }

        for index in h..height {
            let x = 8 * w;
            let y = index;
            walls[x][y] = true;
        }

        walls
    }

    pub fn path(&self, from: Point, to: Point) -> Vec<Point> {
        let mut visited = vec![vec![false; self.height]; self.width];
        let mut depth: Vec<Vec<i16>> = vec![vec![-1; self.height]; self.width];

        visited[from.0][from.1] = true;
        depth[from.0][from.1] = 0;

        let mut queue = std::collections::VecDeque::with_capacity(self.width * self.height);
        queue.push_back(from);
        while let Some(pos) = queue.pop_front() {
            let length = depth[pos.0][pos.1];
            if pos == to {
                break;
            }

            for new_pos in neighbors(pos) {
                if visited[new_pos.0][new_pos.1] || self.walls[new_pos.0][new_pos.1] {
                    continue;
                }
                visited[new_pos.0][new_pos.1] = true;
                depth[new_pos.0][new_pos.1] = length + 1;
                queue.push_back(new_pos);
            }
        }

        // not found
        if !visited[to.0][to.1] {
            return Vec::new();
        }

        let mut pos = to;
        let mut result = Vec::with_capacity(depth[pos.0][pos.1] as usize);
        result.push(pos);
        while pos != from {
            let length = depth[pos.0][pos.1];
            for prev_pos in neighbors(pos) {
                if depth[prev_pos.0][prev_pos.1] == length - 1 {
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
