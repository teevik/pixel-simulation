use crate::pixel_simulation::cell_model::CellModel;
use palette::Srgba;
use std::num::Wrapping;

#[derive(Clone)]
pub struct Cell {
    pub model: &'static CellModel,
    pub color: Srgba<u8>,
    pub last_updated: Wrapping<u8>,
}

impl Cell {
    pub fn new(model: &'static CellModel, last_updated: Wrapping<u8>) -> Self {
        Self {
            model,
            color: model.generate_color(),
            last_updated,
        }
    }
}
