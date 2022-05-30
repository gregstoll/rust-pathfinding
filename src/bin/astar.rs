use std::path::Path;

use pathfinding::prelude::astar;
use rust_pathfinding::{Board, Pos};

fn main() {
    let board = Board::new(vec![
        "21397X2",
        "1X19452",
        "62251X1",
        "1612179",
        "1348512",
        "61453X1",
        "7861243"], false);
    let start = Pos(0,1);
    let goal = Pos(6,2);
    let result = astar(
        &start,
        |p| board.get_successors(p).iter().map(|s| (s.pos, s.cost)).collect::<Vec<_>>(),
        |p| ((p.0 - goal.0).abs() + (p.1 - goal.1).abs()) as u32,
        |p| *p==goal);
    let result = result.expect("No path found");
    println!("total cost: {:}", result.1);
    board.draw_to_image(Path::new("astar.png"), Some(&result.0));
}