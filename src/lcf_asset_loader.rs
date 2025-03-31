use bevy::{
    asset::{Asset, AssetLoader},
    reflect::TypePath,
};

#[derive(Asset, TypePath)]
pub struct LcfAsset(pub lcf_rs::Lcf);

pub struct LcfAssetLoader;

#[derive(Debug, thiserror::Error)]
pub enum LcfAssetLoaderError {
    #[error("IO error")]
    IO(#[from] std::io::Error),
    #[error("Parse error")]
    Parse(#[from] lcf_rs::nom::Err<lcf_rs::nom::error::Error<Vec<u8>>>),
    #[error("Build error")]
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
            Ok(Ok(lcf)) => Ok(LcfAsset(lcf)),
            Ok(Err(err)) => Err(LcfAssetLoaderError::Build(err)),
            Err(err) => Err(LcfAssetLoaderError::Parse(err.to_owned())),
        }
    }

    fn extensions(&self) -> &[&str] {
        &["ldb", "lmt", "lmu", "lsd"]
    }
}
