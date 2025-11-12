use atlaste_lcf::MapUnitAsset;
use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};

use crate::state::{CurrentCodePage, GameData};

#[derive(Resource)]
pub struct NullChipSet(pub Handle<Image>);

pub fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let texture = images.add(Image::new_fill(
        Extent3d {
            width: 30 * 16,
            height: 16 * 16,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0x00, 0xFF, 0x00, 0xFF],
        TextureFormat::bevy_default(),
        RenderAssetUsages::RENDER_WORLD,
    ));
    commands.insert_resource(NullChipSet(texture));
}

#[derive(EntityEvent)]
pub struct Setup {
    entity: Entity,
    map_unit: Handle<MapUnitAsset>,
    chipset: Handle<Image>,
}

#[derive(Component)]
pub struct MapUnit(Handle<MapUnitAsset>);

#[derive(Component)]
pub struct LoadingChipset(Handle<Image>);

pub fn on_add(
    trigger: On<atlaste_ui::sections::map_tree::EntryClicked>,
    mut commands: Commands,
    game: Res<GameData>,
    asset_server: Res<AssetServer>,
) {
    let map =
        asset_server.load::<MapUnitAsset>(game.game_dir.join(format!("Map{:0>4}.lmu", trigger.0)));

    commands.spawn((MapUnit(map), Transform::default(), Visibility::default()));
}

#[allow(clippy::too_many_arguments)]
pub fn on_map_unit_load(
    mut messages: MessageReader<AssetEvent<MapUnitAsset>>,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &MapUnit)>,
    map_units: Res<Assets<MapUnitAsset>>,
    game: Res<GameData>,
    code_page: Res<CurrentCodePage>,
    mut commands: Commands,
    null_chipset: Res<NullChipSet>,
) {
    let finished_loading = messages
        .read()
        .filter_map(|message| match message {
            AssetEvent::LoadedWithDependencies { id } => Some(id),
            _ => None,
        })
        .filter_map(|id| asset_server.get_id_handle(*id))
        .collect::<Vec<_>>();

    if !finished_loading.is_empty() {
        for (entity, map_view) in query.iter() {
            if finished_loading.contains(&map_view.0) {
                // will always be present, it just loaded
                let map = &map_units.get(&map_view.0).unwrap();

                let texture = match map.chipset {
                    Some(chipset) => {
                        let chipset = &game.database.chipsets[chipset as usize - 1].file;
                        let file = code_page.0.to_encoding().decode(chipset).0.to_string();

                        // bevy 18 will add a setting to loading images but it does not help me because chipsets are not 1x480, they are 30x16
                        asset_server.load(game.game_dir.join("ChipSet/").join(file + ".png")) // TODO: it can be a .bmp too
                    }
                    None => null_chipset.0.clone(),
                };

                commands.entity(entity).insert(LoadingChipset(texture));
            }
        }
    }
}

pub fn on_image_load(
    mut messages: MessageReader<AssetEvent<Image>>,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &MapUnit, &LoadingChipset)>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
) {
    // TODO: if the image does not load then use the null chipset
    let finished_loading = messages
        .read()
        .filter_map(|message| match message {
            AssetEvent::LoadedWithDependencies { id } => Some(id),
            _ => None,
        })
        .filter_map(|id| asset_server.get_id_handle(*id))
        .collect::<Vec<_>>();
    if !finished_loading.is_empty() {
        for (entity, map_unit, chipset) in query.iter() {
            if finished_loading.contains(&chipset.0) {
                let image_handle = chipset.0.clone();
                let image = images.get_mut(&image_handle).unwrap();
                let pixels = image.data.take().unwrap();
                // wgpu reads the elements as lines on the image instead of as squares, so they need to be repacked
                image.data = Some(fix_pixels(pixels));
                image.reinterpret_size(Extent3d {
                    width: 16,
                    height: 16,
                    depth_or_array_layers: 480,
                });

                commands.entity(entity).remove::<LoadingChipset>();
                commands.trigger(Setup {
                    entity,
                    map_unit: map_unit.0.clone(),
                    chipset: image_handle,
                });
            }
        }
    }
}

pub fn setup_view(setup: On<Setup>, mut commands: Commands, map_units: Res<Assets<MapUnitAsset>>) {
    let map = &map_units.get(setup.map_unit.id()).unwrap().0;
    let chunk_size = UVec2::new(map.width, map.height);

    commands.spawn((
        TilemapChunk {
            tileset: setup.chipset.clone(),
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
        ChildOf(setup.entity),
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

// there is definitely a better way to do this but this worked first try so i will not be changing it
#[must_use]
fn fix_pixels(pixels: Vec<u8>) -> Vec<u8> {
    let mut new_pixels = Vec::with_capacity(pixels.len());

    for tile_index in 0..480 {
        let start_x = tile_index % 30;
        let start_y = tile_index / 30;
        for square_index in 0..256 {
            let add_x = square_index % 16;
            let add_y = square_index / 16;
            for byte in 0..4 {
                let pixel =
                    pixels[(start_x * 16 + add_x + (start_y * 16 + add_y) * 480) * 4 + byte];
                new_pixels.push(pixel);
            }
        }
    }

    new_pixels
}
