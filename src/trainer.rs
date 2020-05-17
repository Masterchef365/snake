use crate::game::{Game, StepResult};
use crate::neuralnet::*;

const MAX_STEPS: usize = 250;

#[derive(Clone)]
struct Trainer {
    model: NeuralNet,
    width: usize,
    height: usize,
    max_steps: usize,
}

impl Trainer {
    pub fn new(width: usize, height: usize, max_steps: usize) -> Self {
        Self {
            model: NeuralNet::new(),
            width,
            height,
            max_steps
        }
    }

    pub fn fuzz(&mut self, learning_rate: f32) {
        self.model.fuzz(learning_rate);
    }

    pub fn run(&mut self) -> usize {
        let mut game = Game::new(self.width, self.height);
        for _ in 0..self.max_steps {
            self.model.play(&mut game);
            match game.step() {
                StepResult::Alive => (),
                StepResult::Died => return game.score(),
            }
        }
        game.score()
    }
}

pub struct Evolver {
    trainers: Vec<Trainer>,
}

impl Evolver {
    pub fn new(units: usize, width: usize, height: usize, max_steps: usize) -> Self {
        Self {
            trainers: (0..units).map(|_| Trainer::new(width, height, max_steps)).collect(),
        }
    }

    pub fn train_step(&mut self, learning_rate: f32) -> (NeuralNet, usize) {
        use rayon::iter::IntoParallelIterator;
        use rayon::iter::ParallelIterator;
        let n_trainers = self.trainers.len();
        let (score, best) = std::mem::replace(&mut self.trainers, Default::default())
            .into_par_iter()
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
        (best.model, score)
    }
}
