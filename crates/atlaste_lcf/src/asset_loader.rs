use bevy::prelude::*;
use lcf::{ldb::LcfDataBase, lmt::LcfMapTree, lmu::LcfMapUnit};

macro_rules! loader {
    ($loader:ident, $asset:ident, $type:ty, ($exts:expr)) => {
        #[derive(Asset, Debug, TypePath)]
        pub struct $asset(pub $type);

        impl std::ops::Deref for $asset {
            type Target = $type;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        #[derive(Default)]
        pub struct $loader;
        impl bevy::asset::AssetLoader for $loader {
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

loader!(DataBaseLoader, DataBaseAsset, LcfDataBase, ("ldb"));
loader!(MapTreeLoader, MapTreeAsset, LcfMapTree, ("lmt"));
loader!(MapUnitLoader, MapUnitAsset, LcfMapUnit, ("lmu"));
