use crate::{color::Color, vec3::Point3};

use super::Texture;

#[derive(Clone)]
pub struct SolidColor {
    albedo: Color,
}

impl Texture for SolidColor {
    fn value(&self, _u: f32, _v: f32, _p: &Point3) -> Color {
        return self.albedo;
    }
}

impl From<&Color> for SolidColor {
    fn from(value: &Color) -> Self {
        Self { albedo: *value }
    }
}

impl From<Color> for SolidColor {
    fn from(value: Color) -> Self {
        Self { albedo: value }
    }
}
