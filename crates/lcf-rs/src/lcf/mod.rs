mod chunk;
pub mod ldb;
pub mod lmt;
pub mod lmu;
pub mod lsd;
mod utils;

pub use chunk::Error;
use nom::{Parser, bytes::complete::tag, combinator::eof, multi::length_data};
use utils::parse_signed;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Lcf {
    DataBase(ldb::LcfDataBase),
    MapTree(lmt::LcfMapTree),
    MapUnit(lmu::LcfMapUnit),
    SaveData(lsd::LcfSaveData),
}

impl Lcf {
    pub fn from_bytes(
        input: &[u8],
    ) -> Result<Result<Self, chunk::Error>, nom::Err<nom::error::Error<&[u8]>>> {
        parse(input).map(|x| x.1)
    }
}

fn parse(input: &[u8]) -> nom::IResult<&[u8], Result<Lcf, chunk::Error>> {
    let (input, header) = length_data(parse_signed.map(|x| x as usize)).parse(input)?;
    let (input, chunks) = utils::parse_chunks(input)?;
    let (empty, _) = (tag(&[0u8] as &[u8]), eof).parse(input)?;

    Ok((empty, match header {
        b"LcfDataBase" => ldb::LcfDataBase::from_chunks(chunks).map(Lcf::DataBase),
        b"LcfMapTree" => lmt::LcfMapTree::from_chunks(chunks).map(Lcf::MapTree),
        b"LcfMapUnit" => lmu::LcfMapUnit::from_chunks(chunks).map(Lcf::MapUnit),
        b"LcfSaveData" => lsd::LcfSaveData::from_chunks(chunks).map(Lcf::SaveData),
        _ => {
            return Err(nom::Err::Error(nom::error::Error::<&[u8]>::new(
                input,
                nom::error::ErrorKind::Alt,
            )));
        }
    }))
}
