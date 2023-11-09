use std::{io};
use thiserror::Error as ThisError;
use zip::result::ZipError;


#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    ZipEntryError(#[from] ZipError),

    #[error(transparent)]
    IO(#[from] io::Error),

    #[error("cannot find class: {0}")]
    ClassNotFound(String),

    #[error("unknown constant type: {0}")]
    UnKnownConstantType(u8),
}
