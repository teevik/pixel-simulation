use crate::components::PixelSimulation;
use crate::pixel_simulation::CHUNK_CELLS_SIZE;
use bevy::asset::Assets;
use bevy::prelude::{Changed, Image, Query, ResMut};
use palette::Srgba;

pub fn render_pixel_simulation(
    query: Query<&PixelSimulation, Changed<PixelSimulation>>,
    mut images: ResMut<Assets<Image>>,
) {
    for pixel_simulation in query.iter() {
        let image = images.get_mut(&pixel_simulation.image_handle).unwrap();

        for ((x, y), cell) in pixel_simulation.chunk.cells.indexed_iter() {
            let transformed_y = (CHUNK_CELLS_SIZE as usize - 1) - y;

            let cell_index = (transformed_y * CHUNK_CELLS_SIZE as usize) + x;

            let color = match cell {
                None => Srgba::new(0, 0, 0, 0),
                Some(cell) => cell.color,
            };

            let image_index_start = cell_index * 4;

            image.data[image_index_start] = color.blue;
            image.data[image_index_start + 1] = color.green;
            image.data[image_index_start + 2] = color.red;
            image.data[image_index_start + 3] = color.alpha;
        }
    }
}
