use crate::color::ColorGradient;
use crate::pixel_simulation::cell_model::{CellBehaviour, CellModel, Chance, Reaction};
use bevy::utils::HashMap;
use palette::lch::Lch;
use std::sync::LazyLock;

fn generate_cell_models() -> HashMap<&'static str, CellModel> {
    let stone_wall_model = CellModel {
        id: "stone_wall",
        name: "Stone Wall",
        behavior: CellBehaviour::Solid,
        density: 2550.,
        color: ColorGradient {
            from: Lch::new(43., 0., 0.),
            to: Lch::new(22., 0., 0.),
        },
        reactions: HashMap::from([]),
    };

    let sand_model = CellModel {
        id: "sand",
        name: "Sand",
        behavior: CellBehaviour::Powder,
        density: 1602.,
        color: ColorGradient {
            from: Lch::new(78.0, 25.0, 92.0),
            to: Lch::new(83.0, 25.0, 92.0),
        },
        reactions: HashMap::from([]),
    };

    let water_model = CellModel {
        id: "water",
        name: "Water",
        behavior: CellBehaviour::Liquid,
        density: 997.,
        color: ColorGradient {
            from: Lch::new(65.0, 37.0, 249.0),
            to: Lch::new(70.0, 37.0, 249.0),
        },
        reactions: HashMap::from([(
            "sand",
            Reaction {
                self_turns_into: None,
                other_turns_into: Some("stone_wall"),
                one_way: false,
                chance: Chance::Always,
            },
        )]),
    };

    HashMap::from_iter([stone_wall_model, sand_model, water_model].map(|model| (model.id, model)))
}

pub static CELL_MODELS: LazyLock<HashMap<&'static str, CellModel>> =
    LazyLock::new(generate_cell_models);
