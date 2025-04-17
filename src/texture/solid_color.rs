use crate::color::Color;

use super::Texture;

#[derive(Clone)]
pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        let color = Color::new(r, g, b);
        color.into()
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f32, v: f32, p: &crate::simd_vec3::Point3) -> Color {
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
