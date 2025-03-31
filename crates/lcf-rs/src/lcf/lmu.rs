use byteorder::ReadBytesExt;
use nom::{Parser, bytes::complete::take, multi::many0};

#[derive(serde::Serialize, serde::Deserialize, derive_builder::Builder)]
pub struct LcfMapUnit {
    #[builder(default = 1)]
    pub chipset: u32,
    #[builder(default = 20)]
    pub width: u32,
    #[builder(default = 15)]
    pub height: u32,
    pub lower_map: Vec<u16>,
    pub upper_map: Vec<u16>,
}

impl From<LcfMapUnitBuilderError> for super::chunk::Error {
    fn from(value: LcfMapUnitBuilderError) -> Self {
        match value {
            LcfMapUnitBuilderError::UninitializedField(x) => Self::UninitializedField(x),
            LcfMapUnitBuilderError::ValidationError(x) => Self::ValidationError(x),
        }
    }
}

impl LcfMapUnit {
    pub(crate) fn from_chunks(chunks: Vec<(i128, &[u8])>) -> Result<Self, super::chunk::Error> {
        let mut builder = LcfMapUnitBuilder::create_empty();

        dbg!(&chunks.iter().map(|x| x.0).collect::<Vec<_>>());

        for (id, data) in chunks {
            match id {
                1 => drop(builder.chipset(super::chunk::read_number(data)? as u32)),
                2 => drop(builder.width(super::chunk::read_number(data)? as u32)),
                3 => drop(builder.height(super::chunk::read_number(data)? as u32)),
                71 => drop(builder.lower_map(parse_layer(data)?)),
                72 => drop(builder.upper_map(parse_layer(data)?)),
                _ => eprintln!("Unrecognized ID {id} in LMU"),
            }
        }

        builder.build().map_err(super::chunk::Error::from)
    }
}

fn parse_layer(data: &[u8]) -> Result<Vec<u16>, super::chunk::Error> {
    many0(
        take::<usize, &[u8], nom::error::Error<&[u8]>>(2usize)
            .map(|mut x: &[u8]| x.read_u16::<byteorder::LittleEndian>().unwrap()),
    )
    .parse(data)
    .map(|x| x.1)
    .map_err(|_: nom::Err<_>| super::chunk::Error::Parse)
}
