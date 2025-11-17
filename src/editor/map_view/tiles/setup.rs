use bevy::{
    prelude::*,
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};

use crate::editor::map_view::map_unit::MapUnit;

pub fn on_add_chipset(
    event: On<Add, super::MapChipSet>,
    query: Query<(&MapUnit, &super::MapChipSet)>,
    mut commands: Commands,
) -> Result {
    let (MapUnit { map, .. }, chipset) = query.get(event.entity)?;

    let chunk_size = UVec2::new(map.width, map.height);

    commands.spawn((
        Name::new("Tiles"),
        ChildOf(event.entity),
        Transform::IDENTITY,
        Visibility::Inherited,
        Pickable::IGNORE,
        Children::spawn((
            Spawn((
                Name::new("Lower"),
                Pickable::IGNORE,
                TilemapChunk {
                    tileset: chipset.0.clone(),
                    tile_display_size: UVec2::splat(1),
                    chunk_size,
                    alpha_mode: bevy::sprite_render::AlphaMode2d::Blend,
                },
                TilemapChunkTileData(
                    (0..chunk_size.element_product() as usize)
                        .map(|i| Some(TileData::from_tileset_index(convert(map.lower[i]))))
                        .collect(),
                ),
            )),
            Spawn((Name::new("Events"), Pickable::IGNORE)), // todo
            Spawn((Name::new("Upper"), Pickable::IGNORE)),  // todo
        )),
    ));

    Ok(())
}

#[must_use]
const fn convert(id: u16) -> u16 {
    match id {
        // ground layer unanimated
        5000..=5143 => {
            let index = id - 5000;
            let col = index % 6;
            let base = 12 + (index / 96) * 6;
            (index % 96 - col) * 5 + col + base
        }
        4000..4600 => id - 3520,
        _ => 0, // todo
    }
}
