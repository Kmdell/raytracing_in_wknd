use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

use super::Material;

pub struct Lambertion {
    pub albedo: Color,
}

impl Lambertion {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
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
        *attenuation = self.albedo;
        true
    }
}
