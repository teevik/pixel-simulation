use crate::components::{MainCamera, PixelSimulation};
use crate::pixel_simulation::cell::Cell;
use crate::pixel_simulation::cell_model::CellModel;
use crate::pixel_simulation::cell_models::CELL_MODELS;
use crate::pixel_simulation::{CHUNK_CELLS_SIZE, CHUNK_WORLD_SIZE};
use bevy::input::Input;
use bevy::math::Vec2;
use bevy::prelude::{
    Camera, GlobalTransform, KeyCode, Local, MouseButton, Query, Res, Window, With,
};
use bevy::window::PrimaryWindow;
use std::default::default;

pub struct SelectedCellModel(&'static CellModel);

impl Default for SelectedCellModel {
    fn default() -> Self {
        Self(CELL_MODELS.get("stone_wall").unwrap())
    }
}

pub fn update_pixel_simulation(
    mut query: Query<&mut PixelSimulation>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    main_camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    mut selected_cell_model: Local<SelectedCellModel>,
) {
    for pressed_key in keys.get_just_pressed() {
        match pressed_key {
            KeyCode::Key1 => {
                *selected_cell_model = SelectedCellModel(CELL_MODELS.get("stone_wall").unwrap());
            }

            KeyCode::Key2 => {
                *selected_cell_model = SelectedCellModel(CELL_MODELS.get("sand").unwrap());
            }

            KeyCode::Key3 => {
                *selected_cell_model = SelectedCellModel(CELL_MODELS.get("water").unwrap());
            }

            _ => {}
        }
    }

    let window = window_query.single();
    let (camera, camera_transform) = main_camera_query.single();

    // let cursor_position = window.cursor_position();

    let world_position: Option<Vec2> = try {
        let cursor_position = window.cursor_position()?;
        let world_position = camera.viewport_to_world_2d(camera_transform, cursor_position)?;

        world_position
    };

    let should_spawn_cell = mouse_button_inputs.pressed(MouseButton::Left);
    let should_erase_cell = mouse_button_inputs.pressed(MouseButton::Right);

    if let Some(world_position) = world_position && (should_spawn_cell || should_erase_cell) {
        for mut pixel_simulation in query.iter_mut() {
            let cell_position = world_position * (CHUNK_CELLS_SIZE as f32) / CHUNK_WORLD_SIZE;
            let cell_position = cell_position.floor().as_ivec2();

            for x_offset in -1..=1 {
                for y_offset in -1..=1 {
                    let (x,y) = (cell_position.x + x_offset, cell_position.y + y_offset);

                    if x < 0 || y<0 || x>= CHUNK_CELLS_SIZE as i32 || y >= CHUNK_CELLS_SIZE as i32 { continue; }
                    let (x, y) = (x as usize, y as usize);

                     if should_spawn_cell {
                         let target_is_empty = pixel_simulation.chunk.cells.get((x, y)).expect("Never out of bounds").is_none();

                         if target_is_empty {
                            pixel_simulation.chunk.cells[(x, y)] = Some(Cell::new((*selected_cell_model).0, default()));
                         }
                    } else {
                         pixel_simulation.chunk.cells[(x, y)] = None;
                    };
                }
            }
        }
    }
}
