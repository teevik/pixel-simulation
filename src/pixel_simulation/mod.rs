use palette::FromColor;
use rand::Rng;

pub mod cell;
pub mod cell_model;
pub mod cell_models;
pub mod chunk;

pub const CHUNK_CELLS_SIZE: usize = 200;
pub const CHUNK_WORLD_SIZE: f32 = 800.;
pub const TICKS_PER_SECOND: f64 = 20.;
