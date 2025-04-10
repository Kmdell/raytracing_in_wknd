use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

use super::Material;

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = ray_in.direction().reflect(&record.normal);
        let reflected = reflected.unit_vector() + (self.fuzz * Vec3::random_unit_vector());
        *scattered = Ray::new(&record.p, &reflected, ray_in.time());
        *attenuation = self.albedo;
        (scattered.direction().dot(&record.normal)) > 0.0
    }
}
