mod error;
mod model;
mod palette;

use std::fs;
use std::path::{Path, PathBuf};

use clap::Parser;
use dot_vox::{DotVoxData, SceneNode};
use palette::Palette;

use crate::error::{Error, Result};
use crate::model::Obj;

#[derive(Parser, Debug)]
#[command(version, author, about)]
pub struct Args {
    #[arg(help = "Path to the input .vox file")]
    input: PathBuf,
    #[arg(
        short,
        long,
        default_value = "out/",
        help = "Path to the output directory"
    )]
    output: PathBuf,
    #[arg(
        short = 'p',
        long = "palette",
        help = "Write palette textures to the given directory"
    )]
    palette: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Parse .vox file.
    let vox = {
        let input = fs::read(&args.input)?;
        dot_vox::load_bytes(input.as_ref()).map_err(Error::DotVox)?
    };

    // Create output directory.
    fs::create_dir_all(&args.output)?;

    // Get root group node.
    let SceneNode::Group { children, .. } = &vox.scenes[1] else {
        return Err(Error::InvalidSceneGraph);
    };

    // Convert children.
    for &child in children {
        convert_transform(&vox, &args.output, child)?;
    }

    // Save palette if requested.
    if let Some(path) = args.palette {
        let palette = Palette::new(&vox.palette, &vox.materials);
        palette.write(path)?;
    }

    Ok(())
}

// Scene Graph
//
// T : Transform Node
// G : Group Node
// S : Shape Node
//
//      T
//      |
//      G
//     / \
//    T   T
//    |   |
//    G   S
//   / \
//  T   T
//  |   |
//  S   S
fn convert_transform(vox: &DotVoxData, path: &Path, scene: u32) -> Result<()> {
    let SceneNode::Transform {
        attributes, child, ..
    } = &vox.scenes[scene as usize]
    else {
        return Err(Error::InvalidSceneGraph);
    };

    let name = attributes.get("_name").cloned();

    match &vox.scenes[*child as usize] {
        // Group.
        SceneNode::Group { children, .. } => {
            let name = name.unwrap_or_else(|| format!("group_{child}"));
            let path = path.join(name);
            fs::create_dir_all(&path)?;

            for &child in children {
                convert_transform(vox, &path, child)?;
            }
        }
        // Shape.
        SceneNode::Shape { models, .. } => {
            let name = name.map_or_else(
                || format!("model_{child}.obj"),
                |name| format!("{name}.obj"),
            );
            let path = path.join(name);

            match models.as_slice() {
                // Simple model.
                [model] => {
                    let obj = Obj::new(&vox.models[model.model_id as usize]);
                    obj.write(&path)?;
                }
                // Animations.
                frames => {
                    fs::create_dir_all(&path)?;

                    for frame in frames {
                        let name = format!(
                            "frame_{}.obj",
                            frame.frame_index().ok_or(Error::InvalidSceneGraph)?
                        );
                        let obj = Obj::new(&vox.models[frame.model_id as usize]);
                        obj.write(path.join(name))?;
                    }
                }
            }
        }
        // Transforms should not be followed by another Transform.
        _ => return Err(Error::InvalidSceneGraph),
    }

    Ok(())
}
