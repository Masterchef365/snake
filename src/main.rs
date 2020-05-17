mod snake_widget;
mod board;
use board::{Board, Tile};
use snake_widget::SnakeWidget;
use iced::*;
use iced::canvas::*;


fn main() {
    <SnakeApp as Sandbox>::run(Default::default());
}

struct SnakeApp {}

#[derive(Debug)]
pub enum Message {}

impl Sandbox for SnakeApp {
    type Message = Message;

    fn new() -> Self {
        Self {}
    }

    fn title(&self) -> String {
        String::from("Snake evolution")
    }

    fn update(&mut self, _message: Self::Message) {

    }

    fn view(&mut self) -> Element<Self::Message> {
        let board = Board::empty(10, 10);
        let widget = SnakeWidget::new(board, 10.0, 1.0);
        Canvas::new(widget)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
