mod array2d;
mod bfs;

pub type PointCoord = i32;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Point(PointCoord, PointCoord);

fn main() {
    let start = std::time::Instant::now();

    let bfs = bfs::BFS::new(100, 100);

    for _ in 0..100_000 {
        let _result = bfs.path(Point(1, 1), Point(98, 98));
    }

    let end = std::time::Instant::now();

    let elapsed = end.duration_since(start);
    println!("Time: {}", elapsed.as_millis());
}
