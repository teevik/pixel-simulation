use crate::color::ColorGradient;
use palette::{FromColor, Srgba};
use rand::{thread_rng, Rng};

pub enum CellBehaviour {
    Solid,
    Powder,
    // Liquid,
}

pub struct CellModel {
    pub id: u32,
    pub name: &'static str,
    pub behavior: CellBehaviour,
    pub density: f32,
    pub color: ColorGradient, // TODO Solid color or gradient?
}

impl CellModel {
    pub fn generate_color(&self) -> Srgba<u8> {
        let factor = thread_rng().gen_range(0. ..1.);

        let lch_color = self.color.get(factor);

        Srgba::from_color(lch_color).into_format()
    }
}
