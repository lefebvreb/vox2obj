use std::io;

use image::ImageError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid scene graph")]
    InvalidSceneGraph,
    #[error("failed to parse .vox file: {0}")]
    DotVox(&'static str),
    #[error("image error: {0}")]
    Image(#[from] ImageError),
    #[error("io error: {0}")]
    Io(#[from] io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
