use nom::{Parser, multi::length_count};

pub(crate) const HEADER: &[u8] = b"LcfMapTree";

#[derive(serde::Serialize, serde::Deserialize, derive_builder::Builder)]
pub struct LcfMapTree {}

impl From<LcfMapTreeBuilderError> for crate::Error {
    fn from(value: LcfMapTreeBuilderError) -> Self {
        match value {
            LcfMapTreeBuilderError::UninitializedField(x) => Self::UninitializedField(x),
            LcfMapTreeBuilderError::ValidationError(x) => Self::ValidationError(x),
        }
    }
}

impl LcfMapTree {
    pub fn from_bytes(input: &[u8]) -> crate::lcf::ParseResult<Self> {
        let (input, header) = crate::lcf::read_header(input)?;
        if header != HEADER {
            return Ok((input, Err(crate::Error::InvalidHeader)));
        }

        Self::from_body(input)
    }

    pub(crate) fn from_body(input: &[u8]) -> crate::lcf::ParseResult<'_, Self> {
        let (input, (maps, order, active, start, ())) = (
            length_count(
                crate::lcf::read_number,
                (crate::lcf::read_number, crate::lcf::parse_chunks),
            ),
            length_count(crate::lcf::read_number, crate::lcf::read_number),
            crate::lcf::read_number,
            crate::lcf::parse_chunks,
            crate::lcf::check_empty,
        )
            .parse(input)?;

        Ok((input, Self::from_chunks(maps, order, active, start)))
    }

    pub(crate) fn from_chunks(
        _maps: Vec<(u64, Vec<crate::lcf::Chunk<'_>>)>,
        _order: Vec<u64>,
        _active: u64,
        _start: Vec<crate::lcf::Chunk<'_>>,
    ) -> Result<Self, crate::Error> {
        let builder = LcfMapTreeBuilder::create_empty();
        builder.build().map_err(crate::Error::from)
    }
}
