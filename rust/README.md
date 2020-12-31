# Rust

Performance improvements are feature gated (see `Cargo.toml`)

To run with all of them enabled, use `cargo run --release --all-features`

`unsafe-indexing` feature disables out of bounds checks, and all such usages should be marked as unsafe for easier spotting of potential problems, but it is not done to show how safe API is used. Because all of this should not be necessary 99% of the time.
