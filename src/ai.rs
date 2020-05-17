use crate::board::Tile;
use crate::game::{Direction, Game};
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

pub fn input_neurons(game: &Game) -> Box<[f32]> {
    let d: [(isize, isize); 8] = [
        (-1, 1),  // Top left
        (0, 1),   // Top middle
        (1, 1),   // Top right
        (-1, 0),  // Middle left
        (1, 0),   // Middle right
        (-1, -1), // Bottom left
        (0, -1),  // Bottom middle
        (1, -1),  // Bottom right
    ];
    let mut neurons = Vec::with_capacity(24);
    for (dx, dy) in &d {
        for tile in &[Some(Tile::Snake), Some(Tile::Food), None] {
            let dist = game.dist(*tile, *dx, *dy);
            let dist_sigmoid = 1.0 / (1.0 + (-(dist.unwrap_or(0) as f32)).exp());
            neurons.push(dist_sigmoid);
        }
    }
    neurons.into()
}

pub fn operate_game(game: &mut Game, neurons: &[f32]) {
    game.set_direction(get_direction(neurons));
}

pub fn get_direction(neurons: &[f32]) -> Direction {
    let largest = neurons
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(&b).unwrap())
        .unwrap()
        .0;
    match largest {
        0 => Direction::Up,
        1 => Direction::Down,
        2 => Direction::Left,
        3 => Direction::Right,
        _ => panic!("Neuron returned is not in set"),
    }
}

#[test]
fn operate_test() {
    assert_eq!(Direction::Up, get_direction([1.0, 0.0, 0.5, 0.0].as_ref()));
    assert_eq!(
        Direction::Down,
        get_direction([0.0, 1.0, 0.5, 0.0].as_ref())
    );
    assert_eq!(
        Direction::Left,
        get_direction([1.0, 0.0, 1.5, 0.0].as_ref())
    );
    assert_eq!(
        Direction::Right,
        get_direction([0.0, 0.0, 0.0, 0.5].as_ref())
    );
}

#[derive(Debug, Clone)]
struct Layer {
    weights: Box<[f32]>,
    biases: Box<[f32]>,
    input_size: usize,
    output_size: usize,
}

impl Layer {
    pub fn new(input_size: usize, output_size: usize) -> Self {
        let size = input_size * output_size;
        let unif = Uniform::new(0.0, 1.0);
        let mut rng = thread_rng();
        Self {
            weights: rng.sample_iter(&unif).take(size).collect(),
            biases: rng.sample_iter(&unif).take(size).collect(),
            input_size,
            output_size,
        }
    }

    pub fn fuzz(&mut self, learning_rate: f32) {
        let unif = Uniform::new(-learning_rate, learning_rate);
        let mut rng = thread_rng();
        for (v, d) in self
            .weights
            .iter_mut()
            .chain(self.biases.iter_mut())
            .zip(rng.sample_iter(&unif))
        {
            *v += d;
        }
    }

    pub fn infer(&self, input: &[f32]) -> Box<[f32]> {
        let mut output = Vec::new();
        for (weight_row, bias_row) in self
            .weights
            .chunks_exact(self.input_size)
            .zip(self.biases.chunks_exact(self.input_size))
        {
            let mut out = 0.0;
            for ((weight, bias), input) in weight_row.iter().zip(bias_row.iter()).zip(input.iter())
            {
                out += weight * input + bias
            }
            output.push(out);
        }
        output.into()
    }
}

// TODO: Store allocations for inference in here in here
#[derive(Debug, Clone)]
struct NeuralNet {
    hidden_0: Layer,
    hidden_1: Layer,
}

impl NeuralNet {
    pub fn new() -> Self {
        Self {
            hidden_0: Layer::new(24, 18),
            hidden_1: Layer::new(18, 4),
        }
    }

    pub fn infer(&self, input_layer: &[f32]) -> Box<[f32]> {
        let intermediate = self.hidden_0.infer(input_layer);
        self.hidden_1.infer(&intermediate)
    }

    pub fn fuzz(&mut self, learning_rate: f32) {
        self.hidden_0.fuzz(learning_rate);
        self.hidden_1.fuzz(learning_rate);
    }
}
