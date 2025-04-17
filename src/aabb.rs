use crate::{interval::Interval, ray::Ray, vec3::Point3};

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Default for Aabb {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Aabb {
    pub const EMPTY: Aabb = Self {
        x: Interval::EMPTY,
        y: Interval::EMPTY,
        z: Interval::EMPTY,
    };

    pub const UNIVERSE: Aabb = Self {
        x: Interval::UNIVERSE,
        y: Interval::UNIVERSE,
        z: Interval::UNIVERSE,
    };

    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut tmp = Self { x, y, z };
        tmp.pad_to_minimums();

        tmp
    }

    pub fn from_points(a: &Point3, b: &Point3) -> Self {
        // Treat the two points a and b as extrema for the bounding box, so we don't require a
        // particular minimum/maximum coordinate order.

        let mut tmp = Self::new(
            Interval::new(a[0].min(b[0]), a[0].max(b[0])),
            Interval::new(a[1].min(b[1]), a[1].max(b[1])),
            Interval::new(a[2].min(b[2]), a[2].max(b[2])),
        );
        tmp.pad_to_minimums();

        tmp
    }

    pub fn from_aabbs(box0: &Aabb, box1: &Aabb) -> Aabb {
        let x = Interval::from_intervals(&box0.x, &box1.x);
        let y = Interval::from_intervals(&box0.y, &box1.y);
        let z = Interval::from_intervals(&box0.z, &box1.z);
        Self { x, y, z }
    }

    pub fn axis_interval(&self, n: usize) -> &Interval {
        match n {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }

    pub fn hit(&self, ray: &Ray, mut ray_t: Interval) -> bool {
        //println!("Ray Interval: {:?}, Ray: {:?}", ray_t, ray);
        let ray_origin = ray.origin();
        let ray_dir = ray.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis];

            let t0 = (ax.min - ray_origin[axis]) * adinv;
            let t1 = (ax.max - ray_origin[axis]) * adinv;

            if t0 < t1 {
                ray_t.min = ray_t.min.max(t0);
                ray_t.max = ray_t.max.min(t1);
            } else {
                ray_t.min = ray_t.min.max(t1);
                ray_t.max = ray_t.max.min(t0);
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        true
    }

    pub fn longest_axis(&self) -> usize {
        // Returns the index of the longest axis of the bounding box
        let max = self.x.size().max(self.y.size()).max(self.z.size());
        match max {
            val if self.y.size() == val => 1,
            val if self.z.size() == val => 2,
            _ => 0,
        }
    }

    fn pad_to_minimums(&mut self) {
        // Adjust the AABB so that no side is narrower than some delta, padding if necessary
        let delta = 0.0001;
        if self.x.size() < delta {
            self.x = self.x.expands(delta)
        }
        if self.y.size() < delta {
            self.y = self.y.expands(delta)
        }
        if self.z.size() < delta {
            self.z = self.z.expands(delta)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn default_aabb() {
        let aabb = Aabb::default();

        assert!(aabb.x.min == Interval::EMPTY.min);
        assert!(aabb.x.max == Interval::EMPTY.max);
        assert!(aabb.y.min == Interval::EMPTY.min);
        assert!(aabb.y.max == Interval::EMPTY.max);
        assert!(aabb.z.min == Interval::EMPTY.min);
        assert!(aabb.z.max == Interval::EMPTY.max);
    }

    #[test]
    fn new_aabb() {
        let aabb = Aabb::new(
            Interval::new(-3.0, 5.0),
            Interval::new(-1.0, 50.0),
            Interval::new(-10.0, 7.0),
        );

        assert!(aabb.x.min == -3.0);
        assert!(aabb.x.max == 5.0);
        assert!(aabb.y.min == -1.0);
        assert!(aabb.y.max == 50.0);
        assert!(aabb.z.min == -10.0);
        assert!(aabb.z.max == 7.0);
    }
}
