use std::fs;

use image::{RgbImage, load_from_memory, GenericImageView};

use crate::MoeglError;

pub struct Texture {
    rgba: RgbImage,
    dimensions: (u32, u32),
}

impl Texture {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, image::ImageError> {
        let diffuse_image = load_from_memory(bytes)?;
        let diffuse_rgba = diffuse_image.to_rgb8();
    
        Ok(Texture {
            rgba: diffuse_rgba,
            dimensions: diffuse_image.dimensions(),
        })
    }

    /// Creates a texture from string slice at runtime.
    pub fn from_path(path: &str) -> Result<Self, MoeglError> {
        let bytes = fs::read(path)?;

        Texture::from_bytes(&bytes)
            .map_err(Into::into)
    }
}
