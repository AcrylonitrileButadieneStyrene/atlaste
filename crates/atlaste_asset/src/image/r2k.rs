use bevy::{
    asset::{AssetLoader, ReadAssetBytesError},
    prelude::*,
};
use png::DecodingError;

#[derive(Asset, TypePath)]
pub struct R2kImage {
    pub image: Handle<Image>,
    pub alpha_key: Option<u32>,
}

#[derive(Debug, thiserror::Error)]
pub enum R2kImageLoadError {
    #[error("io error {0}")]
    IO(#[from] std::io::Error),
    #[error("read error {0}")]
    Read(#[from] ReadAssetBytesError),
    #[error("decode error {0}")]
    Decode(#[from] DecodingError),
}

pub struct R2kImageAssetLoader;
impl AssetLoader for R2kImageAssetLoader {
    type Asset = R2kImage;
    type Settings = ();
    type Error = R2kImageLoadError;

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

        let alpha_key = match ext {
            Some("png") => png::Decoder::new(std::io::Cursor::new(buf))
                .read_info()?
                .info()
                .palette
                .as_ref()
                .map(|palette| {
                    u32::from(palette[0]) << 16 | u32::from(palette[1]) << 8 | u32::from(palette[2])
                }),
            Some("bmp") => todo!(),
            Some(x) => panic!("unknown image type {x}"),
            None => panic!("image has no extension"),
        };

        Ok(Self::Asset { image, alpha_key })
    }

    fn extensions(&self) -> &[&str] {
        &["bmp", "png"]
    }
}
