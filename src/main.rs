mod board;
mod game;
mod neuralnet;
mod snake_widget;
mod ui;
use crate::game::{Game, StepResult};
use crate::neuralnet::*;
use rand::Rng;
use ui::SnakeApp;

fn eval(model: &NeuralNet, width: usize, height: usize, max_steps: usize) -> f32 {
    let mut game = Game::new(width, height);
    for _ in 0..max_steps {
        model.play(&mut game);
        match game.step() {
            StepResult::Alive => (),
            StepResult::Died => return game.score() as f32,
        }
    }
    game.score() as f32
}

pub fn run_in_parallel(
    trainers: &[NeuralNet],
    width: usize,
    height: usize,
    max_steps: usize,
) -> Vec<f32> {
    use rayon::iter::IntoParallelIterator;
    use rayon::iter::ParallelIterator;
    trainers
        .into_par_iter()
        .map(|net| eval(net, width, height, max_steps))
        .collect()
}

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

    let mut best_score = 1.0;
    let mut best_net: Option<NeuralNet> = None;

    let mut gene_pool = Vec::with_capacity(units);
    for _ in 0..units {
        gene_pool.push(NeuralNet::new());
    }

    let mut rng = rand::thread_rng();
    for iter in 1..epochs {
        // Run the nets
        let scores = run_in_parallel(&gene_pool, width, height, max_steps);
        let mean = scores.iter().sum::<f32>() / scores.len() as f32;
        let mut pairs: Vec<_> = gene_pool.iter().zip(scores).collect();
        pairs.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
        let (epoch_best_net, epoch_best_score) = *pairs.first().unwrap();

        let learning_rate = 1.0 / (best_score * decay);

        println!(
            "Epoch {}/{} ({:.00}%) [Learning rate: {:.04}, All time best: {}]: (Best: {}, Avg: {:.04})",
            iter, epochs, iter as f32 * 100.0 / epochs as f32, learning_rate, best_score, epoch_best_score, mean
        );

        if epoch_best_score > best_score {
            best_score = epoch_best_score;
            best_net = Some(epoch_best_net.clone());
        }

        // Pick the (units/8) best and duplicated them across the training space
        let best_n: Vec<_> = pairs
            .drain(..)
            .take(units / 8)
            .map(|(trainer, _)| trainer.clone())
            .collect();

        gene_pool.clear();
        for _ in 0..units {
            let selection = rng.gen_range(0, best_n.len() - 1);
            let mut new_net = best_n[selection].clone();
            new_net.fuzz(learning_rate);
            gene_pool.push(new_net);
        }
    }
    <SnakeApp as iced::Application>::run(iced::Settings::with_flags((
        best_net.unwrap(),
        width,
        height,
    )));
}
