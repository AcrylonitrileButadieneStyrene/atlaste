#[derive(serde::Serialize, serde::Deserialize, derive_builder::Builder)]
pub struct LcfSaveData {}

impl From<LcfSaveDataBuilderError> for super::chunk::Error {
    fn from(value: LcfSaveDataBuilderError) -> Self {
        match value {
            LcfSaveDataBuilderError::UninitializedField(x) => Self::UninitializedField(x),
            LcfSaveDataBuilderError::ValidationError(x) => Self::ValidationError(x),
        }
    }
}

impl LcfSaveData {
    pub(crate) fn from_chunks(_chunks: Vec<(i128, &[u8])>) -> Result<Self, super::chunk::Error> {
        todo!();
    }
}
