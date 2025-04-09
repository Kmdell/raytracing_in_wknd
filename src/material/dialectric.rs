use crate::{color::Color, hittable::HitRecord, ray::Ray, rtweekend::random_float};

use super::Material;

pub struct Dialectric {
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index of the enclosing media
    refractive_index: f32,
}

impl Dialectric {
    pub fn new(refractive_index: f32) -> Self {
        Self { refractive_index }
    }

    fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
        // Use Schlick's approximation for reflectance
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dialectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if record.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = ray_in.direction().unit_vector();
        let cos_theta = (-unit_direction).dot(&record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > random_float() {
            unit_direction.reflect(&record.normal)
        } else {
            unit_direction.refract(&record.normal, ri)
        };

        *scattered = Ray::new(&record.p, &direction);

        true
    }
}
