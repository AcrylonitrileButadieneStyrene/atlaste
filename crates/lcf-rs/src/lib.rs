#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::cast_possible_truncation)]

mod error;
mod formats;
mod lcf;

pub use error::Error;
pub use formats::{
    ldb::{self, LcfDataBase},
    lmt::{self, LcfMapTree},
    lmu::{self, LcfMapUnit},
    lsd::{self, LcfSaveData},
};
pub use lcf::Lcf;

pub use nom;
