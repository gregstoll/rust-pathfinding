use std::path::Path;

use pathfinding::prelude::bfs;
use rust_pathfinding::{Board, Pos};

fn main() {
    let board = Board::new(vec![
        "21941",
        "1X587",
        "238X1",
        "18285",
        "13485"], false);
    board.draw_to_image(Path::new("board.png"), Some(vec![Pos(0,0), Pos(0, 1)]));
}
