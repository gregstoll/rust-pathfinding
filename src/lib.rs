use std::path::Path;
use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_line_segment_mut, draw_text_mut, draw_filled_rect_mut, draw_polygon_mut};
use imageproc::point::Point;
use imageproc::rect::Rect;
use rusttype::{Font, Scale};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub i16, pub i16);

pub struct Board {
    pub width: u8,
    pub height: u8,
    pub data: Vec<Vec<Option<u8>>>,
    pub allow_diagonal: bool
}

impl Board {
    pub fn new(board_lines: Vec<&str>, allow_diagonal: bool) -> Board {
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

    pub fn get_successors(&self, position: &Pos) -> Vec<Successor> {
        let mut successors = Vec::new();
        for dx in -1i16..=1 {
            for dy in -1i16..=1 {
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

    pub fn draw_to_image(&self, file_path: &Path, pos_path: Option<&Vec<Pos>>) {
        const CELL_WIDTH: u32 = 50;
        const CELL_HEIGHT: u32 = 50;
        let mut image = RgbImage::new(self.width as u32 * CELL_WIDTH, self.height as u32 * CELL_HEIGHT);
        image.fill(255u8);
        const BLACK: Rgb<u8> = Rgb([0u8, 0u8, 0u8]);
        const BLUE: Rgb<u8> = Rgb([0u8, 0u8, 255u8]);
        const RED: Rgb<u8> = Rgb([255u8, 0u8, 0u8]);
        const LIGHT_GRAY: Rgb<u8> = Rgb([150u8, 150u8, 150u8]);

        // draw inner border lines
        for i in 1u8..self.width {
            draw_line_segment_mut(&mut image, (i as f32 * CELL_WIDTH as f32, 0.0), (i as f32 * CELL_WIDTH as f32, self.height as f32 * CELL_HEIGHT as f32), BLACK);
        }
        for i in 1u8..self.height {
            draw_line_segment_mut(&mut image, (0.0, i as f32 * CELL_HEIGHT as f32), (self.width as f32 * CELL_WIDTH as f32, i as f32 * CELL_HEIGHT as f32), BLACK);
        }
        
        let font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
        let font = Font::try_from_vec(font).unwrap();
        let height = 24.0;
        let scale = Scale {
            x: height * 2.0,
            y: height,
        };
        let start_pos = pos_path.map(|v| v.first()).flatten();
        let end_pos = pos_path.map(|v| v.last()).flatten();
        // draw the numbers/walls (with start and end positions)
        for y in 0..self.height {
            for x in 0..self.width {
                let board_value = self.data[y as usize][x as usize];
                let cur_pos = Pos(x as i16, y as i16);
                let mut cur_color: &Rgb<u8> = &BLACK;
                // This would be a nice place to use is_some_and(), but it's still unstable
                // https://github.com/rust-lang/rust/issues/93050
                if let Some(start_pos_real) = start_pos {
                    if start_pos_real == &cur_pos {
                        cur_color = &BLUE;
                    }
                }
                if let Some(end_pos_real) = end_pos {
                    if end_pos_real == &cur_pos {
                        cur_color = &RED;
                    }
                }
                match board_value {
                    Some(board_value) => {
                        draw_text_mut(&mut image, 
                            *cur_color, 
                            x as i32 * CELL_WIDTH as i32 + 13,
                            y as i32 * CELL_HEIGHT as i32 + 13, 
                            scale,
                            &font,
                            &format!("{}", board_value));
                    }
                    None => {
                        draw_filled_rect_mut(&mut image, Rect::at(x as i32 * CELL_WIDTH as i32, y as i32 * CELL_HEIGHT as i32).of_size(CELL_WIDTH, CELL_HEIGHT), *cur_color);
                    }
                }
            }
        }

        fn get_line_endpoint(start: &Pos, end: &Pos) -> (f32, f32) {
            let x_delta = 10.0 * match end.0.cmp(&start.0) {
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Greater => 1
            } as f32;
            let y_delta = 10.0 * match end.1.cmp(&start.1) {
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Greater => 1
            } as f32;

            ((start.0 as f32 + 0.5) * CELL_WIDTH as f32 + x_delta, (start.1 as f32 + 0.5) * CELL_HEIGHT as f32 + y_delta)
        }
        fn get_points_for_rectangle_around_line(start: &(f32, f32), end: &(f32, f32), width: f32, space_for_arrow: f32) -> Vec<Point<i32>> {
            let (x1, y1) = start;
            let (x2, y2) = end;
            let x_delta = x2 - x1;
            let y_delta = y2 - y1;
            let x_delta_norm = x_delta / x_delta.hypot(y_delta);
            let y_delta_norm = y_delta / x_delta.hypot(y_delta);

            vec![
                Point::new((x1 - y_delta_norm * (width / 2.0)) as i32, (y1 + x_delta_norm * (width / 2.0)) as i32),
                Point::new((x1 + y_delta_norm * (width / 2.0)) as i32, (y1 - x_delta_norm * (width / 2.0)) as i32),
                Point::new((x2 + y_delta_norm * (width / 2.0) - x_delta_norm * space_for_arrow) as i32, (y2 - x_delta_norm * (width / 2.0) - y_delta_norm * space_for_arrow) as i32),
                Point::new((x2 - y_delta_norm * (width / 2.0) - x_delta_norm * space_for_arrow) as i32, (y2 + x_delta_norm * (width / 2.0) - y_delta_norm * space_for_arrow) as i32),
            ]
        }
        fn get_points_for_arrowhead(start: &(f32, f32), end: &(f32, f32), width: f32, length: f32) -> Vec<Point<i32>> {
            //
            //    start
            //    ***
            //    * *
            //    * *
            //  ******* <- midpoint of this line is arrow_middle
            //    ***
            //    end

            let (x1, y1) = start;
            let (x2, y2) = end;
            let x_delta = x2 - x1;
            let y_delta = y2 - y1;
            let x_delta_norm = x_delta / x_delta.hypot(y_delta);
            let y_delta_norm = y_delta / x_delta.hypot(y_delta);
            let arrow_middle_x = x2 - x_delta_norm * length;
            let arrow_middle_y = y2 - y_delta_norm * length;

            vec![
                Point::new(*x2 as i32, *y2 as i32),
                Point::new((arrow_middle_x - y_delta_norm * width) as i32, (arrow_middle_y + x_delta_norm * width) as i32),
                Point::new((arrow_middle_x + y_delta_norm * width) as i32, (arrow_middle_y - x_delta_norm * width) as i32),
            ]
        }
        // Draw the path
        if let Some(pos_path) = pos_path {
            pos_path.windows(2).for_each(|pair| {
                let start_pos = &pair[0];
                let end_pos = &pair[1];
                let start_line_endpoint = get_line_endpoint(start_pos, end_pos);
                let end_line_endpoint = get_line_endpoint(end_pos, start_pos);
                draw_polygon_mut(&mut image, &get_points_for_rectangle_around_line(&start_line_endpoint, &end_line_endpoint, 5.0, 12.0), LIGHT_GRAY);
                draw_polygon_mut(&mut image, &get_points_for_arrowhead(&start_line_endpoint, &end_line_endpoint, 7.0, 12.0), LIGHT_GRAY);
                // TODO - use draw_polygon_mut to draw an arrowhead
            });
        }

        image.save(file_path).unwrap();
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub struct Successor {
    pub pos: Pos,
    pub cost: u8,
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