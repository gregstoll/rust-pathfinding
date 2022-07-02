# rust-pathfinding
Examples of pathfinding in Rust

This repo is the example code for the article [Pathfinding in Rust: A tutorial with examples](https://blog.logrocket.com/pathfinding-rust-tutorial-examples/) on the [LogRocket blog](https://blog.logrocket.com/). It shows examples of doing breadth-first search, Dijkstra's algorithm, and A* search.

To run these, use:
- `cargo run --bin bfs` to run the breadth-first search example - this will output bfs.png in the root directory.
- `cargo run --bin dijkstra` to run the Dijkstra's algorithm example - this will output dijkstra.png in the root directory.
- `cargo run --bin astar` to run the A* search example - this will output astar.png in the root directory.

The code uses the [pathfinding](https://crates.io/crates/pathfinding) crate to do the searches. The `Board` struct uses the [imageproc](https://crates.io/crates/imageproc) crate to draw the board and paths.

The `Board` struct is defined in [lib.rs](https://github.com/gregstoll/rust-pathfinding/blob/main/src/lib.rs), and there are some unit tests in that file as well.
