#![feature(lazy_cell)]
#![feature(try_blocks)]
#![feature(let_chains)]

mod color;
mod components;
mod pixel_simulation;
mod systems;

use bevy::prelude::*;
use pixel_simulation::TICKS_PER_SECOND;
use systems::{
    chunk_gizmo, render_pixel_simulation, setup_pixel_simulation, simulate_pixel_simulation,
    update_pixel_simulation,
};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resizable: false,

            ..default()
        }),
        ..default()
    }));

    app.insert_resource(ClearColor(Color::rgb_u8(234, 231, 217)));

    app.add_systems(Startup, setup_pixel_simulation::setup_pixel_simulation);

    app.add_systems(Update, update_pixel_simulation::update_pixel_simulation);
    app.add_systems(Update, chunk_gizmo::chunk_gizmo);

    app.add_systems(
        FixedUpdate,
        simulate_pixel_simulation::simulate_pixel_simulation,
    );
    app.insert_resource(FixedTime::new_from_secs(1. / TICKS_PER_SECOND));

    app.add_systems(Update, bevy::window::close_on_esc);

    app.add_systems(PostUpdate, render_pixel_simulation::render_pixel_simulation);

    app.run();
}
