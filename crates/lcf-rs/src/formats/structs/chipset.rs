use nom::{IResult, Parser, multi::length_count, sequence::terminated};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, derive_builder::Builder)]
pub struct ChipSet {
    pub id: u16,
    #[builder(default)]
    pub name: Option<Vec<u8>>,
    #[builder(default)]
    pub file: Option<Vec<u8>>,
}

impl From<ChipSetBuilderError> for crate::Error {
    fn from(value: ChipSetBuilderError) -> Self {
        match value {
            ChipSetBuilderError::UninitializedField(x) => Self::UninitializedField(x),
            ChipSetBuilderError::ValidationError(x) => Self::ValidationError(x),
        }
    }
}

impl ChipSet {
    pub fn from_chunks_2d(input: &[u8]) -> IResult<&[u8], Result<Vec<Self>, crate::Error>> {
        terminated(
            length_count(
                crate::lcf::read_number,
                (crate::lcf::read_number, crate::lcf::parse_chunks)
                    .map(|(id, chunks)| Self::from_chunks(id as u16, &chunks)),
            )
            .map(|x| x.into_iter().collect()),
            crate::lcf::check_empty,
        )
        .parse(input)
    }

    pub fn from_chunks(id: u16, chunks: &[crate::lcf::Chunk]) -> Result<Self, crate::Error> {
        let mut builder = ChipSetBuilder::create_empty();
        builder.id(id);

        for (id, data) in chunks {
            match id {
                1 => drop(builder.name(Some(Vec::from(*data)))),
                2 => drop(builder.file(Some(Vec::from(*data)))),
                _ => (),
            }
        }

        builder.build().map_err(crate::Error::from)
    }
}
