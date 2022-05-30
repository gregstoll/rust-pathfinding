use std::path::Path;

use pathfinding::prelude::dijkstra;
use rust_pathfinding::{Board, Pos};

fn main() {
    let board = Board::new(vec![
        "89341",
        "1X534",
        "62891",
        "17214",
        "13285"], false);
    let start = Pos(0,1);
    let goal = Pos(4,2);
    let result = dijkstra(
        &start,
        |p| board.get_successors(p).iter().map(|s| (s.pos, s.cost)).collect::<Vec<_>>(),
        |p| *p==goal);
    let result = result.expect("No path found");
    println!("total cost: {:}", result.1);
    board.draw_to_image(Path::new("dijkstra.png"), Some(&result.0));
}