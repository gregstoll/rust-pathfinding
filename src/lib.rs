#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i16, i16);

pub struct Board {
    pub width: u8,
    pub height: u8,
    pub data: Vec<Vec<Option<u8>>>,
    pub allow_diagonal: bool
}

impl Board {
    fn new(board_lines: Vec<&str>, allow_diagonal: bool) -> Board {
        let width = board_lines[0].len() as u8;
        let height = board_lines.len() as u8;
        let mut data = Vec::new();
        for board_line in board_lines {
            let mut row: Vec<Option<u8>> = Vec::new();
            for c in board_line.chars() {
                match c {
                    'X' => row.push(None),
                    '1'..='9' => row.push(Some(c as u8 - '0' as u8)),
                    _ => panic!("invalid character")
                }
            }
            data.push(row);
        }
        Board {width, height, data, allow_diagonal}
    }
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
struct Successor {
    pos: Pos,
    cost: u8,
}
// Used to make writing tests easier
impl PartialEq<(Pos, u8)> for Successor {
    fn eq(&self, other: &(Pos, u8)) -> bool {
        self.pos == other.0 && self.cost == other.1
    }
}

fn get_successors(position: &Pos, board: &Board) -> Vec<Successor> {
    let mut successors = Vec::new();
    for dx in (-1 as i16)..=1 {
        for dy in (-1 as i16)..=1 {
            if board.allow_diagonal {
                if dx == 0 && dy == 0 {
                    continue;
                }
            }
            else {
                // Omit diagonal moves (and moving to the same position)
                if (dx + dy).abs() != 1 {
                    continue;
                }
            }
            let new_position = Pos(position.0 + dx, position.1 + dy);
            if new_position.0 < 0 || new_position.0 >= board.width.into() || new_position.1 < 0 || new_position.1 >= board.height.into() {
                continue;
            }
            let board_value = board.data[new_position.1 as usize][new_position.0 as usize];
            if let Some(board_value) = board_value {
                successors.push(Successor { pos: new_position, cost: board_value});
            }
        }
    }
    return successors;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_onebyoneboard_nosuccessors() {
        let board = Board::new(vec!["1"], false);
        let result = get_successors(&Pos(0, 0), &board);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_twobytwoboardwithobstacle() {
        let board = Board::new(vec![
            "21",
            "1X"], false);
        let result = get_successors(&Pos(0, 1), &board);
        assert_eq!(result, vec![(Pos(0, 0), 2)]);
    }

    #[test]
    fn test_twobytwoboardwithobstacleanddiagonal() {
        let board = Board::new(vec![
            "21",
            "1X"], true);
        let result = get_successors(&Pos(0, 1), &board);
        assert_eq!(result, vec![(Pos(0, 0), 2), (Pos(1, 0), 1)]);
    }
 
    #[test]
    fn test_bigboardmovingfrommiddle() {
        let board = Board::new(vec![
            "21941",
            "1X587",
            "238X1",
            "18285",
            "13485"], false);
        let result = get_successors(&Pos(2, 2), &board);
        assert_eq!(result, vec![(Pos(1, 2), 3), (Pos(2, 1), 5), (Pos(2, 3), 2)]);
    }

    #[test]
    fn test_surroundedbywalls() {
        let board = Board::new(vec![
            "21941",
            "1XX87",
            "2X8X1",
            "18X85",
            "13485"], false);
        let result = get_successors(&Pos(2, 2), &board);
        assert_eq!(result.len(), 0);
    }

}