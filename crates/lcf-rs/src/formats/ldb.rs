use nom::{Parser, sequence::terminated};

pub(crate) const HEADER: &[u8] = b"LcfDataBase";

#[derive(serde::Serialize, serde::Deserialize, derive_builder::Builder)]
pub struct LcfDataBase {}

impl From<LcfDataBaseBuilderError> for crate::Error {
    fn from(value: LcfDataBaseBuilderError) -> Self {
        match value {
            LcfDataBaseBuilderError::UninitializedField(x) => Self::UninitializedField(x),
            LcfDataBaseBuilderError::ValidationError(x) => Self::ValidationError(x),
        }
    }
}

impl LcfDataBase {
    pub fn from_bytes(input: &[u8]) -> crate::lcf::ParseResult<Self> {
        let (input, header) = crate::lcf::read_header(input)?;
        if header != HEADER {
            return Ok((input, Err(crate::Error::InvalidHeader)));
        }

        Self::from_body(input)
    }

    pub(crate) fn from_body(input: &[u8]) -> crate::lcf::ParseResult<'_, Self> {
        let (input, chunks) =
            terminated(crate::lcf::parse_chunks, crate::lcf::check_empty).parse(input)?;
        Ok((input, Self::from_chunks(chunks)))
    }

    pub(crate) fn from_chunks(_chunks: Vec<crate::lcf::Chunk<'_>>) -> Result<Self, crate::Error> {
        let builder = LcfDataBaseBuilder::create_empty();
        builder.build().map_err(crate::Error::from)
    }
}
