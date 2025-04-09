use bevy::{asset::AssetLoader, prelude::*};
use bevy_asset_loader::prelude::*;

use crate::state::GameState;

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
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::Loaded)
                    .load_collection::<GameAssets>(),
            );
    }
}

#[derive(AssetCollection, Resource)]
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
                match <$type>::from_bytes(&buf) {
                    Ok((_, Ok(x))) => Ok($asset(x)),
                    Ok((_, Err(err))) => Err(LcfAssetLoaderError::Build(err)),
                    Err(err) => Err(LcfAssetLoaderError::Parse(err.to_owned())),
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
    #[error("Parse error: {0:?}")]
    Parse(#[from] lcf_rs::nom::Err<lcf_rs::nom::error::Error<Vec<u8>>>),
    #[error("Build error: {0:?}")]
    Build(#[from] lcf_rs::Error),
}

#[derive(Asset, Debug, TypePath)]
pub struct DataBaseAsset(pub lcf_rs::LcfDataBase);
loader!(DataBaseLoader, DataBaseAsset, lcf_rs::LcfDataBase, ("ldb"));

#[derive(Asset, Debug, TypePath)]
pub struct MapTreeAsset(pub lcf_rs::LcfMapTree);
loader!(MapTreeLoader, MapTreeAsset, lcf_rs::LcfMapTree, ("lmt"));

#[derive(Asset, Debug, TypePath)]
pub struct MapUnitAsset(pub lcf_rs::LcfMapUnit);
loader!(MapUnitLoader, MapUnitAsset, lcf_rs::LcfMapUnit, ("lmu"));
