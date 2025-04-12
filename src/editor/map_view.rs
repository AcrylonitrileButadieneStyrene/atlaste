use bevy::{asset::LoadState, prelude::*};
use bevy_ecs_tilemap::prelude::*;

use crate::{
    lcf_asset_loader::{DataBaseAsset, GameAssets, MapUnitAsset},
    state::{CurrentCodePage, GamePath},
};

#[derive(Event)]
pub struct Add(pub u16);

#[derive(Component)]
pub struct MapUnit(Handle<MapUnitAsset>);

#[derive(Component)]
pub struct Loading;

#[derive(Event)]
pub struct Loaded(Entity);

pub fn on_add(
    trigger: Trigger<Add>,
    mut commands: Commands,
    game_path: Res<GamePath>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let file = format!("Map{:0>4}.lmu", trigger.0);
    let map = asset_server.load::<MapUnitAsset>(game_path.with(|p| p.join(&file)));
    commands
        .spawn(Mesh2d(meshes.add(Rectangle::default())))
        .with_child((MapUnit(map), Loading, Transform::from_xyz(0., 0., 0.)));
}

pub fn process_loading(
    mut commands: Commands,
    query: Query<(Entity, &MapUnit), With<Loading>>,
    asset_server: Res<AssetServer>,
    mut loaded_events: EventWriter<Loaded>,
) {
    query
        .iter()
        .for_each(|(entity_id, map)| match asset_server.load_state(&map.0) {
            LoadState::NotLoaded | LoadState::Loading => (),
            LoadState::Loaded => {
                if let Some(mut entity) = commands.get_entity(entity_id) {
                    entity.remove::<Loading>();
                    loaded_events.send(Loaded(entity_id));
                }
            }
            LoadState::Failed(err) => {
                log::error!("Failed to load map: {err:?}");
            }
        });
}

const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 16.0, y: 16.0 };
const GRID_SIZE: TilemapGridSize = TilemapGridSize { x: 16.0, y: 16.0 };

// TODO: break this up into multiple systems
pub fn process_loaded(
    mut commands: Commands,
    mut loaded: EventReader<Loaded>,
    map_unit: Query<&MapUnit>,
    map_units: Res<Assets<MapUnitAsset>>,

    asset_server: Res<AssetServer>,

    game_assets: Res<GameAssets>,
    databases: Res<Assets<DataBaseAsset>>,
    code_page: Res<CurrentCodePage>,
    game_path: Res<GamePath>,
) {
    for Loaded(entity) in loaded.read() {
        let map = &map_unit
            .get(*entity)
            .ok()
            .and_then(|map_unit| map_units.get(map_unit.0.id()))
            .unwrap()
            .0;

        let database = databases.get(game_assets.database.id()).unwrap();
        let Some(chipset) = database.0.chipsets[map.chipset as usize - 1].file.as_ref() else {
            log::warn!("Map is missing a chipset");
            return;
        };
        let file = code_page.0.to_encoding().decode(chipset).0.to_string();

        let texture =
            asset_server.load::<Image>(game_path.with(|p| p.join("ChipSet/").join(file + ".png"))); // TODO: it can be a .bmp too
        let size = TilemapSize::new(map.width, map.height);
        let mut storage = TileStorage::empty(size);

        // TODO: spawn the upper layer
        for (x, y) in itertools::iproduct!(0..size.x, 0..size.y) {
            let tile_pos = TilePos {
                x,
                y: size.y - y - 1,
            };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(*entity),
                    texture_index: TileTextureIndex(
                        lcf_rs::LcfMapUnit::convert_layer_to_chipset_index(
                            map.lower[(y * size.x + x) as usize] as usize,
                        ) as u32,
                    ),
                    ..Default::default()
                })
                .id();
            storage.set(&tile_pos, tile_entity);
        }

        commands.entity(*entity).insert(TilemapBundle {
            tile_size: TILE_SIZE,
            grid_size: GRID_SIZE,
            map_type: TilemapType::Square,
            texture: TilemapTexture::Single(texture),
            size,
            storage,
            ..Default::default()
        });
    }
}
