use std::sync::Arc;

use bevy::{
    asset::{AssetLoader, ReadAssetBytesError},
    prelude::*,
};
use png::DecodingError;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<IndexedImage>()
            .register_asset_loader(IndexedImageAssetLoader);
    }
}

#[derive(Asset, TypePath)]
pub struct IndexedImage {
    pub image: Handle<Image>,
    pub palette: Option<Arc<[u8]>>,
}

#[derive(Debug, thiserror::Error)]
pub enum IndexedImageLoadError {
    #[error("io error {0}")]
    IO(#[from] std::io::Error),
    #[error("read error {0}")]
    Read(#[from] ReadAssetBytesError),
    #[error("decode error {0}")]
    Decode(#[from] DecodingError),
}

struct IndexedImageAssetLoader;
impl AssetLoader for IndexedImageAssetLoader {
    type Asset = IndexedImage;
    type Settings = ();
    type Error = IndexedImageLoadError;

    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let path = load_context.path().to_owned();
        let image = load_context.load(path.clone());
        let ext = path.extension().and_then(|ext| ext.to_str());

        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).await?;

        let palette = match ext {
            Some("png") => {
                let decoder = png::Decoder::new(std::io::Cursor::new(buf)).read_info()?;
                decoder
                    .info()
                    .palette
                    .as_ref()
                    .map(|data| data.to_vec().into_boxed_slice().into())
            }
            Some("bmp") => todo!(),
            Some(x) => panic!("unknown image type {x}"),
            None => panic!("image has no extension"),
        };

        Ok(Self::Asset { image, palette })
    }

    fn extensions(&self) -> &[&str] {
        &["bmp", "png"]
    }
}
