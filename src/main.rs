mod convert;
mod error;
mod obj;

use std::fs;
use std::path::PathBuf;

use clap::Parser;

use crate::error::{Error, Result};

#[derive(Parser, Debug)]
#[command(version, author, about)]
pub struct Args {
    #[arg(help = "Path to the input .vox file")]
    input: PathBuf,
    #[arg(
        short,
        long,
        default_value = "out.obj",
        help = "Path to the output .obj file name"
    )]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let input = fs::read(&args.input)?;
    let vox = dot_vox::load_bytes(input.as_ref()).map_err(Error::DotVox)?;

    let obj = match vox.models.as_slice() {
        [] => return Err(Error::EmptyVox),
        [ref model] => convert::convert_model(model),
        _ => return Err(Error::TooManyModels),
    };

    fs::write(args.output, obj.to_string())?;

    Ok(())
}
