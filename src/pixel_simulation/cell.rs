use crate::pixel_simulation::cell_model::CellModel;
use palette::Srgba;

#[derive(Clone)]
pub struct Cell {
    pub model: &'static CellModel,
    pub color: Srgba<u8>,
    pub last_updated: bool,
}

impl Cell {
    pub fn new(model: &'static CellModel) -> Self {
        Self {
            model,
            color: model.generate_color(),
            last_updated: false,
        }
    }
}
