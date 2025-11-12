use atlaste_lcf::MapUnitAsset;
use bevy::{asset::LoadState, prelude::*};

use crate::{
    editor::map_view::map_unit,
    state::{CurrentCodePage, GameData},
};

#[derive(Component)]
pub struct MapUnit(pub Handle<MapUnitAsset>);

#[derive(Component)]
pub struct Loading;

pub fn check_load(
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &MapUnit), With<Loading>>,
    map_units: Res<Assets<MapUnitAsset>>,
    game: Res<GameData>,
    code_page: Res<CurrentCodePage>,
    mut commands: Commands,
    fallback: Res<super::chipset::Fallback>,
) {
    for (entity, map_view) in query.iter() {
        let mut entity = commands.entity(entity);
        let load = match asset_server.get_load_state(&map_view.0) {
            Some(LoadState::Loading) => continue,
            Some(LoadState::Loaded) => true,
            Some(LoadState::Failed(_)) => false,
            Some(LoadState::NotLoaded) | None => {
                error!("Unloaded map unit was marked as loading");
                false
            }
        };

        entity.remove::<map_unit::Loading>();

        if !load {
            continue;
        }

        // will always be present, it just loaded
        let map = &map_units.get(&map_view.0).unwrap();

        let texture = match map.chipset {
            Some(chipset) => {
                let chipset = &game.database.chipsets[chipset as usize - 1].file;
                let file = code_page.0.to_encoding().decode(chipset).0.to_string();

                // bevy 18 will add a setting to loading images but it does not help me because chipsets are not 1x480, they are 30x16
                asset_server.load(game.game_dir.join("ChipSet/").join(file + ".png")) // TODO: it can be a .bmp too
            }
            None => fallback.0.clone(),
        };

        entity.insert(super::chipset::Loading(texture));
    }
}
