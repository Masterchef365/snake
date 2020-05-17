mod board;
mod game;
mod neuralnet;
mod snake_widget;
mod trainer;
mod ui;
use ui::SnakeApp;

fn main() {
    let mut evolver = trainer::Evolver::new(10_000, 20, 20, 250);
    let mut net = None;
    for iter in 1..90 {
        let learning_rate = 1.0 / (iter as f32).powf(0.3);
        let (model, score) = evolver.train_step(learning_rate);
        println!("{} ({}): {}", iter, learning_rate, score,);
        net = Some(model);
    }
    <SnakeApp as iced::Application>::run(iced::Settings::with_flags(net.unwrap()));
}
