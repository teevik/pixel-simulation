use crate::components::{MainCamera, PixelSimulation};
use crate::pixel_simulation::chunk::Chunk;
use crate::pixel_simulation::{CHUNK_CELLS_SIZE, CHUNK_WORLD_SIZE};
use bevy::asset::Assets;
use bevy::core::Name;
use bevy::hierarchy::BuildChildren;
use bevy::math::Vec2;
use bevy::prelude::{
    default, Camera2dBundle, Commands, Image, ResMut, SpatialBundle, Sprite, SpriteBundle,
};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::texture::ImageSampler;
use bevy::sprite::Anchor;

pub fn setup_pixel_simulation(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let camera_bundle = {
        let mut camera_bundle = Camera2dBundle::default();
        let translation = &mut camera_bundle.transform.translation;

        translation.x = CHUNK_WORLD_SIZE / 2.;
        translation.y = CHUNK_WORLD_SIZE / 2.;

        camera_bundle
    };

    commands.spawn((Name::new("Main Camera"), MainCamera, camera_bundle));

    let mut image = Image::new(
        Extent3d {
            width: CHUNK_CELLS_SIZE as u32,
            height: CHUNK_CELLS_SIZE as u32,
            ..default()
        },
        TextureDimension::D2,
        vec![0; 4 * CHUNK_CELLS_SIZE as usize * CHUNK_CELLS_SIZE as usize],
        TextureFormat::Bgra8UnormSrgb,
    );
    image.sampler = ImageSampler::nearest();

    let image_handle = images.add(image);

    commands
        .spawn((
            Name::new("Chunk"),
            SpatialBundle::default(),
            PixelSimulation {
                chunk: Chunk::new(),
                image_handle: image_handle.clone(),
            },
        ))
        .with_children(|children| {
            children.spawn((
                Name::new("Image"),
                SpriteBundle {
                    texture: image_handle,
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(CHUNK_WORLD_SIZE)),
                        anchor: Anchor::BottomLeft,
                        ..default()
                    },
                    ..default()
                },
            ));
        });
}
