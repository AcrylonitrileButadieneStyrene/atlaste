use std::sync::Arc;

use atlaste_lcf::lcf::lmu::LcfMapUnit;
use bevy::{
    color::palettes::basic::BLACK,
    prelude::*,
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};

#[derive(EntityEvent)]
pub struct Spawn {
    pub entity: Entity,
    pub map: Arc<LcfMapUnit>,
    pub chipset: Handle<Image>,
}

pub fn on_spawn_tiles(setup: On<Spawn>, mut commands: Commands) {
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
        Name::new("Tiles"),
        ChildOf(setup.entity),
        Transform::IDENTITY,
        Visibility::Inherited,
        Pickable::IGNORE,
        Children::spawn((
            Spawn((
                Name::new("Lower"),
                Pickable::IGNORE,
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
            Spawn((Name::new("Events"), Pickable::IGNORE)), // todo
            Spawn((Name::new("Upper"), Pickable::IGNORE)),  // todo
        )),
    ));
}

pub fn on_spawn_background(
    setup: On<Spawn>,
    mut commands: Commands,
    rectangle: Res<crate::utils::unit_mesh::UnitRectangle>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Name::new("Background"),
        ChildOf(setup.entity),
        Transform::from_translation(-Vec3::Z),
        Visibility::Inherited,
        crate::editor::select::Selectable(
            Vec2::new(setup.map.width as f32 + 0.2, setup.map.height as f32 + 0.2),
            -2.0,
        ),
        Children::spawn((
            // prevent transparency from seeing the window outline
            Spawn((
                Name::new("Cover"),
                Transform::from_translation(Vec3::new(0., 0., -1.)).with_scale(Vec3::new(
                    setup.map.width as f32,
                    setup.map.height as f32,
                    1.0,
                )),
                Pickable::IGNORE,
                Mesh2d(rectangle.0.clone()),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(BLACK))),
            )),
        )),
    ));
}
