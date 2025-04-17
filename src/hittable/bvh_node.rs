use std::{cmp::Ordering, sync::Arc};

use crate::{aabb::Aabb, interval::Interval};

use super::{hittable_list::HittableList, Hittable, HittableObject};

#[derive(Clone)]
pub struct BvhNode {
    left: Arc<HittableObject>,
    right: Arc<HittableObject>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(objects: &mut [HittableObject]) -> Self {
        let mut bbox = Aabb::EMPTY;
        for object in objects.iter() {
            bbox = Aabb::from_aabbs(&bbox, object.bounding_box());
        }

        let axis = bbox.longest_axis();

        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };

        let object_span = objects.len();

        let (left, right) = match object_span {
            1 => {
                let object = Arc::new(objects[0].clone());
                (object.clone(), object)
            }
            2 => (Arc::new(objects[0].clone()), Arc::new(objects[1].clone())),
            _ => {
                objects[..].sort_by(comparator);
                let mid = object_span / 2;
                let left = Arc::new(HittableObject::bvh_node(&mut objects[..mid]));
                let right = Arc::new(HittableObject::bvh_node(&mut objects[mid..]));
                (left, right)
            }
        };

        Self { left, right, bbox }
    }

    fn box_compare(a: &HittableObject, b: &HittableObject, axis_index: usize) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis_index);
        let b_axis_interval = b.bounding_box().axis_interval(axis_index);

        a_axis_interval.min.total_cmp(&b_axis_interval.min)
    }

    fn box_x_compare(a: &HittableObject, b: &HittableObject) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &HittableObject, b: &HittableObject) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &HittableObject, b: &HittableObject) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl From<HittableList> for BvhNode {
    fn from(mut value: HittableList) -> Self {
        Self::new(&mut value.objects)
    }
}

impl Hittable for BvhNode {
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }

    fn hit(
        &self,
        ray: &crate::ray::Ray,
        ray_t: &crate::interval::Interval,
        hit_record: &mut super::HitRecord,
    ) -> bool {
        // TODO: This shit is broken
        if !self.bbox.hit(ray, *ray_t) {
            return false;
        }

        let hit_left = self.left.hit(ray, ray_t, hit_record);
        let hit_right = self.right.hit(
            ray,
            &Interval::new(ray_t.min, if hit_left { hit_record.t } else { ray_t.max }),
            hit_record,
        );

        hit_left || hit_right
    }
}
