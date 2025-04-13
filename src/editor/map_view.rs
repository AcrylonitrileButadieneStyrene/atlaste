use bevy::{asset::LoadState, prelude::*};
use bevy_ecs_tilemap::prelude::*;

use crate::{
    lcf_asset_loader::{DataBaseAsset, GameAssets, MapUnitAsset},
    state::{CurrentCodePage, GamePath},
};

#[derive(Event)]
pub struct Add(pub u16);

#[derive(Component)]
pub struct Loading;

#[derive(Event)]
pub struct Setup(Entity);

#[derive(Component)]
pub struct MapUnit(Handle<MapUnitAsset>);

const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 16.0, y: 16.0 };
const GRID_SIZE: TilemapGridSize = TilemapGridSize { x: 16.0, y: 16.0 };

pub fn on_add(
    trigger: Trigger<Add>,
    mut commands: Commands,
    game_path: Res<GamePath>,
    asset_server: Res<AssetServer>,
) {
    let map = asset_server
        .load::<MapUnitAsset>(game_path.with(|p| p.join(format!("Map{:0>4}.lmu", trigger.0))));

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
                let mut ent = commands.entity(entity);
                ent.trigger(Setup(entity));
                ent.remove::<Loading>();
            }
            LoadState::Failed(err) => {
                log::error!("Failed to load map: {err:?}");
                commands.entity(entity).despawn_recursive();
            }
        });
}

// This runs very infrequently (when a map is added by the user)
// So it's more efficient to run it as an observer than an event reader
// Observers do not parallelize at all, they have exclusive world access
// So this system shall be oversized.
#[allow(clippy::too_many_arguments)]
pub fn setup_view(
    trigger: Trigger<Setup>,
    mut commands: Commands,
    query: Query<&MapUnit>,
    map_units: Res<Assets<MapUnitAsset>>,
    asset_server: Res<AssetServer>,
    game_assets: Res<GameAssets>,
    databases: Res<Assets<DataBaseAsset>>,
    code_page: Res<CurrentCodePage>,
    game_path: Res<GamePath>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let map_unit = &query.get(trigger.0).unwrap();
    let map = &map_units.get(map_unit.0.id()).unwrap().0;

    let database = databases.get(game_assets.database.id()).unwrap();
    let Some(chipset) = database.0.chipsets[map.chipset as usize - 1].file.as_ref() else {
        log::error!("Map is missing a chipset");
        return;
    };
    let file = code_page.0.to_encoding().decode(chipset).0.to_string();
    let texture =
        asset_server.load::<Image>(game_path.with(|p| p.join("ChipSet/").join(file + ".png"))); // TODO: it can be a .bmp too
    let size = TilemapSize::new(map.width, map.height);
    let mut storage = TileStorage::empty(size);

    let tilemap = commands.spawn_empty().set_parent(trigger.0).id();

    // TODO: spawn the upper layer
    for (x, y) in itertools::iproduct!(0..size.x, 0..size.y) {
        let tile_pos = TilePos {
            x,
            y: size.y - y - 1,
        };
        let tile_entity = commands
            .spawn(TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(tilemap),
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

    commands.entity(tilemap).insert(TilemapBundle {
        tile_size: TILE_SIZE,
        grid_size: GRID_SIZE,
        map_type: TilemapType::Square,
        texture: TilemapTexture::Single(texture),
        size,
        storage,
        ..Default::default()
    });

    let x = size.x as f32 * GRID_SIZE.x;
    let y = size.y as f32 * GRID_SIZE.y;

    commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::default())),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::hsl(0., 0., 0.3)))),
            Transform::from_translation(Vec3::new(
                (x - GRID_SIZE.x) / 2.0,
                (y - GRID_SIZE.y) / 2.0,
                -1.,
            ))
            .with_scale(Vec3::new(x, y, 1.0)),
        ))
        .set_parent(trigger.0);
}
