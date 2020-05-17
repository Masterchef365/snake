mod board;
mod game;
mod snake_widget;
mod ai;
mod ui;
use ui::SnakeApp;

fn main() {
    <SnakeApp as iced::Application>::run(Default::default());
}
