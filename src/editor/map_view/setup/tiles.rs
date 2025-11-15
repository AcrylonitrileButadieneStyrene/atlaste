use bevy::{
    prelude::*,
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};

pub fn on_spawn(setup: On<super::Spawn>, mut commands: Commands) {
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
