use atlaste_lcf::MapUnitAsset;
use bevy::{
    asset::LoadState,
    prelude::*,
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};

use crate::state::{CurrentCodePage, GameData};

#[derive(Event)]
pub struct Add(pub u32);

#[derive(Component)]
pub struct Loading;

#[derive(EntityEvent)]
pub struct Setup(Entity);

#[derive(Component)]
pub struct MapUnit(Handle<MapUnitAsset>);

pub fn on_add(
    trigger: On<Add>,
    mut commands: Commands,
    game: Res<GameData>,
    asset_server: Res<AssetServer>,
) {
    let map =
        asset_server.load::<MapUnitAsset>(game.game_dir.join(format!("Map{:0>4}.lmu", trigger.0)));

    commands.spawn((
        Loading,
        MapUnit(map),
        Transform::default(),
        Visibility::default(),
    ));
}

pub fn process_loading(
    mut commands: Commands,
    query: Query<(Entity, &MapUnit), With<Loading>>,
    asset_server: Res<AssetServer>,
) {
    query
        .iter()
        .for_each(|(entity, map)| match asset_server.load_state(&map.0) {
            LoadState::NotLoaded | LoadState::Loading => (),
            LoadState::Loaded => {
                commands.entity(entity).remove::<Loading>();
                commands.trigger(Setup(entity));
            }
            LoadState::Failed(err) => {
                log::error!("Failed to load map: {err:?}");
                commands.entity(entity).despawn();
            }
        });
}

#[allow(clippy::too_many_arguments)]
pub fn setup_view(
    trigger: On<Setup>,
    mut commands: Commands,
    query: Query<&MapUnit>,
    map_units: Res<Assets<MapUnitAsset>>,
    asset_server: Res<AssetServer>,
    game: Res<GameData>,
    code_page: Res<CurrentCodePage>,
) {
    let map_unit = &query.get(trigger.0).unwrap();
    let map = &map_units.get(map_unit.0.id()).unwrap().0;

    let chipset = &game.database.chipsets[map.chipset.unwrap() as usize - 1].file;
    let file = code_page.0.to_encoding().decode(chipset).0.to_string();
    let texture = asset_server.load::<Image>(game.game_dir.join("ChipSet/").join(file + ".png")); // TODO: it can be a .bmp too

    let chunk_size = UVec2::new(map.width, map.height);

    commands.spawn((
        TilemapChunk {
            tileset: texture,
            tile_display_size: UVec2::splat(1),
            chunk_size,
            alpha_mode: bevy::sprite_render::AlphaMode2d::Opaque,
        },
        TilemapChunkTileData(
            (0..chunk_size.element_product() as usize)
                .map(|i| {
                    Some(TileData::from_tileset_index(
                        convert_layer_to_chipset_index(map.lower[i]),
                    ))
                })
                .collect(),
        ),
        ChildOf(trigger.0),
    ));
}

#[must_use]
const fn convert_layer_to_chipset_index(id: u16) -> u16 {
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
}
