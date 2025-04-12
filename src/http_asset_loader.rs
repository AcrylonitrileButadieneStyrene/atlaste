use bevy::asset::io::{AssetReader, AssetReaderError, PathStream, Reader, VecReader};

pub struct HttpAssetLoader;

impl AssetReader for HttpAssetLoader {
    async fn read<'a>(
        &'a self,
        path: &'a std::path::Path,
    ) -> Result<impl Reader + 'a, AssetReaderError> {
        Ok(VecReader::new(
            ehttp::fetch_async(ehttp::Request::get(
                "https://".to_string() + &path.to_string_lossy(),
            ))
            .await
            .map_err(|err| AssetReaderError::NotFound(err.into()))?
            .bytes,
        ))
    }

    async fn read_meta<'a>(
        &'a self,
        _path: &'a std::path::Path,
    ) -> Result<impl Reader + 'a, AssetReaderError> {
        Err::<VecReader, _>(AssetReaderError::NotFound(std::path::PathBuf::new())) // TODO: use a lazy_static instead of allocating each time
    }

    async fn read_directory<'a>(
        &'a self,
        _path: &'a std::path::Path,
    ) -> Result<Box<PathStream>, AssetReaderError> {
        Err(AssetReaderError::HttpError(403))
    }

    async fn is_directory<'a>(
        &'a self,
        _path: &'a std::path::Path,
    ) -> Result<bool, AssetReaderError> {
        Err(AssetReaderError::HttpError(400))
    }
}
