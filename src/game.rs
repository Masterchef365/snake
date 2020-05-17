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

    pub fn step(&mut self) -> StepResult {
        let (x, y) = self.snake.back().expect("Snake has no head");
        let (dx, dy) = self.direction.delta();
        let (next_x, next_y) = (*x as isize + dx, *y as isize + dy);
        let (nx, ny) = (next_x as usize, next_y as usize);

        if next_x < 0 || nx > self.width || next_y < 0 || ny > self.height {
            return StepResult::Died;
        }

        if self.snake.contains(&(nx, ny)) {
            return StepResult::Died;
        }

        let (food_x, food_y) = self.food;

        if nx == food_x || ny == food_y {
            self.food = random_position(self.width, self.width);
        } else {
            self.snake.pop_front();
        }

        self.snake.push_back((nx, ny));

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
