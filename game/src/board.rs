pub struct Board {
    pub tiles: Box<[Tile]>,
    pub width: usize,
}

impl Board {
    pub fn fill(tile: Tile, width: usize, height: usize) -> Self {
        Self {
            tiles: vec![tile; width * height].into(),
            width,
        }
    }

    pub fn height(&self) -> usize {
        self.tiles.len() / self.width
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(y * self.width + x)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        self.tiles.get_mut(y * self.width + x)
    }

    pub fn rows(&self) -> impl Iterator<Item = &[Tile]> {
        self.tiles.chunks_exact(self.width)
    }

    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [Tile]> {
        self.tiles.chunks_exact_mut(self.width)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tile {
    Empty,
    Snake,
    Food,
}
