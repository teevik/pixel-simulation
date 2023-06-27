use crate::pixel_simulation::chunk::Chunk;
use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct PixelSimulation {
    pub chunk: Chunk,
    pub image_handle: Handle<Image>,
}
