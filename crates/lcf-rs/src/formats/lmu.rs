use nom::{
    Parser, bytes::complete::take, multi::many0, number::complete::le_u16, sequence::terminated,
};

pub(crate) const HEADER: &[u8] = b"LcfMapUnit";

#[derive(Debug, serde::Serialize, serde::Deserialize, derive_builder::Builder)]
pub struct LcfMapUnit {
    #[builder(default = 1)]
    pub chipset: u16,
    #[builder(default = 20)]
    pub width: u32,
    #[builder(default = 15)]
    pub height: u32,
    pub lower: Vec<u16>,
    pub upper: Vec<u16>,
}

impl From<LcfMapUnitBuilderError> for crate::Error {
    fn from(value: LcfMapUnitBuilderError) -> Self {
        match value {
            LcfMapUnitBuilderError::UninitializedField(x) => Self::UninitializedField(x),
            LcfMapUnitBuilderError::ValidationError(x) => Self::ValidationError(x),
        }
    }
}

impl LcfMapUnit {
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
        let mut builder = LcfMapUnitBuilder::create_empty();

        for (id, data) in chunks {
            match id {
                1 => drop(builder.chipset(read_number_handled(data)? as u16)),
                2 => drop(builder.width(read_number_handled(data)? as u32)),
                3 => drop(builder.height(read_number_handled(data)? as u32)),
                71 => drop(builder.lower(parse_layer(data)?)),
                72 => drop(builder.upper(parse_layer(data)?)),
                _ => log::info!("Unrecognized ID {id} in LMU"),
            }
        }

        builder.build().map_err(crate::Error::from)
    }

    pub fn convert_layer_to_chipset_index(id: usize) -> usize {
        match id {
            // ground layer unanimated
            5000..=5143 => {
                let index = id - 5000;
                let col = index % 6;
                let base = 12 + (index / 96) * 6;
                (index - col) * 5 + col + base
            }
            _ => 0, // todo
        }
    }
}

fn parse_layer(data: &[u8]) -> Result<Vec<u16>, crate::Error> {
    many0(take::<usize, &[u8], nom::error::Error<&[u8]>>(2usize).and_then(le_u16))
        .parse(data)
        .map(|x| x.1)
        .map_err(|_: nom::Err<_>| crate::Error::Parse)
}

pub fn read_number_handled(data: &[u8]) -> Result<u64, crate::Error> {
    Ok(crate::lcf::read_number(data)
        .map_err(|_| crate::Error::Parse)?
        .1)
}
