use std::path::Path;
use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_line_segment_mut, draw_text_mut, draw_filled_rect_mut};
use imageproc::rect::Rect;
use rusttype::{Font, Scale};

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

    fn get_successors(&self, position: &Pos) -> Vec<Successor> {
        let mut successors = Vec::new();
        for dx in (-1 as i16)..=1 {
            for dy in (-1 as i16)..=1 {
                if self.allow_diagonal {
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
                if new_position.0 < 0 || new_position.0 >= self.width.into() || new_position.1 < 0 || new_position.1 >= self.height.into() {
                    continue;
                }
                let board_value = self.data[new_position.1 as usize][new_position.0 as usize];
                if let Some(board_value) = board_value {
                    successors.push(Successor { pos: new_position, cost: board_value});
                }
            }
        }
        return successors;
    }

    fn draw_to_image(&self, path: &Path) {
        let mut image = RgbImage::new(self.width as u32 * 50, self.height as u32 * 50);
        const black: Rgb<u8> = Rgb([0u8, 0u8, 0u8]);
        // draw inner border lines
        for i in 1u8..self.width {
            draw_line_segment_mut(&mut image, (i as f32 * 50.0, 0.0), (i as f32 * 50.0, self.height as f32 * 50.0), black);
        }
        for i in 1u8..self.height {
            draw_line_segment_mut(&mut image, (0.0, i as f32 * 50.0), (self.width as f32 * 50.0, i as f32 * 50.0), black);
        }
        
        let font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
        let font = Font::try_from_vec(font).unwrap();
        let height = 12.4;
        let scale = Scale {
            x: height * 2.0,
            y: height,
        };
        for i in 0..self.height {
            for j in 0..self.width {
                let board_value = self.data[i as usize][j as usize];
                match board_value {
                    Some(board_value) => {
                        draw_text_mut(&mut image, 
                            black, 
                            i as i32 * 50 + 10,
                            j as i32 * 50 + 10, 
                            scale,
                            &font,
                            &format!("{}", board_value));
                    }
                    None => {
                        draw_filled_rect_mut(&mut image, Rect::at(i as i32 * 50 + 10, j as i32 * 50 + 10).of_size(30, 30), black);
                    }
                }
            }
        }
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_onebyoneboard_nosuccessors() {
        let board = Board::new(vec!["1"], false);
        let result = board.get_successors(&Pos(0, 0));
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_twobytwoboardwithobstacle() {
        let board = Board::new(vec![
            "21",
            "1X"], false);
        let result = board.get_successors(&Pos(0, 1));
        assert_eq!(result, vec![(Pos(0, 0), 2)]);
    }

    #[test]
    fn test_twobytwoboardwithobstacleanddiagonal() {
        let board = Board::new(vec![
            "21",
            "1X"], true);
        let result = board.get_successors(&Pos(0, 1));
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
        let result = board.get_successors(&Pos(2, 2));
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
        let result = board.get_successors(&Pos(2, 2));
        assert_eq!(result.len(), 0);
    }

}