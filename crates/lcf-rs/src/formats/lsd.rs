use nom::{Parser, sequence::terminated};

pub(crate) const HEADER: &[u8] = b"LcfSaveData";

#[derive(Debug, serde::Serialize, serde::Deserialize, derive_builder::Builder)]
pub struct LcfSaveData {}

impl From<LcfSaveDataBuilderError> for crate::Error {
    fn from(value: LcfSaveDataBuilderError) -> Self {
        match value {
            LcfSaveDataBuilderError::UninitializedField(x) => Self::UninitializedField(x),
            LcfSaveDataBuilderError::ValidationError(x) => Self::ValidationError(x),
        }
    }
}

impl LcfSaveData {
    pub fn from_bytes(input: &[u8]) -> crate::lcf::ParseResult<'_, Self> {
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

    pub(crate) fn from_chunks(chunks: Vec<crate::lcf::Chunk<'_>>) -> Result<Self, crate::Error> {
        let builder = LcfSaveDataBuilder::create_empty();

        for (id, _data) in chunks {
            #[allow(clippy::match_single_binding)]
            match id {
                _ => log::info!("Unrecognized ID {id} in LSD"),
            }
        }

        builder.build().map_err(crate::Error::from)
    }
}
