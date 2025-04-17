use crate::{color::Color, hittable::HitRecord, ray::Ray, texture::TextureType, vec3::Vec3};

use super::Material;

#[derive(Clone)]
pub struct Lambertion {
    pub tex: TextureType,
}

impl Lambertion {
    pub fn new(tex: TextureType) -> Self {
        Self { tex }
    }
}

impl From<Color> for Lambertion {
    fn from(value: Color) -> Self {
        Self::new(TextureType::solid_color(&value))
    }
}

impl From<&Color> for Lambertion {
    fn from(value: &Color) -> Self {
        Self::new(TextureType::solid_color(value))
    }
}

impl Material for Lambertion {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = record.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        *scattered = Ray::new(&record.p, &scatter_direction, ray_in.time());
        *attenuation = self.tex.value(record.u, record.v, &record.p);
        true
    }
}
