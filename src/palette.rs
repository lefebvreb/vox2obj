use std::fs::{self, File};
use std::path::{Path, PathBuf};

use bitflags::bitflags;
use dot_vox::{Color, Material};
use image::{ImageBuffer, ImageFormat, Pixel, PixelWithColorType, Rgb, RgbImage, Rgba, RgbaImage};

use crate::error::Result;

fn write_png_to<P>(image: &ImageBuffer<P, Vec<u8>>, path: PathBuf) -> Result<()>
where
    P: Pixel<Subpixel = u8> + PixelWithColorType,
{
    let mut file = File::create(path)?;
    image.write_to(&mut file, ImageFormat::Png)?;
    Ok(())
}

fn put_pixel(image: &mut RgbImage, i: u32, value: f32) {
    image.put_pixel(i - 1, 0, Rgb([(value * 255.0) as u8; 3]));
}

bitflags! {
    #[derive(Debug)]
    pub struct Property: u32 {
        const METALLIC = 1;
        const ROUGHNESS = 2;
        const EMISSION = 4;
    }
}

#[derive(Debug)]
pub struct Palette {
    pub albedo: RgbaImage,
    pub properties: Property,
    pub metallic: RgbImage,
    pub roughness: RgbImage,
    pub emission: RgbImage,
}

impl Palette {
    pub fn new(colors: &[Color], materials: &[Material]) -> Self {
        let mut this = Self {
            albedo: RgbaImage::new(256, 1),
            properties: Property::empty(),
            metallic: RgbImage::new(256, 1),
            roughness: RgbImage::new(256, 1),
            emission: RgbImage::new(256, 1),
        };

        for (i, color) in colors.iter().enumerate() {
            this.albedo.put_pixel(i as u32, 0, Rgba(color.into()));
        }

        for material in materials {
            let i = material.id;

            if let Some(metallic) = material.metalness() {
                this.properties |= Property::METALLIC;
                put_pixel(&mut this.metallic, i, metallic);
            }

            if let Some(roughness) = material.roughness() {
                this.properties |= Property::ROUGHNESS;
                put_pixel(&mut this.roughness, i, roughness);
            }

            if let Some(emission) = material.emission() {
                this.properties |= Property::EMISSION;
                put_pixel(&mut this.emission, i, emission);
            }
        }

        this
    }

    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        fs::create_dir_all(path)?;

        write_png_to(&self.albedo, path.join("albedo.png"))?;

        if self.properties.contains(Property::METALLIC) {
            write_png_to(&self.metallic, path.join("metalness.png"))?;
        }

        if self.properties.contains(Property::ROUGHNESS) {
            write_png_to(&self.roughness, path.join("roughness.png"))?;
        }

        if self.properties.contains(Property::EMISSION) {
            write_png_to(&self.emission, path.join("emission.png"))?;
        }

        Ok(())
    }
}
