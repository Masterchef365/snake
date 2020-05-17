use crate::game::{Game, StepResult};
use crate::neuralnet::*;

const MAX_STEPS: usize = 250;

#[derive(Clone)]
pub struct Trainer {
    pub model: NeuralNet,
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
            max_steps,
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

pub struct TrainOutput {
    pub trainers: Vec<(usize, Trainer)>,
}

impl TrainOutput {
    pub fn best(&self) -> &(usize, Trainer) {
        self.trainers
            .iter()
            .max_by(|(a, _), (b, _)| a.cmp(b))
            .unwrap()
    }

    pub fn mean(&self) -> f32 {
        let mut acc = 0.0;
        for (score, _) in &self.trainers {
            acc += *score as f32;
        }
        acc / self.trainers.len() as f32
    }
}

pub struct Evolver {
    trainers: Vec<Trainer>,
}

impl Evolver {
    pub fn new(units: usize, width: usize, height: usize, max_steps: usize) -> Self {
        Self {
            trainers: (0..units)
                .map(|_| Trainer::new(width, height, max_steps))
                .collect(),
        }
    }

    pub fn train_step(&mut self, learning_rate: f32) -> TrainOutput {
        use rayon::iter::IntoParallelIterator;
        use rayon::iter::ParallelIterator;
        let n_trainers = self.trainers.len();
        let trainers = std::mem::replace(&mut self.trainers, Default::default())
            .into_par_iter()
            .map(|mut trainer| (trainer.run(), trainer))
            .collect();
        let train_out = TrainOutput { trainers };
        self.trainers = (0..n_trainers)
            .map(|_| {
                let mut instance = train_out.best().1.clone();
                instance.fuzz(learning_rate);
                instance
            })
            .collect();
        train_out
    }
}
