use std::sync::Arc;

use crate::{
    aabb::Aabb,
    interval::Interval,
    material::MaterialType,
    simd_vec3::{Point3, Vec3},
};

use super::{HitRecord, Hittable};

const UNIT_INTERVAL: Interval = Interval { min: 0.0, max: 1.0 };

#[derive(Clone)]
pub struct Quad {
    Q: Point3,
    u: Vec3,
    v: Vec3,
    mat: Arc<MaterialType>,
    bbox: Aabb,
    normal: Vec3,
    D: f32,
    w: Vec3,
}

impl Quad {
    pub fn new(Q: Point3, u: Vec3, v: Vec3, mat: MaterialType) -> Quad {
        let n = u.cross(&v);
        let normal = n.unit_vector();
        let D = normal.dot(&Q);
        let w = n / n.dot(&n);

        let mut tmp = Self {
            Q,
            u,
            v,
            mat: Arc::new(mat),
            bbox: Aabb::default(),
            normal,
            D,
            w,
        };

        tmp.set_bounding_box();

        tmp
    }

    fn set_bounding_box(&mut self) {
        // Compute the bounding box of all four vertices
        let bbox_diagonal1 = Aabb::from_points(&self.Q, &(self.Q + self.u + self.v));
        let bbox_diagonal2 = Aabb::from_points(&(self.Q + self.u), &(self.Q + self.v));
        self.bbox = Aabb::from_aabbs(&bbox_diagonal1, &bbox_diagonal2);
    }

    fn is_interior(&self, a: f32, b: f32, rec: &mut HitRecord) -> bool {
        // Given the hit point in plane coordinates, return false if it is outside the primitive,
        // otherwise set the hit record UV coordinates nd return true

        if !UNIT_INTERVAL.contains(a) || !UNIT_INTERVAL.contains(b) {
            return false;
        }

        rec.u = a;
        rec.v = b;

        true
    }
}

impl Hittable for Quad {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        ray_t: &crate::interval::Interval,
        hit_record: &mut super::HitRecord,
    ) -> bool {
        let denom = self.normal.dot(ray.direction());

        // No hit if the ray is parallel to the plane
        if denom.abs() < 1e-8 {
            return false;
        }

        // Returns false if the hit point parameter t is outside the ray interval
        let t = (self.D - self.normal.dot(ray.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        // Determine f the hit point lies within the planar sphere using its plane coordinates.
        let intersection = ray.at(t);
        let planar_hitpt_vector = intersection - self.Q;
        let alpha = self.w.dot(&planar_hitpt_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpt_vector));

        if !self.is_interior(alpha, beta, hit_record) {
            return false;
        }

        // Ray hits 2D shape; set the rest of the hit record and return true
        hit_record.t = t;
        hit_record.p = intersection;
        hit_record.mat = self.mat.clone();
        hit_record.set_face_normal(ray, &self.normal);

        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
