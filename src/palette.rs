use std::fs::{self, File};
use std::path::{Path, PathBuf};

use dot_vox::{Color, Material};
use image::{ImageFormat, Rgba, RgbaImage};

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
    pub fn new(colors: &[Color], materials: &[Material]) -> Self {
        let mut palette = Self {
            albedo: RgbaImage::new(256, 1),
        };

        for (i, color) in colors.iter().enumerate() {
            palette.albedo.put_pixel(i as u32, 0, Rgba(color.into()));
        }

        palette
    }

    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        fs::create_dir_all(path)?;
        write_png_to(&self.albedo, path.join("albedo.png"))?;
        Ok(())
    }
}
