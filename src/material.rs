use dialectric::Dialectric;
use diffuse_light::DiffuseLight;
use isotropic::Isotropic;
use lambertian::Lambertion;
use metal::Metal;

use crate::{color::Color, hittable::HitRecord, ray::Ray, texture::TextureType, vec3::Point3};
pub mod dialectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

#[derive(Clone)]
pub enum MaterialType {
    None,
    Lambertian(Lambertion),
    Metal(Metal),
    Dialectric(Dialectric),
    DiffuseLight(DiffuseLight),
    Isotropic(Isotropic),
}

impl MaterialType {
    pub fn metal(albedo: Color, fuzz: f32) -> MaterialType {
        MaterialType::Metal(Metal::new(albedo, fuzz))
    }

    pub fn lambertion(tex: TextureType) -> MaterialType {
        MaterialType::Lambertian(Lambertion::new(tex))
    }

    pub fn dialectric(refractive_index: f32) -> MaterialType {
        MaterialType::Dialectric(Dialectric::new(refractive_index))
    }

    pub fn diffuse_light(tex: TextureType) -> MaterialType {
        MaterialType::DiffuseLight(DiffuseLight::new(tex))
    }

    pub fn isotropic_tex(tex: TextureType) -> MaterialType {
        MaterialType::Isotropic(Isotropic::new_texture(tex))
    }

    pub fn isotropic_color(albedo: &Color) -> MaterialType {
        MaterialType::Isotropic(Isotropic::new_color(&albedo))
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
            MaterialType::DiffuseLight(mat) => mat.scatter(ray_in, record, attenuation, scattered),
            MaterialType::Isotropic(mat) => mat.scatter(ray_in, record, attenuation, scattered),
            MaterialType::None => false,
        }
    }

    pub fn emitted(&self, u: f32, v: f32, p: &Point3) -> Color {
        match self {
            MaterialType::Lambertian(mat) => mat.emitted(u, v, p),
            MaterialType::Metal(mat) => mat.emitted(u, v, p),
            MaterialType::Dialectric(mat) => mat.emitted(u, v, p),
            MaterialType::DiffuseLight(mat) => mat.emitted(u, v, p),
            MaterialType::Isotropic(mat) => mat.emitted(u, v, p),
            MaterialType::None => Color::default(),
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

    fn emitted(&self, u: f32, v: f32, p: &Point3) -> Color {
        Color::default()
    }
}
