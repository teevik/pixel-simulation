use crate::components::PixelSimulation;
use crate::pixel_simulation::CHUNK_WORLD_SIZE;
use bevy::math::{Vec2, Vec3Swizzles};
use bevy::prelude::{Color, Gizmos, GlobalTransform, Query, With};

pub fn chunk_gizmo(query: Query<&GlobalTransform, With<PixelSimulation>>, mut gizmos: Gizmos) {
    for transform in query.iter() {
        gizmos.rect_2d(
            transform.translation().xy() + Vec2::splat(CHUNK_WORLD_SIZE / 2.),
            0.,
            Vec2::splat(CHUNK_WORLD_SIZE),
            Color::BLACK,
        );
    }
}
