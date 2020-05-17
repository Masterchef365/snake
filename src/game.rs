use crate::board::{Board, Tile};
use rand::Rng;
use std::collections::VecDeque;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn delta(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

pub enum StepResult {
    Alive,
    Died,
}

pub struct Game {
    width: usize,
    height: usize,
    snake: VecDeque<(usize, usize)>,
    food: (usize, usize),
    direction: Direction,
    score: usize,
}

fn random_position(width: usize, height: usize) -> (usize, usize) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0, width), rng.gen_range(0, height))
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            snake: VecDeque::from(vec![(width / 2, height / 2)]),
            direction: Direction::Right,
            food: random_position(width, height),
            score: 0,
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn look(&self) -> Tile {
        /*
        match self.direction {
            let (hx, hy) = self.head();
            Direction::Up => {
                for y in h_y..self.height {

                }
            },
        }
        */
        Tile::Snake
    }

    fn get_tile(&self, x: isize, y: isize) -> Option<Tile> {
        let (xu, yu) = (x as usize, y as usize);

        if x < 0 || xu > self.width || y < 0 || yu > self.height {
            return None;
        }

        if self.snake.contains(&(xu, yu)) {
            return Some(Tile::Snake);
        }

        let (food_x, food_y) = self.food;

        if xu == food_x && yu == food_y {
            Some(Tile::Food)
        } else {
            Some(Tile::Empty)
        }
    }

    pub fn head(&self) -> (usize, usize) {
        *self.snake.back().expect("Snake has no head")

    }

    pub fn step(&mut self) -> StepResult {
        let (hx, hy) = self.head();
        let (dx, dy) = self.direction.delta();
        let (next_x, next_y) = (hx as isize + dx, hy as isize + dy);

        match self.get_tile(next_x, next_y) {
            None | Some(Tile::Snake) => return StepResult::Died,
            Some(Tile::Food) => self.food = random_position(self.width, self.width),
            Some(Tile::Empty) => { self.snake.pop_front(); },
        };

        self.snake.push_back((next_x as usize, next_y as usize));

        StepResult::Alive
    }

    pub fn board(&self) -> Board {
        let mut board = Board::fill(Tile::Empty, self.width, self.height);
        for (x, y) in self.snake.iter() {
            *board.get_mut(*x, *y) = Tile::Snake;
        }
        *board.get_mut(self.food.0, self.food.1) = Tile::Food;
        board
    }
}
