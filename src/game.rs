use serde::Serialize;

#[derive(Serialize)]
pub struct GameState {
    pub size: usize,
    pub cells: Vec<Cell>,
}

#[derive(Serialize)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub color: String,
}

impl GameState {
    pub fn new(size: usize) -> Self {
        let mut cells = Vec::new();

        for y in 0..size {
            for x in 0..size {
                cells.push(Cell {
                    x,
                    y,
                    color: "white".to_string(),
                });
            }
        }

        Self { size, cells }
    }
}
