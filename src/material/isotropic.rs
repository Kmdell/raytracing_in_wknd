use crate::{color::Color, hittable::HitRecord, ray::Ray, texture::TextureType, vec3::Vec3};

use super::Material;

#[derive(Clone)]
pub struct Isotropic {
    texture: TextureType,
}

impl Isotropic {
    pub fn new_color(albedo: &Color) -> Self {
        Isotropic {
            texture: TextureType::solid_color(&albedo),
        }
    }

    pub fn new_texture(tex: TextureType) -> Self {
        Isotropic { texture: tex }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new(&record.p, &Vec3::random_unit_vector(), ray_in.time());
        *attenuation = self.texture.value(record.u, record.v, &record.p);

        true
    }
}
