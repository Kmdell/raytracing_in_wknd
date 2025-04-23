use std::sync::Arc;

use crate::{aabb::Aabb, interval::Interval, ray::Ray, vec3::Vec3};

use super::{HitRecord, Hittable, HittableObject};

#[derive(Clone)]
pub struct Translate {
    object: Arc<HittableObject>,
    offset: Vec3,
    bbox: Aabb,
}

impl Translate {
    pub fn new(object: HittableObject, offset: Vec3) -> Self {
        let bbox = *object.bounding_box() + offset;
        Self {
            object: Arc::new(object),
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
        // Move the ray backwards by the offset
        let offset_r = Ray::new(&(*ray.origin() - self.offset), ray.direction(), ray.time());

        // Determine whether an intersection exists along the offset ray (and if so, where)
        if !self.object.hit(&offset_r, ray_t, hit_record) {
            return false;
        }

        // Move the intersection point forwards by the offset
        hit_record.p += self.offset;

        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
