use crate::{color::Color, texture::TextureType, vec3::Point3};

use super::Material;

#[derive(Clone)]
pub struct DiffuseLight {
    tex: TextureType,
}

impl DiffuseLight {
    pub fn new(tex: TextureType) -> Self {
        Self { tex }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f32, v: f32, p: &Point3) -> Color {
        self.tex.value(u, v, p)
    }
}

impl From<Color> for DiffuseLight {
    fn from(value: Color) -> Self {
        Self {
            tex: TextureType::solid_color(&value),
        }
    }
}

impl From<&Color> for DiffuseLight {
    fn from(value: &Color) -> Self {
        Self {
            tex: TextureType::solid_color(value),
        }
    }
}
