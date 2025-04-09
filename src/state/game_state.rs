use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, States)]
pub enum GameState {
    #[default]
    None,
    Loading,
    Loaded,
}

#[derive(Resource)]
pub struct GamePath(pub std::path::PathBuf);

pub fn load(
    game_path: Res<GamePath>,
    mut next_state: ResMut<NextState<crate::state::GameState>>,
    mut dynamic: ResMut<DynamicAssets>,
) {
    dynamic.register_asset(
        "database",
        Box::new(DynamicAssetPathbuf(game_path.0.join("RPG_RT.ldb"))),
    );
    dynamic.register_asset(
        "map_tree",
        Box::new(DynamicAssetPathbuf(game_path.0.join("RPG_RT.lmt"))),
    );
    next_state.set(GameState::Loading);
}

#[derive(Debug)]
struct DynamicAssetPathbuf(std::path::PathBuf);

impl DynamicAsset for DynamicAssetPathbuf {
    fn load(&self, asset_server: &AssetServer) -> Vec<UntypedHandle> {
        vec![asset_server.load_untyped(self.0.clone()).untyped()]
    }

    fn build(&self, world: &mut World) -> Result<DynamicAssetType, anyhow::Error> {
        let Some(asset_server) = world.get_resource::<AssetServer>() else {
            anyhow::bail!("AssetServer not found");
        };

        Ok(DynamicAssetType::Single(
            asset_server.get_handle_untyped(self.0.clone()).unwrap(),
        ))
    }
}
