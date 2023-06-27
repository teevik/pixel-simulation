use crate::color::ColorGradient;
use bevy::utils::HashMap;
use palette::lch::Lch;
use std::sync::LazyLock;
use crate::pixel_simulation::cell_model::{CellBehaviour, CellModel};

fn generate_cell_models() -> HashMap<u32, CellModel> {
    let stone_wall_model = CellModel {
        id: 1,
        name: "Stone Wall",
        behavior: CellBehaviour::Solid,
        density: 2550.,
        color: ColorGradient {
            from: Lch::new(43., 0., 0.),
            to: Lch::new(22., 0., 0.),
        },
    };

    let sand_model = CellModel {
        id: 2,
        name: "Sand",
        behavior: CellBehaviour::Powder,
        density: 1602.,
        color: ColorGradient {
            from: Lch::new(78.0, 25.0, 92.0),
            to: Lch::new(83.0, 25.0, 92.0),
        },
    };

    HashMap::from_iter([stone_wall_model, sand_model].map(|model| (model.id, model)))
}

pub static CELL_MODELS: LazyLock<HashMap<u32, CellModel>> = LazyLock::new(generate_cell_models);
