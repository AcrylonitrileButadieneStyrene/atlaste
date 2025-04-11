use crate::lmu::read_number_handled;

#[derive(Debug, serde::Serialize, serde::Deserialize, derive_builder::Builder)]
pub struct Map {
    pub name: Vec<u8>,
    pub id: u16,
    #[builder(default)]
    pub parent: u16,

    /// Note: the children are listed in the game-specified order.
    #[builder(default)]
    pub children: Vec<u16>,
}

impl From<MapBuilderError> for crate::Error {
    fn from(value: MapBuilderError) -> Self {
        match value {
            MapBuilderError::UninitializedField(x) => Self::UninitializedField(x),
            MapBuilderError::ValidationError(x) => Self::ValidationError(x),
        }
    }
}

impl Map {
    pub(crate) fn from_chunks(id: u16, chunks: &[crate::lcf::Chunk]) -> Result<Self, crate::Error> {
        let mut builder = MapBuilder::create_empty();
        builder.id(id);

        for (id, data) in chunks {
            match id {
                1 => drop(builder.name(Vec::from(*data))),
                2 => drop(builder.parent(read_number_handled(data)? as u16)),
                _ => (),
                // _ => eprintln!("Unrecognized ID {id} in LMU"),
            }
        }

        builder.build().map_err(crate::Error::from)
    }
}
