use super::Grid;

pub struct GameState {
    grid: Grid,
}

impl GameState {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Grid::new(width, height),
        }
    }

    pub fn grid(&self) -> &Grid {
        &self.grid
    }

    pub fn toggle_cell(&mut self, x: usize, y: usize) {
        self.grid.toggle(x, y);
    }
}
