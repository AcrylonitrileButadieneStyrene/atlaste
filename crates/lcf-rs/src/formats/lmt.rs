use nom::{Parser, multi::length_count};

pub use super::structs::map::Map;

pub(crate) const HEADER: &[u8] = b"LcfMapTree";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LcfMapTree {
    pub maps: Vec<Map>,
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

        Ok((input, Self::from_chunks(&maps, &order, active, start)))
    }

    pub(crate) fn from_chunks(
        maps: &[(u64, Vec<crate::lcf::Chunk<'_>>)],
        order: &[u64],
        _active: u64,
        _start: Vec<crate::lcf::Chunk<'_>>,
    ) -> Result<Self, crate::Error> {
        if cfg!(debug_assertions) {
            assert_eq!(maps.len(), order.len());
            for (index, (id, _)) in maps.iter().enumerate() {
                assert_eq!(index as u64, *id);
            }
        }

        let mut maps = maps
            .iter()
            .map(|(index, data)| Map::from_chunks(*index as u16, data))
            .collect::<Result<Vec<_>, _>>()?;
        debug_assert_eq!(maps[0].id, 0);

        let mut parent_indices = maps
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, map)| (i, map.parent as usize))
            .collect::<Vec<_>>();
        parent_indices.sort_by_key(|(i, _)| order[*i]);
        for (i, parent) in parent_indices {
            maps[parent].children.push(i as u16);
        }

        Ok(Self { maps })
    }
}
