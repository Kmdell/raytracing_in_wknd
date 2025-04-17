use std::{f32::consts::PI, sync::Arc};

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::MaterialType,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Clone)]
pub struct Sphere {
    center: Ray,
    radius: f32,
    mat: Arc<MaterialType>,
    bbox: Aabb,
}

impl Sphere {
    pub fn new_stationary(center: Point3, radius: f32, mat: MaterialType) -> Self {
        let radius = radius.max(0.0);
        let rvec = Vec3::new(radius, radius, radius);
        let temp = Self {
            center: Ray::new(&center, &Vec3::default(), 0.0),
            radius,
            mat: Arc::new(mat),
            bbox: Aabb::from_points(&(center - rvec), &(center + rvec)),
        };

        temp
    }

    pub fn new_moving(center1: Point3, center2: Point3, radius: f32, mat: MaterialType) -> Self {
        let radius = radius.max(0.0);
        let center = Ray::new(&center1, &(center2 - center1), 0.0);

        let rvec = Vec3::new(radius, radius, radius);
        let box1 = Aabb::from_points(&(center.at(0.0) - rvec), &(center.at(0.0) + rvec));
        let box2 = Aabb::from_points(&(center.at(1.0) - rvec), &(center.at(1.0) + rvec));

        Self {
            center,
            radius,
            mat: Arc::new(mat),
            bbox: Aabb::from_aabbs(&box1, &box2),
        }
    }

    pub fn get_sphere_uv(&self, p: &Point3, u: &mut f32, v: &mut f32) {
        // p: a point on the sphere of radius one, centered at the origin
        // u: returned value [0, 1] of angle around the Y axis from X=-1
        // v: returned value [0, 1] of andle from Y=-1 from X=-1
        //      <1 0 0> yields <0.50 0.50>      <-1 0 0> yields <0.00 0.50>
        //      <0 1 0> yields <0.50 1.00>      <0 -1 0> yields <0.50 0.00>
        //      <0 0 1> yields <0.25 0.50>      <0 0 -1> yields <0.75 0.50>

        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
        let current_center = self.center.at(ray.time());
        let oc: Vec3 = current_center - ray.origin();
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
        let outward_normal = (hit_record.p - current_center) / self.radius;
        hit_record.set_face_normal(ray, &outward_normal);
        self.get_sphere_uv(&outward_normal, &mut hit_record.u, &mut hit_record.v);
        hit_record.mat = self.mat.clone();

        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
