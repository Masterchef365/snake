mod board;
mod game;
mod snake_widget;
mod neuralnet;
mod trainer;
mod ui;
use ui::SnakeApp;

fn main() {
    //<SnakeApp as iced::Application>::run(Default::default());
    let mut evolver = trainer::Evolver::new(30, 20, 20);
    for _ in 0..5000 {
        println!("{}", evolver.train_step(0.1));
    }
}
