#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellState {
    Alive,
    Dead,
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    state: CellState,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            state: CellState::Dead,
        }
    }

    pub fn state(&self) -> CellState {
        self.state
    }

    pub fn toggle(&mut self) {
        self.state = match self.state {
            CellState::Alive => CellState::Dead,
            CellState::Dead => CellState::Alive,
        };
    }
}

pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::new(); width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn cell(&self, x: usize, y: usize) -> &Cell {
        &self.cells[self.index(x, y)]
    }

    pub fn toggle(&mut self, x: usize, y: usize) {
        let index = self.index(x, y);
        self.cells[index].toggle();
    }
}
