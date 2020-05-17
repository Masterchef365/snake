use crate::board::{Board, Tile};
use crate::Message;
use iced::canvas::*;
use iced::*;

pub struct SnakeWidget {
    board: Board,
    tile_size: f32,
    margin: f32,
}

impl SnakeWidget {
    pub fn new(board: Board, tile_size: f32, margin: f32) -> Self {
        Self {
            board,
            tile_size,
            margin,
        }
    }

    pub fn set_board(&mut self, board: Board) {
        self.board = board;
    }
}

impl Program<Message> for SnakeWidget {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut frame = Frame::new(bounds.size());
        /*
        frame.fill(
            &Path::rectangle(Point::new(0.0, 0.0), bounds.size()),
            Fill::Color(Color::BLACK),
        );
        */

        let total_height = self.board.height() as f32 * (self.tile_size + self.margin);

        let mut y = 0.0;
        for row in self.board.rows() {
            let mut x = 0.0;
            for tile in row {
                let color = match tile {
                    Tile::Empty => Color::BLACK,
                    Tile::Snake => Color::WHITE,
                    Tile::Food => Color::from_rgb8(0xff, 0x00, 0x00),
                };
                frame.fill(
                    &Path::rectangle(
                        Point::new(x, total_height - y),
                        Size::new(self.tile_size, self.tile_size),
                    ),
                    Fill::Color(color),
                );
                x += self.tile_size + self.margin;
            }
            y += self.tile_size + self.margin;
        }

        vec![frame.into_geometry()]
    }
}
