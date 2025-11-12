use std::sync::Arc;

use atlaste_lcf::lcf::lmu::LcfMapUnit;
use bevy::{
    color::palettes::{basic::BLACK, tailwind},
    prelude::*,
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};

#[derive(EntityEvent)]
pub struct Finalize {
    pub entity: Entity,
    pub map: Arc<LcfMapUnit>,
    pub chipset: Handle<Image>,
}

pub fn on_setup_tiles(setup: On<Finalize>, mut commands: Commands) {
    let chunk_size = UVec2::new(setup.map.width, setup.map.height);
    let convert = |id| -> u16 {
        match id {
            // ground layer unanimated
            5000..=5143 => {
                let index = id - 5000;
                let col = index % 6;
                let base = 12 + (index / 96) * 6;
                (index % 96 - col) * 5 + col + base
            }
            _ => 0, // todo
        }
    };

    commands.spawn((
        Name::new("Map tiles"),
        ChildOf(setup.entity),
        Children::spawn((
            Spawn((
                Name::new("Lower tiles"),
                TilemapChunk {
                    tileset: setup.chipset.clone(),
                    tile_display_size: UVec2::splat(1),
                    chunk_size,
                    alpha_mode: bevy::sprite_render::AlphaMode2d::Opaque,
                },
                TilemapChunkTileData(
                    (0..chunk_size.element_product() as usize)
                        .map(|i| Some(TileData::from_tileset_index(convert(setup.map.lower[i]))))
                        .collect(),
                ),
            )),
            Spawn((Name::new("Events"),)),      // todo
            Spawn((Name::new("Upper tiles"),)), // todo
        )),
    ));
}

pub fn on_setup_background(
    setup: On<Finalize>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Name::new("Map background"),
        ChildOf(setup.entity),
        Transform::from_translation(-Vec3::Z),
        Children::spawn((
            // furthest back
            Spawn((
                Name::new("Outline"),
                Mesh2d(meshes.add(Rectangle::new(
                    setup.map.width as f32 + 0.2,
                    setup.map.height as f32 + 0.2,
                ))),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(tailwind::ZINC_600))),
                ChildOf(setup.entity),
            )),
            // prevent transparency from seeing the window outline
            Spawn((
                Name::new("Backdrop"),
                Mesh2d(meshes.add(Rectangle::new(
                    setup.map.width as f32,
                    setup.map.height as f32,
                ))),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(BLACK))),
                ChildOf(setup.entity),
            )),
            Spawn((Name::new("Panorama"),)), // todo, will require shader (probably)
        )),
    ));
}
