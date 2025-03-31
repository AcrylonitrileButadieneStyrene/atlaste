#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Parse error")]
    Parse,
    #[error("Unitialized field")]
    UninitializedField(&'static str),
    #[error("Validation error")]
    ValidationError(String),
}

pub fn read_number(data: &[u8]) -> Result<i128, Error> {
    Ok(super::utils::parse_signed(data)
        .map_err(|_| Error::Parse)?
        .1 as i128)
}
