use atlaste_lcf::MapUnitAsset;
use bevy::{asset::LoadState, prelude::*};

#[derive(Component)]
pub struct Loading(pub Handle<MapUnitAsset>);

pub fn check(
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &Loading)>,
    map_units: Res<Assets<MapUnitAsset>>,
    mut commands: Commands,
) {
    for (entity, loading) in query.iter() {
        match asset_server.get_load_state(&loading.0) {
            Some(LoadState::Loading) => (),
            Some(LoadState::NotLoaded) | None => {
                error!("Map did not start loading");
            }
            Some(LoadState::Failed(err)) => {
                commands.entity(entity).remove::<Loading>();
                error!("Failed to load map: {err}");
            }
            Some(LoadState::Loaded) => {
                info!("Loaded");
                let map = map_units.get(&loading.0).unwrap();
                commands
                    .entity(entity)
                    .remove::<Loading>()
                    .insert(super::MapUnit {
                        handle: loading.0.clone(),
                        map: map.data.clone(),
                        hash: map.hash,
                    });
            }
        }
    }
}
