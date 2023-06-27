use palette::{Lch, Mix};

pub struct ColorGradient {
    pub from: Lch,
    pub to: Lch,
}

impl ColorGradient {
    pub fn get(&self, factor: f32) -> Lch {
        Lch::mix(self.from, self.to, factor)
    }
}
