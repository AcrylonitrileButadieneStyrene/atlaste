use std::sync::Arc;

use atlaste_lcf::{MapUnitAsset, lcf::lmu::LcfMapUnit};
use bevy::{prelude::*, ui_widgets::observe};

use crate::state::GameData;

#[derive(Component)]
pub struct MapUnit {
    #[expect(unused)] // keep the asset alive incase the map is loaded twice
    handle: Handle<MapUnitAsset>,
    pub map: Arc<LcfMapUnit>,
    #[allow(unused)]
    pub hash: u32, // todo: will be used for version control
}

pub fn on_map_tree_entry_clicked(
    trigger: On<atlaste_ui::sections::map_tree::EntryClicked>,
    mut commands: Commands,
    game: Res<GameData>,
    asset_server: Res<AssetServer>,
) {
    let path = game
        .game_dir
        .resolve(&format!("Map{:0>4}.lmu", trigger.0))
        .unwrap();
    let map = asset_server.load::<MapUnitAsset>(path);

    commands.spawn((
        Transform::default(),
        Visibility::default(),
        atlaste_asset::ObservedAsset {
            handle: map.untyped(),
            despawn: false,
        },
        observe(
            |loaded: On<atlaste_asset::ObservedAssetLoaded>,
             map_units: Res<Assets<MapUnitAsset>>,
             mut commands: Commands| {
                match &loaded.status {
                    Ok(()) => {
                        let handle = loaded.handle.clone().typed();
                        let map = map_units.get(&handle).unwrap();
                        commands.entity(loaded.entity).insert(MapUnit {
                            handle,
                            map: map.data.clone(),
                            hash: map.hash,
                        });
                    }
                    Err(None) => {
                        error!("Map did not start loading");
                    }
                    Err(Some(err)) => {
                        error!("Failed to load map: {err}");
                    }
                }
            },
        ),
    ));
}
