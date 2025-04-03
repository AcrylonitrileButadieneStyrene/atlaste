use bevy::{asset::AssetLoader, prelude::*};

#[derive(Resource)]
pub struct GameData {
    pub database: Handle<crate::lcf_asset_loader::LcfAsset>,
    pub map_tree: Handle<crate::lcf_asset_loader::LcfAsset>,
}

#[derive(Asset, TypePath)]
pub enum LcfAsset {
    DataBase(lcf_rs::LcfDataBase),
    MapTree(lcf_rs::LcfMapTree),
    SaveData(lcf_rs::LcfSaveData),
    MapUnit(lcf_rs::LcfMapUnit),
}

#[derive(Default)]
pub struct LcfAssetLoader;

#[derive(Debug, thiserror::Error)]
pub enum LcfAssetLoaderError {
    #[error("IO error: {0:?}")]
    IO(#[from] std::io::Error),
    #[error("Parse error: {0:?}")]
    Parse(#[from] lcf_rs::nom::Err<lcf_rs::nom::error::Error<Vec<u8>>>),
    #[error("Build error: {0:?}")]
    Build(#[from] lcf_rs::Error),
}

impl AssetLoader for LcfAssetLoader {
    type Asset = LcfAsset;
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
        match lcf_rs::Lcf::from_bytes(&buf) {
            Ok(Ok(lcf_rs::Lcf::DataBase(x))) => Ok(LcfAsset::DataBase(x)),
            Ok(Ok(lcf_rs::Lcf::MapTree(x))) => Ok(LcfAsset::MapTree(x)),
            Ok(Ok(lcf_rs::Lcf::MapUnit(x))) => Ok(LcfAsset::MapUnit(x)),
            Ok(Ok(lcf_rs::Lcf::SaveData(x))) => Ok(LcfAsset::SaveData(x)),
            Ok(Err(err)) => Err(LcfAssetLoaderError::Build(err)),
            Err(err) => Err(LcfAssetLoaderError::Parse(err.to_owned())),
        }
    }

    fn extensions(&self) -> &[&str] {
        &["ldb", "lmt", "lmu", "lsd"]
    }
}

pub fn load_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_dir: Res<crate::app::GameDir>,
) {
    commands.insert_resource(GameData {
        database: asset_server.load(game_dir.0.join("RPG_RT.ldb")),
        map_tree: asset_server.load(game_dir.0.join("RPG_RT.lmt")),
    });
}
