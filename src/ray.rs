use std::f32::INFINITY;

use crate::{
    color::Color,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    vec3::{Point3, Vec3},
};

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3, time: f32) -> Ray {
        Ray {
            orig: *origin,
            dir: *direction,
            time,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.orig + t * self.dir
    }

    pub fn color(&self, depth: u32, world: &impl Hittable) -> Color {
        // If we hit the max ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::default();
        }

        let mut rec = HitRecord::default();
        if world.hit(self, &Interval::new(0.001, INFINITY), &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();
            if rec
                .mat
                .scatter(self, &rec, &mut attenuation, &mut scattered)
            {
                return attenuation * scattered.color(depth - 1, world);
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = self.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            orig: Point3::default(),
            dir: Vec3::default(),
            time: 0.0,
        }
    }
}
