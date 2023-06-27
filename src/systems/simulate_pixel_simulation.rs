use crate::components::PixelSimulation;
use crate::pixel_simulation::cell_model::CellBehaviour;
use crate::pixel_simulation::chunk::Chunk;
use crate::pixel_simulation::CHUNK_CELLS_SIZE;
use bevy::prelude::{Local, Query};
use rand::random;

fn simulate_chunk(chunk: &mut Chunk, last_updated: bool) {
    let is_even_iteration = last_updated;

    let horizontal_range = if is_even_iteration {
        itertools::Either::Left(0..CHUNK_CELLS_SIZE)
    } else {
        itertools::Either::Right((0..CHUNK_CELLS_SIZE).rev())
    };

    // Switch between iterating left and right
    // Always iterate from bottom to top

    let cells = &mut chunk.cells;

    for x in horizontal_range {
        for y in 0..CHUNK_CELLS_SIZE {
            let cell = cells.get((x, y)).expect("Never out of bounds").clone();
            let Some(mut cell) = cell else { continue };

            if cell.last_updated == last_updated {
                continue;
            };

            cell.last_updated = last_updated;

            let mut try_move = |x_offset: i32, y_offset: i32| -> bool {
                let cell = cell.clone();

                let (target_x, target_y) = (x as i32 + x_offset, y as i32 + y_offset);

                if target_x < 0
                    || target_y < 0
                    || target_x >= CHUNK_CELLS_SIZE as i32
                    || target_y >= CHUNK_CELLS_SIZE as i32
                {
                    return false;
                }

                let (target_x, target_y) = (target_x as usize, target_y as usize);

                if matches!(
                    cells
                        .get((target_x, target_y))
                        .expect("Never out of bounds"),
                    None
                ) {
                    cells[(x, y)] = None;
                    cells[(target_x, target_y)] = Some(cell);

                    true
                } else {
                    false
                }
            };

            match cell.model.behavior {
                CellBehaviour::Solid => {}
                CellBehaviour::Powder => {
                    if !try_move(0, -1) {
                        if random() {
                            if !try_move(1, -1) {
                                try_move(-1, -1);
                            }
                        } else {
                            if !try_move(-1, -1) {
                                try_move(-1, -1);
                            }
                        }
                    }
                    // if matches!(cells.get((x, y + 1)), None)
                }
            }
        }
    }
    // for ((x, y), cell) in chunk
    //     .cells
    //     .indexed_iter()
    //     .map(|(position, cell)| (position, cell.clone()))
    // {
    //     chunk.cells[(1, 1)] = None;
    // }
}

pub fn simulate_pixel_simulation(
    mut query: Query<&mut PixelSimulation>,
    mut last_updated: Local<bool>,
) {
    *last_updated = !*last_updated;

    for mut pixel_simulation in query.iter_mut() {
        simulate_chunk(&mut pixel_simulation.chunk, *last_updated);
    }
}
