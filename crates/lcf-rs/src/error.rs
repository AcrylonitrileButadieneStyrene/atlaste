#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid header")]
    InvalidHeader,
    #[error("Parse error")]
    Parse,
    #[error("Unitialized field")]
    UninitializedField(&'static str),
    #[error("Validation error")]
    ValidationError(String),
}
