use atlaste_lcf::MapUnitAsset;
use bevy::{
    prelude::*,
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};

use crate::state::GameData;

pub mod chipset;
pub mod map_unit;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, chipset::setup)
            .add_systems(
                Update,
                (map_unit::check_load, chipset::check_load).run_if(resource_exists::<GameData>),
            )
            .add_observer(on_add)
            .add_observer(on_finalize_setup);
    }
}

#[derive(EntityEvent)]
pub struct FinalizeSetup {
    entity: Entity,
    map_unit: Handle<MapUnitAsset>,
    chipset: Handle<Image>,
}

pub fn on_add(
    trigger: On<atlaste_ui::sections::map_tree::EntryClicked>,
    mut commands: Commands,
    game: Res<GameData>,
    asset_server: Res<AssetServer>,
) {
    let map =
        asset_server.load::<MapUnitAsset>(game.game_dir.join(format!("Map{:0>4}.lmu", trigger.0)));

    commands.spawn((
        map_unit::MapUnit(map),
        map_unit::Loading,
        Transform::default(),
        Visibility::default(),
    ));
}

pub fn on_finalize_setup(
    setup: On<FinalizeSetup>,
    mut commands: Commands,
    map_units: Res<Assets<MapUnitAsset>>,
) {
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
