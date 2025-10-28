use bevy::{asset::AssetLoader, prelude::*};
use bevy_asset_loader::prelude::{LoadingStateAppExt as _, *};

use crate::state::GameLoadState;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<DataBaseAsset>()
            .init_asset::<MapTreeAsset>()
            .init_asset::<MapUnitAsset>()
            .init_asset_loader::<DataBaseLoader>()
            .init_asset_loader::<MapTreeLoader>()
            .init_asset_loader::<MapUnitLoader>()
            .add_loading_state(
                LoadingState::new(GameLoadState::Loading)
                    .continue_to_state(GameLoadState::Loaded)
                    .load_collection::<GameAssets>(),
            );
    }
}

#[derive(Resource, AssetCollection)]
pub struct GameAssets {
    #[asset(key = "database")]
    pub database: Handle<DataBaseAsset>,
    #[asset(key = "map_tree")]
    pub map_tree: Handle<MapTreeAsset>,
}

macro_rules! loader {
    ($loader:ident, $asset:ident, $type:ty, ($exts:expr)) => {
        #[derive(Default)]
        pub struct $loader;
        impl AssetLoader for $loader {
            type Asset = $asset;
            type Settings = ();
            type Error = LcfAssetLoaderError;

            async fn load(
                &self,
                reader: &mut dyn bevy::asset::io::Reader,
                _settings: &Self::Settings,
                _load_context: &mut bevy::asset::LoadContext<'_>,
            ) -> Result<Self::Asset, Self::Error> {
                let mut buf = Vec::new();
                reader.read_to_end(&mut buf).await?;
                match <$type>::read(&mut std::io::Cursor::new(buf)) {
                    Ok(x) => Ok($asset(x)),
                    Err(err) => Err(LcfAssetLoaderError::Parse(err.into())),
                }
            }

            fn extensions(&self) -> &[&str] {
                &[$exts]
            }
        }
    };
}

#[derive(Debug, thiserror::Error)]
pub enum LcfAssetLoaderError {
    #[error("IO error: {0:?}")]
    IO(#[from] std::io::Error),
    #[error("Read error: {0:?}")]
    Parse(#[from] lcf::LcfReadError),
}

#[derive(Asset, Debug, TypePath)]
pub struct DataBaseAsset(pub lcf::ldb::LcfDataBase);
loader!(
    DataBaseLoader,
    DataBaseAsset,
    lcf::ldb::LcfDataBase,
    ("ldb")
);

#[derive(Asset, Debug, TypePath)]
pub struct MapTreeAsset(pub lcf::lmt::LcfMapTree);
loader!(MapTreeLoader, MapTreeAsset, lcf::lmt::LcfMapTree, ("lmt"));

#[derive(Asset, Debug, TypePath)]
pub struct MapUnitAsset(pub lcf::lmu::LcfMapUnit);
loader!(MapUnitLoader, MapUnitAsset, lcf::lmu::LcfMapUnit, ("lmu"));
