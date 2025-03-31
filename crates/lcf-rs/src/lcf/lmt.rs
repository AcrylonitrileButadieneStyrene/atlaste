#[derive(serde::Serialize, serde::Deserialize, derive_builder::Builder)]
pub struct LcfMapTree {}

impl From<LcfMapTreeBuilderError> for super::chunk::Error {
    fn from(value: LcfMapTreeBuilderError) -> Self {
        match value {
            LcfMapTreeBuilderError::UninitializedField(x) => Self::UninitializedField(x),
            LcfMapTreeBuilderError::ValidationError(x) => Self::ValidationError(x),
        }
    }
}

impl LcfMapTree {
    pub(crate) fn from_chunks(_chunks: Vec<(i128, &[u8])>) -> Result<Self, super::chunk::Error> {
        todo!();
    }
}
