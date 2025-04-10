use std::sync::Arc;

use crate::{
    interval::Interval,
    material::MaterialType,
    ray::Ray,
    vec3::{Point3, Vec3},
};
pub mod hittable_list;
pub mod sphere;
pub use sphere::Sphere;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<MaterialType>,
    pub t: f32,
    pub front_face: bool,
}

pub enum HittableObject {
    Sphere(Sphere),
}

impl HittableObject {
    pub fn hit(&self, ray: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
        match self {
            HittableObject::Sphere(sphere) => sphere.hit(ray, ray_t, hit_record),
        }
    }

    pub fn stationary_sphere(center: Point3, radius: f32, mat: MaterialType) -> HittableObject {
        let sphere = Sphere::new_stationary(center, radius, mat);
        HittableObject::Sphere(sphere)
    }

    pub fn moving_sphere(
        center1: Point3,
        center2: Point3,
        radius: f32,
        mat: MaterialType,
    ) -> HittableObject {
        HittableObject::Sphere(Sphere::new_moving(center1, center2, radius, mat))
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector
        // NOTE: the parameter `outward_normal` is assumed to have unit length
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

impl Default for HitRecord {
    fn default() -> HitRecord {
        HitRecord {
            p: Vec3::default(),
            normal: Vec3::default(),
            mat: Arc::new(MaterialType::None),
            t: 0.0,
            front_face: false,
        }
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
        false
    }
}
