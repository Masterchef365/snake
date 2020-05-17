mod board;
mod game;
mod neuralnet;
mod snake_widget;
mod trainer;
mod ui;
use ui::SnakeApp;

fn main() {
    let mut args = std::env::args().skip(1).peekable();
    if args.peek().is_none() {
        println!("Usage: n_epochs decay_rate units width height max_steps");
    }
    let epochs: u32 = args.next().unwrap().parse().unwrap();
    let decay: f32 = args.next().unwrap().parse().unwrap();
    let units: usize = args.next().unwrap().parse().unwrap();
    let width: usize = args.next().unwrap().parse().unwrap();
    let height: usize = args.next().unwrap().parse().unwrap();
    let max_steps: usize = args.next().unwrap().parse().unwrap();
    let mut evolver = trainer::Evolver::new(units, width, height, max_steps);
    let mut net = None;
    for iter in 1..epochs {
        let learning_rate = 1.0 / (iter as f32).powf(decay);
        let (model, score) = evolver.train_step(learning_rate);
        println!("{} ({}): {}", iter, learning_rate, score,);
        net = Some(model);
    }
    <SnakeApp as iced::Application>::run(iced::Settings::with_flags((net.unwrap(), width, height)));
}
