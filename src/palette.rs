use std::fs::{self, File};
use std::path::{Path, PathBuf};

use image::{ImageFormat, RgbaImage};

use crate::error::Result;

fn write_png_to(image: &RgbaImage, path: PathBuf) -> Result<()> {
    let mut file = File::create(path)?;
    image.write_to(&mut file, ImageFormat::Png)?;
    Ok(())
}

#[derive(Debug)]
pub struct Palette {
    pub albedo: RgbaImage,
}

impl Palette {
    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        fs::create_dir_all(path)?;
        write_png_to(&self.albedo, path.join("albedo.png"))?;
        Ok(())
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self { 
            albedo: RgbaImage::new(256, 1),
        }
    }
}
