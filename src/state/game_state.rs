use bevy::{asset::AssetPath, prelude::*};
use bevy_asset_loader::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, States)]
pub enum GameLoadState {
    #[default]
    NotLoaded,
    Loading,
    Loaded,
}

#[derive(Resource)]
pub struct GamePath(pub std::path::PathBuf);

impl GamePath {
    pub fn with(
        &'_ self,
        transform: impl FnOnce(&std::path::Path) -> std::path::PathBuf,
    ) -> String {
        transform(&self.0)
            .to_string_lossy()
            .to_string()
            .replace('\\', "/")
    }
}

pub fn load(
    game_path: Res<GamePath>,
    mut next_state: ResMut<NextState<crate::state::GameLoadState>>,
    mut dynamic: ResMut<DynamicAssets>,
) {
    dynamic.register_asset(
        "database",
        Box::new(Dynamic(game_path.with(|p| p.join("RPG_RT.ldb")).into())),
    );
    dynamic.register_asset(
        "map_tree",
        Box::new(Dynamic(game_path.with(|p| p.join("RPG_RT.lmt")).into())),
    );
    next_state.set(GameLoadState::Loading);
}

#[derive(Debug)]
struct Dynamic<'a>(pub AssetPath<'a>);

impl DynamicAsset for Dynamic<'_> {
    fn load(&self, asset_server: &AssetServer) -> Vec<UntypedHandle> {
        vec![asset_server.load_untyped(&self.0).untyped()]
    }

    fn build(&self, world: &mut World) -> Result<DynamicAssetType, anyhow::Error> {
        world
            .resource::<AssetServer>()
            .get_handle_untyped(&self.0)
            .map(DynamicAssetType::Single)
            .ok_or_else(|| panic!())
    }
}
