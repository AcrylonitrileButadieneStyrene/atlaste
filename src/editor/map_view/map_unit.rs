use atlaste_lcf::MapUnitAsset;
use bevy::{asset::LoadState, prelude::*};

use crate::state::{CurrentCodePage, GameData};

#[derive(Component)]
pub struct Loading(pub Handle<MapUnitAsset>);

pub fn check_load(
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &Loading), Without<super::chipset::Loading>>,
    map_units: Res<Assets<MapUnitAsset>>,
    game: Res<GameData>,
    code_page: Res<CurrentCodePage>,
    mut commands: Commands,
    fallback: Res<super::chipset::Fallback>,
) {
    for (entity, loading) in query.iter() {
        match asset_server.get_load_state(&loading.0) {
            Some(LoadState::NotLoaded) | None => {
                error!("Unloaded map unit was marked as loading");
                continue;
            }
            Some(LoadState::Loading) => continue,
            Some(LoadState::Failed(_)) => {
                error!("Unloaded map unit was marked as loading");
                continue;
            }
            Some(LoadState::Loaded) => {
                // will always be present, it just loaded
                let map = &map_units.get(&loading.0).unwrap();

                let texture = match map.chipset {
                    Some(chipset) => {
                        let chipset = &game.database.chipsets[chipset as usize - 1].file;
                        let file = code_page.0.to_encoding().decode(chipset).0.to_string();

                        // bevy 18 will add a setting to loading images but it does not help me because chipsets are not 1x480, they are 30x16
                        asset_server.load(game.game_dir.join("ChipSet/").join(file + ".png")) // TODO: it can be a .bmp too
                    }
                    None => fallback.0.clone(),
                };

                commands
                    .entity(entity)
                    .insert(super::chipset::Loading(texture));
            }
        };
    }
}
