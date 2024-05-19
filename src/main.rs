mod convert;
mod error;
mod obj;
mod palette;

use std::fs;
use std::path::PathBuf;

use clap::Parser;
use palette::Palette;

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
    #[arg(
        short = 'p',
        long = "write-palette-to",
        help = "Write palette to the given directory"
    )]
    palette: Option<PathBuf>,
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

    obj.write(args.output)?;

    if let Some(path) = args.palette {
        let palette = Palette::new(&vox.palette, &vox.materials);
        palette.write(path)?;
    }

    Ok(())
}
