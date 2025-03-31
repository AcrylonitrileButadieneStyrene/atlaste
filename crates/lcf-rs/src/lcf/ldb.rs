#[derive(serde::Serialize, serde::Deserialize, derive_builder::Builder)]
pub struct LcfDataBase {}

impl From<LcfDataBaseBuilderError> for super::chunk::Error {
    fn from(value: LcfDataBaseBuilderError) -> Self {
        match value {
            LcfDataBaseBuilderError::UninitializedField(x) => Self::UninitializedField(x),
            LcfDataBaseBuilderError::ValidationError(x) => Self::ValidationError(x),
        }
    }
}

impl LcfDataBase {
    pub(crate) fn from_chunks(_chunks: Vec<(i128, &[u8])>) -> Result<Self, super::chunk::Error> {
        todo!();
    }
}
