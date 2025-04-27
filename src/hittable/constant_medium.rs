use std::{f32::INFINITY, sync::Arc};

use crate::{
    color::Color, interval::Interval, material::MaterialType, ray::Ray, texture::TextureType,
    utility::random_float, vec3::Vec3,
};

use super::{HitRecord, Hittable, HittableObject};

#[derive(Clone)]
pub struct ConstantMedium {
    boundary: Arc<HittableObject>,
    neg_inv_density: f32,
    phase_function: Arc<MaterialType>,
}

impl ConstantMedium {
    pub fn new_texture(
        boundary: HittableObject,
        neg_inv_density: f32,
        texture: TextureType,
    ) -> Self {
        let neg_inv_density = -1.0 / neg_inv_density;
        ConstantMedium {
            boundary: Arc::new(boundary),
            neg_inv_density,
            phase_function: Arc::new(MaterialType::isotropic_tex(texture)),
        }
    }

    pub fn new_color(boundary: HittableObject, neg_inv_density: f32, albedo: &Color) -> Self {
        let neg_inv_density = -1.0 / neg_inv_density;
        Self {
            boundary: Arc::new(boundary),
            neg_inv_density,
            phase_function: Arc::new(MaterialType::isotropic_color(&albedo)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        if !self.boundary.hit(ray, &Interval::UNIVERSE, &mut rec1) {
            return false;
        }

        if !self
            .boundary
            .hit(ray, &Interval::new(rec1.t + 0.001, INFINITY), &mut rec2)
        {
            return false;
        }

        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }

        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }

        if rec1.t >= rec2.t {
            return false;
        }

        let ray_length = ray.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_float().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        hit_record.t = rec1.t + hit_distance / ray_length;
        hit_record.p = ray.at(hit_record.t);

        hit_record.normal = Vec3::new(1.0, 0.0, 0.0); // arbitrary
        hit_record.front_face = true; // also arbitrary
        hit_record.mat = self.phase_function.clone();

        true
    }

    fn bounding_box(&self) -> &crate::aabb::Aabb {
        self.boundary.bounding_box()
    }
}
