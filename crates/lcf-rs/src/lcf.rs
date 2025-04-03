use crate::{
    Error,
    ldb::{self, LcfDataBase},
    lmt::{self, LcfMapTree},
    lmu::{self, LcfMapUnit},
    lsd::{self, LcfSaveData},
};

use nom::{
    Parser,
    bytes::complete::{tag, take, take_while_m_n},
    combinator::{eof, opt, recognize, verify},
    multi::{length_data, many0},
    sequence::terminated,
};

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Lcf {
    DataBase(LcfDataBase),
    MapTree(LcfMapTree),
    MapUnit(LcfMapUnit),
    SaveData(LcfSaveData),
}

impl Lcf {
    pub fn from_bytes(
        input: &[u8],
    ) -> Result<Result<Self, Error>, nom::Err<nom::error::Error<&[u8]>>> {
        Self::parse(input).map(|x| x.1)
    }

    fn parse(input: &[u8]) -> ParseResult<Self> {
        let (input, header) = read_header(input)?;

        match header {
            ldb::HEADER => LcfDataBase::from_body(input).map(|x| (x.0, x.1.map(Lcf::DataBase))),
            lmt::HEADER => LcfMapTree::from_body(input).map(|x| (x.0, x.1.map(Lcf::MapTree))),
            lmu::HEADER => LcfMapUnit::from_body(input).map(|x| (x.0, x.1.map(Lcf::MapUnit))),
            lsd::HEADER => LcfSaveData::from_body(input).map(|x| (x.0, x.1.map(Lcf::SaveData))),
            _ => Ok((input, Err(Error::InvalidHeader))),
        }
    }
}

pub type ParseResult<'a, T> = nom::IResult<&'a [u8], Result<T, Error>>;
pub type Chunk<'a> = (u64, &'a [u8]);

pub fn read_header(input: &[u8]) -> nom::IResult<&[u8], &[u8]> {
    length_data(read_number).parse(input)
}

pub fn parse_chunks(input: &[u8]) -> nom::IResult<&[u8], Vec<crate::lcf::Chunk<'_>>> {
    terminated(
        many0((verify(read_number, |id| *id != 0), length_data(read_number))),
        opt(tag(&[0u8] as &[u8])),
    )
    .parse(input)
}

pub fn check_empty(input: &[u8]) -> nom::IResult<&[u8], ()> {
    let (input, _) = (opt(tag(&[0u8] as &[u8])), eof).parse(input)?;
    Ok((input, ()))
}

pub fn read_number(input: &[u8]) -> nom::IResult<&[u8], u64> {
    let (input, bytes) = recognize((
        take_while_m_n(0, 4, |x| x >= 0x80),
        verify(take(1usize), |x: &[u8]| x[0] < 0x80),
    ))
    .parse(input)?;

    let mut value = 0u64;
    for byte in bytes {
        value <<= 7;
        value |= u64::from(byte & 0x7F);
    }

    Ok((input, value))
}

#[test]
fn test_parse_chunks() {
    let input = &[0x01u8, 0x02u8, 0x00u8, 0x00u8, 0xffu8] as &[u8];

    let (input, vec) = parse_chunks(input).unwrap();

    assert_eq!(input, &[0xffu8]);
    assert_eq!(vec, vec![(1, &[0x00u8, 0x00u8] as &[u8])]);
}

#[test]
fn test_parse_signed() {
    let input = &[0x93u8, 0x45u8] as &[u8];
    let (input, num) = read_number(input).unwrap();

    assert_eq!(input, &[]);
    assert_eq!(num, 2501);
}
