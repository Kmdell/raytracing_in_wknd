use image::{DynamicImage, GenericImageView, Pixel, Rgb};

use crate::{color::Color, interval::Interval, simd_vec3::Point3};

use super::Texture;

#[derive(Clone)]
pub struct Image {
    img: DynamicImage,
}

impl Image {
    pub fn new(img: DynamicImage) -> Image {
        Self { img }
    }
}

impl Texture for Image {
    fn value(&self, mut u: f32, mut v: f32, _p: &Point3) -> Color {
        // If e have no texture data, then return solid cyan as a debugging aid
        let (width, height) = self.img.dimensions();
        if height <= 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        // Clamp input texture coordinates to [0,1] x [0,1]
        u = Interval::new(0.0, 1.0).clamp(u);
        v = 1.0 - Interval::new(0.0, 1.0).clamp(v); // Flip V to image coordinates

        let mut i = (u * width as f32) as u32;
        if i >= width {
            i = width - 1;
        }

        let j = (v * height as f32) as u32;
        if j >= height {
            i = height - 1;
        }

        let Rgb([r, g, b]) = self.img.get_pixel(i, j).to_rgb();
        let color_scale = 1.0 / 255.0;

        Color::new(
            r as f32 * color_scale,
            g as f32 * color_scale,
            b as f32 * color_scale,
        )
    }
}
