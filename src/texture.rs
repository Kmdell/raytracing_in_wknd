use crate::{color::Color, vec3::Point3};

mod checker;
mod image;
mod noise;
mod solid_color;

use ::image::DynamicImage;
use checker::Checker;
use image::Image;
use noise::Noise;
pub use solid_color::SolidColor;

pub trait Texture {
    fn value(&self, _u: f32, _v: f32, _p: &Point3) -> Color {
        Color::default()
    }
}

#[derive(Clone)]
pub enum TextureType {
    SolidColor(SolidColor),
    Checker(Checker),
    Image(Image),
    Noise(Noise),
}

impl From<Color> for TextureType {
    fn from(value: Color) -> Self {
        Self::solid_color(&value)
    }
}

impl From<&Color> for TextureType {
    fn from(value: &Color) -> Self {
        Self::solid_color(value)
    }
}

impl TextureType {
    pub fn solid_color(albedo: &Color) -> TextureType {
        TextureType::SolidColor(SolidColor::from(albedo))
    }

    pub fn checker(scale: f32, even: &Color, odd: &Color) -> TextureType {
        TextureType::Checker(Checker::from_colors(scale, even, odd))
    }

    pub fn image(image: DynamicImage) -> TextureType {
        TextureType::Image(Image::new(image))
    }

    pub fn noise(scale: f32) -> TextureType {
        TextureType::Noise(Noise::new(scale))
    }

    pub fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        match self {
            TextureType::Checker(checker) => checker.value(u, v, p),
            TextureType::SolidColor(solid) => solid.value(u, v, p),
            TextureType::Image(image) => image.value(u, v, p),
            TextureType::Noise(noise) => noise.value(u, v, p),
        }
    }
}
