use std::{f32::INFINITY, sync::Arc};

use crate::{
    aabb::Aabb,
    interval::Interval,
    ray::Ray,
    utility::degree_to_radians,
    vec3::{Point3, Vec3},
};

use super::{HitRecord, Hittable, HittableObject};

#[derive(Clone)]
pub struct RotateY {
    object: Arc<HittableObject>,
    sin_theta: f32,
    cos_theta: f32,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(object: HittableObject, angle: f32) -> Self {
        let radians = degree_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = *object.bounding_box();

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f32 * bbox.x.max + (1 - i) as f32 * bbox.x.min;
                    let y = j as f32 * bbox.y.max + (1 - j) as f32 * bbox.y.min;
                    let z = k as f32 * bbox.z.max + (1 - k) as f32 * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        let bbox = Aabb::from_points(&min, &max);

        Self {
            object: Arc::new(object),
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
        // Transform the ray from world to object space.
        let origin = Point3::new(
            self.cos_theta * ray.origin().x() - self.sin_theta * ray.origin().z(),
            ray.origin().y(),
            self.sin_theta * ray.origin().x() + self.cos_theta * ray.origin().z(),
        );

        let direction = Point3::new(
            self.cos_theta * ray.direction().x() - self.sin_theta * ray.direction().z(),
            ray.direction().y(),
            self.sin_theta * ray.direction().x() + self.cos_theta * ray.direction().z(),
        );

        let rotated_r = Ray::new(&origin, &direction, ray.time());

        // Determine whether an intersection exists in object space (and if so, where).

        if !self.object.hit(&rotated_r, ray_t, hit_record) {
            return false;
        }

        // Transform the intersection from object space to world space.

        hit_record.p = Point3::new(
            self.cos_theta * hit_record.p.x() + self.sin_theta * hit_record.p.z(),
            hit_record.p.y(),
            -self.sin_theta * hit_record.p.x() + self.cos_theta * hit_record.p.z(),
        );

        hit_record.normal = Vec3::new(
            self.cos_theta * hit_record.normal.x() + self.sin_theta * hit_record.normal.z(),
            hit_record.normal.y(),
            -self.sin_theta * hit_record.normal.x() + self.cos_theta * hit_record.normal.z(),
        );

        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
