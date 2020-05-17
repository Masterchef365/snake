use snake_game::board::{Board, Tile};
use crate::ui::Message;
use iced::canvas::*;
use iced::*;

const BG_COLOR: Color = Color::from_rgb(0.1, 0.1, 0.1);
const EMPTY_COLOR: Color = Color::from_rgb(0.2, 0.2, 0.2);
const FOOD_COLOR: Color = Color::from_rgb(1.0, 0.0, 0.0);
const SNAKE_COLOR: Color = Color::WHITE;

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
}

impl Program<Message> for SnakeWidget {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut frame = Frame::new(bounds.size());
        frame.fill(
            &Path::rectangle(Point::new(0.0, 0.0), bounds.size()),
            Fill::Color(BG_COLOR),
        );

        let total_height = (self.board.height() - 1) as f32 * (self.tile_size + self.margin);

        let mut y = 0.0;
        for row in self.board.rows() {
            let mut x = 0.0;
            for tile in row {
                let color = match tile {
                    Tile::Empty => EMPTY_COLOR,
                    Tile::Snake => SNAKE_COLOR,
                    Tile::Food => FOOD_COLOR,
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
