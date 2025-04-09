use std::sync::Arc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::MaterialType,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Sphere {
    center: Point3,
    radius: f32,
    mat: Arc<MaterialType>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, mat: MaterialType) -> Self {
        let radius = radius.max(0.0);
        // TODO: Initialize material pointer `mat`
        Self {
            center,
            radius,
            mat: Arc::new(mat),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
        let oc: Vec3 = self.center - ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.p = ray.at(hit_record.t);
        let outward_normal = (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(ray, &outward_normal);
        hit_record.mat = self.mat.clone();

        true
    }
}
