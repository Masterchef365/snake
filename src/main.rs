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
        return;
    }
    let epochs: u32 = args.next().unwrap().parse().unwrap();
    let decay: f32 = args.next().unwrap().parse().unwrap();
    let units: usize = args.next().unwrap().parse().unwrap();
    let width: usize = args.next().unwrap().parse().unwrap();
    let height: usize = args.next().unwrap().parse().unwrap();
    let max_steps: usize = args.next().unwrap().parse().unwrap();

    let mut evolver = trainer::Evolver::new(units, width, height, max_steps);
    let mut best_score = 0.0;
    let mut best_net = None;
    let mut learning_rate = 1.0;
    for iter in 1..epochs {
        //let learning_rate = 1.0 / (iter as f32).powf(decay);
        let train_out = evolver.train_step(learning_rate);
        let (epoch_best_score, epoch_best_trainer) = train_out.best();
        let avg = train_out.mean();
        println!(
            "Epoch {}/{} ({:.00}%) [Learning rate: {:.04}, All time best: {}]: (Best: {}, Avg: {:.04})",
            iter, epochs, iter as f32 * 100.0 / epochs as f32, learning_rate, best_score, epoch_best_score, avg
        );
        learning_rate = 1.0 / (avg * 3.0);
        if *epoch_best_score > best_score {
            best_score = *epoch_best_score;
            best_net = Some(epoch_best_trainer.model.clone());
        }
    }
    <SnakeApp as iced::Application>::run(iced::Settings::with_flags((
        best_net.unwrap(),
        width,
        height,
    )));
}
