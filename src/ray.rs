use std::f32::INFINITY;

use crate::{
    color::Color,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    vec3::{Point3, Vec3},
};

#[derive(Debug, Clone)]
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
