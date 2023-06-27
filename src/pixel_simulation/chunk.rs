use crate::pixel_simulation::cell::Cell;
use crate::pixel_simulation::CHUNK_CELLS_SIZE;
use ndarray::Array2;

// pub type

pub struct Chunk {
    pub cells: Array2<Option<Cell>>,
}

impl Chunk {
    pub fn new() -> Self {
        let cells = Array2::from_shape_fn((CHUNK_CELLS_SIZE, CHUNK_CELLS_SIZE), |(x, y)| None);

        Self { cells }
    }
}
