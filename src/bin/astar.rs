use pathfinding::prelude::astar;
use rust_pathfinding::{Board, Pos};

//const pattern : 
fn main() {
    let board = Board::new(vec![
        "21941",
        "1X534",
        "23891",
        "18215",
        "13485"], false);
    let start = Pos(0,1);
    let goal = Pos(4,2);
    /*let result = bfs(
        &start,
        |p| board.get_successors(p).iter().map(|successor| successor.pos).collect::<Vec<_>>(), 
        |p| *p==goal);
    let result = result.expect("No path found");
    board.draw_to_image(Path::new("board.png"), Some(&result));*/
}