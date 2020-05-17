pub struct Board {
    pub tiles: Box<[Tile]>,
    pub width: usize,
}

impl Board {
    pub fn empty(width: usize, height: usize) -> Self {
        Self {
            tiles: vec![Tile::Empty; width * height].into(),
            width,
        }
    }

    pub fn rows(&self) -> impl Iterator<Item=&[Tile]> {
        self.tiles.chunks_exact(self.width)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Tile {
    Empty,
    Snake,
    Food,
}
