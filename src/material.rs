use dialectric::Dialectric;
use lambertian::Lambertion;
use metal::Metal;

use crate::{color::Color, hittable::HitRecord, ray::Ray};
pub mod dialectric;
pub mod lambertian;
pub mod metal;

pub enum MaterialType {
    None,
    Lambertian(Lambertion),
    Metal(Metal),
    Dialectric(Dialectric),
}

impl MaterialType {
    pub fn metal(albedo: Color, fuzz: f32) -> MaterialType {
        MaterialType::Metal(Metal::new(albedo, fuzz))
    }

    pub fn lambertion(albedo: Color) -> MaterialType {
        MaterialType::Lambertian(Lambertion::new(albedo))
    }

    pub fn dialectric(refractive_index: f32) -> MaterialType {
        MaterialType::Dialectric(Dialectric::new(refractive_index))
    }

    pub fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            MaterialType::Lambertian(mat) => mat.scatter(ray_in, record, attenuation, scattered),
            MaterialType::Metal(mat) => mat.scatter(ray_in, record, attenuation, scattered),
            MaterialType::Dialectric(mat) => mat.scatter(ray_in, record, attenuation, scattered),
            MaterialType::None => false,
        }
    }
}

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        false
    }
}
