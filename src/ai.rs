use crate::board::Tile;
use crate::game::{Direction, Game};

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
    assert_eq!(Direction::Down, get_direction([0.0, 1.0, 0.5, 0.0].as_ref()));
    assert_eq!(Direction::Left, get_direction([1.0, 0.0, 1.5, 0.0].as_ref()));
    assert_eq!(Direction::Right, get_direction([0.0, 0.0, 0.0, 0.5].as_ref()));
}
