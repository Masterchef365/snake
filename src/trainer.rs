use crate::game::{Game, StepResult};
use crate::neuralnet::*;

#[derive(Clone)]
struct Trainer {
    model: NeuralNet,
    width: usize,
    height: usize,
}

impl Trainer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            model: NeuralNet::new(),
            width,
            height,
        }
    }

    pub fn fuzz(&mut self, learning_rate: f32) {
        self.model.fuzz(learning_rate);
    }

    pub fn run(&mut self) -> usize {
        let mut game = Game::new(self.width, self.height);
        loop {
            let input_layer = input_neurons(&game);
            let output_layer = self.model.infer(&input_layer);
            operate_game(&mut game, &output_layer);
            match game.step() {
                StepResult::Alive => (),
                StepResult::Died => return game.score(),
            }
        }
    }
}

pub struct Evolver {
    trainers: Vec<Trainer>,
}

impl Evolver {
    pub fn new(units: usize, width: usize, height: usize) -> Self {
        Self {
            trainers: (0..units).map(|_| Trainer::new(width, height)).collect(),
        }
    }

    pub fn train_step(&mut self, learning_rate: f32) -> usize {
        let n_trainers = self.trainers.len();
        let (score, best) = self
            .trainers
            .drain(..)
            .map(|mut trainer| (trainer.run(), trainer))
            .max_by(|(a, _), (b, _)| a.cmp(b))
            .unwrap();
        self.trainers = (0..n_trainers)
            .map(|_| {
                let mut instance = best.clone();
                instance.fuzz(learning_rate);
                instance
            })
            .collect();
        score
    }
}
