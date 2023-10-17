use crate::color::ColorGradient;
use bevy::utils::HashMap;
use palette::{FromColor, Srgba};
use rand::{thread_rng, Rng};

pub enum CellBehaviour {
    Solid,
    Powder,
    Liquid,
}

pub enum Chance {
    Always,
    Sometimes(f64),
}

pub struct Reaction {
    pub self_turns_into: Option<&'static str>,
    pub other_turns_into: Option<&'static str>,
    pub chance: Chance,
    pub one_way: bool,
}

pub struct CellModel {
    pub id: &'static str,
    pub name: &'static str,
    pub behavior: CellBehaviour,
    pub density: f32,
    pub color: ColorGradient, // TODO Solid color or gradient?
    pub reactions: HashMap<&'static str, Reaction>,
}

impl CellModel {
    pub fn generate_color(&self) -> Srgba<u8> {
        let factor = thread_rng().gen_range(0. ..1.);

        let lch_color = self.color.get(factor);

        Srgba::from_color(lch_color).into_format()
    }
}
