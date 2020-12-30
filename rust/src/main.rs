mod array2d;
mod bfs;

fn main() {
    let start = std::time::Instant::now();

    let bfs = bfs::BFS::new(100, 100);

    for _ in 0..100_000 {
        let _result = bfs.path((1, 1), (98, 98));
    }

    let end = std::time::Instant::now();

    let elapsed = end.duration_since(start);
    println!("Time: {}", elapsed.as_millis());
}
