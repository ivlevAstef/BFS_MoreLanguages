// Because these warnings are annoying when testing different things
#![allow(dead_code, unused_imports)]

#[cfg_attr(feature = "hacky-array2d", path = "array2d/hacky.rs")]
#[cfg_attr(not(feature = "hacky-array2d"), path = "array2d/normal.rs")]
mod array2d;
mod bfs;
mod point;
#[cfg_attr(feature = "hacky-queue", path = "queue/hacky.rs")]
#[cfg_attr(not(feature = "hacky-queue"), path = "queue/normal.rs")]
mod queue;

use point::Point;

fn main() {
    let start = std::time::Instant::now();

    let bfs = bfs::BFS::new(100, 100);

    for _ in 0..100_000 {
        let _result = bfs.path(Point::new(1, 1), Point::new(98, 98));
    }

    let end = std::time::Instant::now();

    let elapsed = end.duration_since(start);
    println!("Time: {}", elapsed.as_millis());
}
