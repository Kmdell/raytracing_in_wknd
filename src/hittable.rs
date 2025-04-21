use std::sync::Arc;

use crate::{
    aabb::Aabb,
    interval::Interval,
    material::MaterialType,
    ray::Ray,
    vec3::{Point3, Vec3},
};
pub mod bvh_node;
pub mod hittable_list;
pub mod quad;
pub mod sphere;

use bvh_node::BvhNode;
use quad::Quad;
use sphere::Sphere;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<MaterialType>,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub front_face: bool,
}

#[derive(Clone)]
pub enum HittableObject {
    Sphere(Sphere),
    BvhNode(BvhNode),
    Quad(Quad),
}

impl HittableObject {
    pub fn hit(&self, ray: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
        match self {
            HittableObject::Sphere(sphere) => sphere.hit(ray, ray_t, hit_record),
            HittableObject::BvhNode(bvh_node) => bvh_node.hit(ray, ray_t, hit_record),
            HittableObject::Quad(quad) => quad.hit(ray, ray_t, hit_record),
        }
    }

    pub fn bounding_box(&self) -> &Aabb {
        match self {
            HittableObject::Sphere(sphere) => sphere.bounding_box(),
            HittableObject::BvhNode(bvh_node) => bvh_node.bounding_box(),
            HittableObject::Quad(quad) => quad.bounding_box(),
        }
    }

    pub fn stationary_sphere(center: Point3, radius: f32, mat: MaterialType) -> HittableObject {
        HittableObject::Sphere(Sphere::new_stationary(center, radius, mat))
    }

    pub fn moving_sphere(
        center1: Point3,
        center2: Point3,
        radius: f32,
        mat: MaterialType,
    ) -> HittableObject {
        HittableObject::Sphere(Sphere::new_moving(center1, center2, radius, mat))
    }

    pub fn bvh_node(objects: &mut [HittableObject]) -> HittableObject {
        HittableObject::BvhNode(BvhNode::new(objects))
    }

    pub fn quad(q: Point3, u: Vec3, v: Vec3, mat: MaterialType) -> HittableObject {
        HittableObject::Quad(Quad::new(q, u, v, mat))
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
            u: 0.0,
            v: 0.0,
            front_face: false,
        }
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
        false
    }

    fn bounding_box(&self) -> &Aabb;
}
