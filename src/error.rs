use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Provided .vox file does not contain any model")]
    EmptyVox,
    #[error("Provided .vox file contains multiple models, only one is allowed")]
    TooManyModels,
    #[error("failed to parse .vox file: {0}")]
    DotVox(&'static str),
    #[error("io error: {0}")]
    Io(#[from] io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
