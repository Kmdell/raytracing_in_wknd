use crate::{aabb::Aabb, interval::Interval};

use super::{HitRecord, Hittable, HittableObject};

pub struct HittableList {
    pub objects: Vec<HittableObject>,
    bbox: Aabb,
}

impl HittableList {
    pub fn new(hittable_object: HittableObject) -> Self {
        let bbox = *hittable_object.bounding_box();
        Self {
            objects: vec![hittable_object],
            bbox,
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, hittable_object: HittableObject) {
        self.bbox = Aabb::from_aabbs(&self.bbox, &hittable_object.bounding_box());
        self.objects.push(hittable_object);
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self {
            objects: vec![],
            bbox: Aabb::default(),
        }
    }
}

impl Hittable for HittableList {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        ray_t: &Interval,
        hit_record: &mut super::HitRecord,
    ) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            let mut temp_record: HitRecord = HitRecord::default();

            if object.hit(
                ray,
                &Interval::new(ray_t.min, closest_so_far),
                &mut temp_record,
            ) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *hit_record = temp_record;
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
